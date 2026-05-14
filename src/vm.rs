use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

use crate::gc::Gc;
use crate::instruction::Instruction;
use crate::value::{ClassMethodDef, SharedInstanceData, Value};

struct TryInfo {
    stack_depth: usize,
    call_stack_depth: usize,
    catch_ip: isize,
    #[allow(dead_code)]
    finally_ip: isize,
    try_start: isize,
    try_end: isize,
    saved_vars: HashMap<String, Value>,
}

pub struct VM {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
    globals: HashMap<String, Value>,
    code: Vec<Instruction>,
    ip: isize,
    func_table: Vec<(String, Vec<String>, Vec<Instruction>, Vec<String>)>,
    #[allow(dead_code)]
    class_table: Vec<(
        String,
        Vec<(String, Vec<String>, Vec<Instruction>, Vec<String>)>,
    )>,
    call_stack: Vec<Frame>,
    try_stack: Vec<TryInfo>,
    #[allow(dead_code)]
    error_flag: bool,
    #[allow(dead_code)]
    error_value: Option<Value>,
    halted: bool,
    pending_instance: Option<Value>,
}

struct Frame {
    ip: isize,
    code: Vec<Instruction>,
    vars: HashMap<String, Value>,
    stack_depth: usize,
    try_stack_depth: usize,
}

impl VM {
    pub fn new(
        code: Vec<Instruction>,
        func_table: Vec<(String, Vec<String>, Vec<Instruction>, Vec<String>)>,
        class_table: Vec<(
            String,
            Vec<(String, Vec<String>, Vec<Instruction>, Vec<String>)>,
        )>,
    ) -> Self {
        VM {
            stack: Vec::new(),
            vars: HashMap::new(),
            globals: HashMap::new(),
            code,
            ip: 0,
            func_table,
            class_table,
            call_stack: Vec::new(),
            try_stack: Vec::new(),
            error_flag: false,
            error_value: None,
            halted: false,
            pending_instance: None,
        }
    }

    pub fn get_globals(&self) -> &HashMap<String, Value> {
        &self.globals
    }

    pub fn set_globals(&mut self, globals: HashMap<String, Value>) {
        self.globals = globals;
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap_or(Value::Null)
    }

    fn peek(&self) -> Value {
        self.stack.last().cloned().unwrap_or(Value::Null)
    }

    fn var(&mut self, name: &str) -> Value {
        if let Some(frame) = self.call_stack.last() {
            if let Some(val) = frame.vars.get(name) {
                return val.clone();
            }
        }
        if let Some(val) = self.vars.get(name) {
            return val.clone();
        }
        if let Some(val) = self.globals.get(name) {
            return val.clone();
        }
        if let Some(val) = self.get_self_field(name) {
            return val;
        }
        Value::Null
    }

    fn get_self_field(&self, name: &str) -> Option<Value> {
        let self_val = self
            .call_stack
            .last()
            .and_then(|f| f.vars.get("self"))
            .or_else(|| self.vars.get("self"));
        match self_val {
            Some(Value::SharedInstance(data)) => data.borrow().fields.get(name).cloned(),
            _ => None,
        }
    }

    fn set_var(&mut self, name: &str, val: Value) {
        if let Some(frame) = self.call_stack.last_mut() {
            if frame.vars.contains_key(name) {
                frame.vars.insert(name.to_string(), val);
                return;
            }
            if Self::try_set_self_field(frame, name, val.clone()) {
                return;
            }
        }
        if Self::try_set_self_field_map(&mut self.vars, name, val.clone()) {
            return;
        }
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), val);
        } else if self.globals.contains_key(name) {
            self.globals.insert(name.to_string(), val);
        } else {
            self.vars.insert(name.to_string(), val);
        }
    }

    fn try_set_self_field(frame: &mut Frame, name: &str, val: Value) -> bool {
        if let Some(self_val) = frame.vars.get("self") {
            match self_val {
                Value::SharedInstance(data) if data.borrow().fields.contains_key(name) => {
                    data.borrow_mut().fields.insert(name.to_string(), val);
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    fn try_set_self_field_map(vars: &mut HashMap<String, Value>, name: &str, val: Value) -> bool {
        if let Some(self_val) = vars.get("self") {
            match self_val {
                Value::SharedInstance(data) if data.borrow().fields.contains_key(name) => {
                    data.borrow_mut().fields.insert(name.to_string(), val);
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    fn set_global(&mut self, name: &str, val: Value) {
        self.globals.insert(name.to_string(), val);
    }

    #[allow(dead_code)]
    fn binary_op(&mut self, op: fn(f64, f64) -> f64) {
        let b = self.pop();
        let a = self.pop();
        match (&a, &b) {
            (Value::Number(x), Value::Number(y)) => {
                self.push(Value::Number(op(*x, *y)));
            }
            (Value::String(x), Value::String(y)) if op(0.0, 0.0) == 0.0 => {}
            _ => {
                self.push(Value::Null);
            }
        }
    }

    fn compare(&mut self, cmp: fn(f64, f64) -> bool) {
        let b = self.pop();
        let a = self.pop();
        match (&a, &b) {
            (Value::Number(x), Value::Number(y)) => {
                self.push(Value::Bool(cmp(*x, *y)));
            }
            (Value::String(x), Value::String(y)) => {
                self.push(Value::Bool(cmp(x.len() as f64, y.len() as f64)));
            }
            _ => {
                self.push(Value::Bool(false));
            }
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            if self.halted {
                break;
            }
            if self.ip as usize >= self.code.len() {
                break;
            }

            if let Err(e) = self.process_drops() {
                return Err(e);
            }

            let inst = self.code[self.ip as usize].clone();
            self.ip += 1;
            self.exec_inst(inst)?;

            if self.halted {
                break;
            }
        }
        let _ = self.process_drops();
        Ok(())
    }

    fn exec_inst(&mut self, inst: Instruction) -> Result<(), String> {
        match inst {
            Instruction::Const(v) => self.push(v),
            Instruction::LoadVar(name) => {
                let val = self.var(&name);
                self.push(val);
            }
            Instruction::StoreVar(name) => {
                let val = self.pop();
                if self.call_stack.is_empty() {
                    self.set_global(&name, val);
                } else {
                    self.set_var(&name, val);
                }
            }
            Instruction::LoadLocal(idx) => {
                if let Some(frame) = self.call_stack.last() {
                    if let Some(val) = frame.vars.values().nth(idx) {
                        self.push(val.clone());
                    } else {
                        self.push(Value::Null);
                    }
                }
            }
            Instruction::StoreLocal(idx) => {
                let val = self.pop();
                if let Some(frame) = self.call_stack.last_mut() {
                    let key = frame.vars.keys().nth(idx).cloned();
                    if let Some(k) = key {
                        frame.vars.insert(k, val);
                    }
                }
            }
            Instruction::Add => {
                let b = self.pop();
                let a = self.pop();
                match (&a, &b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Number(x + y)),
                    (Value::String(x), Value::String(y)) => {
                        self.push(Value::String(format!("{}{}", x, y)))
                    }
                    (Value::String(x), _) => {
                        self.push(Value::String(format!("{}{}", x, b.to_string())))
                    }
                    (_, Value::String(y)) => {
                        self.push(Value::String(format!("{}{}", a.to_string(), y)))
                    }
                    _ => self.push(Value::Null),
                }
            }
            Instruction::Sub => {
                let b = self.pop();
                let a = self.pop();
                match (&a, &b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Number(x - y)),
                    _ => self.push(Value::Null),
                }
            }
            Instruction::Mul => {
                let b = self.pop();
                let a = self.pop();
                match (&a, &b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Number(x * y)),
                    _ => self.push(Value::Null),
                }
            }
            Instruction::Div => {
                let b = self.pop();
                let a = self.pop();
                match (&a, &b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Number(x / y)),
                    _ => self.push(Value::Null),
                }
            }
            Instruction::Mod => {
                let b = self.pop();
                let a = self.pop();
                match (&a, &b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Number(x % y)),
                    _ => self.push(Value::Null),
                }
            }
            Instruction::Pow => {
                let b = self.pop();
                let a = self.pop();
                match (&a, &b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Number(x.powf(*y))),
                    _ => self.push(Value::Null),
                }
            }
            Instruction::Neg => {
                let a = self.pop();
                match a {
                    Value::Number(n) => self.push(Value::Number(-n)),
                    _ => self.push(Value::Null),
                }
            }
            Instruction::And => {
                let b = self.pop();
                let a = self.pop();
                self.push(Value::Bool(a.is_truthy() && b.is_truthy()));
            }
            Instruction::Or => {
                let b = self.pop();
                let a = self.pop();
                self.push(Value::Bool(a.is_truthy() || b.is_truthy()));
            }
            Instruction::Not => {
                let a = self.pop();
                self.push(Value::Bool(!a.is_truthy()));
            }
            Instruction::Eq => {
                let b = self.pop();
                let a = self.pop();
                self.push(Value::Bool(a.to_string() == b.to_string()));
            }
            Instruction::Ne => {
                let b = self.pop();
                let a = self.pop();
                self.push(Value::Bool(a.to_string() != b.to_string()));
            }
            Instruction::Gt => self.compare(|x, y| x > y),
            Instruction::Lt => self.compare(|x, y| x < y),
            Instruction::Ge => self.compare(|x, y| x >= y),
            Instruction::Le => self.compare(|x, y| x <= y),
            Instruction::Jump(offset) => {
                self.ip = (self.ip as isize + offset - 1) as isize;
            }
            Instruction::JumpIfFalse(offset) => {
                let val = self.pop();
                if !val.is_truthy() {
                    self.ip = (self.ip as isize + offset - 1) as isize;
                }
            }
            Instruction::JumpIfTrue(offset) => {
                let val = self.pop();
                if val.is_truthy() {
                    self.ip = (self.ip as isize + offset - 1) as isize;
                }
            }
            Instruction::Call(argc) => {
                let stack_len = self.stack.len();
                let callee = self.stack[stack_len - 1].clone();

                match callee {
                    Value::Func {
                        name: _,
                        code,
                        params,
                        captures,
                    } => {
                        let mut new_vars = HashMap::new();
                        for cap in &captures {
                            new_vars.insert(cap.clone(), self.var(cap));
                        }
                        for (i, param) in params.iter().enumerate() {
                            if i < argc {
                                let arg_idx = stack_len - argc - 1 + i;
                                let arg_val = self.stack[arg_idx].clone();
                                new_vars.insert(param.clone(), arg_val);
                            }
                        }
                        for _ in 0..=argc {
                            self.pop();
                        }

                        let frame = Frame {
                            ip: self.ip,
                            code: self.code.clone(),
                            vars: self.vars.clone(),
                            stack_depth: self.stack.len(),
                            try_stack_depth: self.try_stack.len(),
                        };
                        self.call_stack.push(frame);
                        self.code = code;
                        self.ip = 0;
                        self.vars = new_vars;
                    }
                    Value::NativeFunc(f) => {
                        let mut args = Vec::new();
                        for i in 0..argc {
                            let arg_idx = stack_len - argc - 1 + i;
                            args.push(self.stack[arg_idx].clone());
                        }
                        for _ in 0..=argc {
                            self.pop();
                        }
                        match f(&args) {
                            Ok(result) => self.push(result),
                            Err(e) => return Err(e),
                        }
                    }
                    Value::String(method_name) if argc > 0 => {
                        let name = method_name.clone();
                        let obj_idx = stack_len - argc - 1;
                        if obj_idx < stack_len {
                            let obj_val = self.stack[obj_idx].clone();
                            let (class_name, methods) = match &obj_val {
                                Value::SharedInstance(data) => {
                                    let d = data.borrow();
                                    (d.class.clone(), d.methods.clone())
                                }
                                _ => {
                                    return Err(format!("cannot call method on non-instance"));
                                }
                            };
                            if let Some(m) = methods.iter().find(|m| m.name == name) {
                                let mut new_vars = HashMap::new();
                                new_vars.insert("self".into(), obj_val.clone());
                                let num_real_args = argc - 1;
                                for (i, param) in m.params.iter().enumerate() {
                                    if i < num_real_args {
                                        let arg_idx = obj_idx + 1 + i;
                                        let arg_val = self.stack[arg_idx].clone();
                                        new_vars.insert(param.clone(), arg_val);
                                    }
                                }
                                for _ in 0..=argc {
                                    self.pop();
                                }

                                let frame = Frame {
                                    ip: self.ip,
                                    code: self.code.clone(),
                                    vars: self.vars.clone(),
                                    stack_depth: self.stack.len(),
                                    try_stack_depth: self.try_stack.len(),
                                };
                                self.call_stack.push(frame);
                                self.code = m.code.clone();
                                self.ip = 0;
                                self.vars = new_vars;
                            } else {
                                return Err(format!("method {} not found on {}", name, class_name));
                            }
                        } else {
                            return Err(format!("invalid method call"));
                        }
                    }
                    _ => {
                        return Err(format!("cannot call non-function: {:?}", callee));
                    }
                }
            }
            Instruction::Return => {
                let val = self.pop();
                if let Some(frame) = self.call_stack.pop() {
                    let depth = frame.stack_depth;
                    while self.stack.len() > depth {
                        self.pop();
                    }
                    if self.pending_instance.is_some() {
                        let instance = if let Some(modified_self) = self.vars.get("self") {
                            modified_self.clone()
                        } else {
                            self.pending_instance.take().unwrap_or(Value::Null)
                        };
                        self.pending_instance = None;
                        self.push(instance);
                    } else {
                        self.push(val);
                    }
                    while self.try_stack.len() > frame.try_stack_depth {
                        self.try_stack.pop();
                    }
                    self.code = frame.code;
                    self.ip = frame.ip;
                    self.vars = frame.vars;
                } else {
                    self.push(val);
                }
            }
            Instruction::MakeFunc(name, params, idx) => {
                if let Some((_, _, code, captures)) = self.func_table.get(idx) {
                    self.push(Value::Func {
                        name,
                        params,
                        code: code.clone(),
                        captures: captures.clone(),
                    });
                }
            }
            Instruction::MakeClosure(idx, captures) => {
                let entry = self.func_table.get(idx).cloned();
                if let Some((name, params, code, _)) = entry {
                    let captured_vals: Vec<String> = captures.iter().map(|c| c.clone()).collect();
                    self.push(Value::Func {
                        name,
                        params,
                        code,
                        captures: captured_vals,
                    });
                }
            }
            Instruction::MakeClass(name, methods) => {
                let mut class_methods = Vec::new();
                for (mname, params, code, captures) in &methods {
                    class_methods.push(ClassMethodDef {
                        name: mname.clone(),
                        params: params.clone(),
                        code: code.clone(),
                        captures: captures.clone(),
                    });
                }
                self.push(Value::Class {
                    name,
                    fields: vec![],
                    methods: class_methods,
                    publics: vec![],
                    constructor_params: vec![],
                });
            }
            Instruction::Instantiate(class_name) => {
                let mut class_val = self.var(&class_name);
                if let Value::Null = class_val {
                    class_val = self
                        .globals
                        .get(&class_name)
                        .cloned()
                        .unwrap_or(Value::Null);
                }
                match class_val {
                    Value::Class {
                        name,
                        fields,
                        methods,
                        ..
                    } => {
                        let mut instance_fields: HashMap<String, Value> = HashMap::new();
                        for (n, v) in &fields {
                            instance_fields.insert(n.clone(), v.clone());
                        }

                        let has_destroy = methods.iter().any(|m| m.name == "destroy");
                        let shared = Value::SharedInstance(Gc::new(
                            SharedInstanceData {
                                class: name.clone(),
                                fields: instance_fields,
                                methods: methods.clone(),
                                has_destroy,
                            },
                        ));

                        let ctor = methods.iter().find(|m| m.name == "create");
                        if let Some(ctor) = ctor {
                            let argc = ctor.params.len();
                            let mut new_vars = HashMap::new();
                            new_vars.insert("self".into(), shared.clone());
                            for (i, param) in ctor.params.iter().enumerate() {
                                if i < argc {
                                    let arg_idx = self.stack.len() - argc + i;
                                    if arg_idx < self.stack.len() {
                                        let arg_val = self.stack[arg_idx].clone();
                                        new_vars.insert(param.clone(), arg_val);
                                    }
                                }
                            }
                            for _ in 0..argc {
                                self.pop();
                            }

                            self.pending_instance = Some(shared);
                            let frame = Frame {
                                ip: self.ip,
                                code: self.code.clone(),
                                vars: self.vars.clone(),
                                stack_depth: self.stack.len(),
                                try_stack_depth: self.try_stack.len(),
                            };
                            self.call_stack.push(frame);
                            self.code = ctor.code.clone();
                            self.ip = 0;
                            self.vars = new_vars;
                        } else {
                            self.push(shared);
                        }
                    }
                    _ => return Err(format!("{} is not a class", class_name)),
                }
            }
            Instruction::LoadMethod(name) => {
                self.push(Value::String(name));
            }
            Instruction::MakeList(count) => {
                let mut items = Vec::new();
                for _ in 0..count {
                    items.insert(0, self.pop());
                }
                self.push(Value::List(Gc::new(items)));
            }
            Instruction::MakeMap(count) => {
                let mut map = HashMap::new();
                for _ in 0..count {
                    let val = self.pop();
                    let key = match self.pop() {
                        Value::String(s) => s,
                        v => v.to_string(),
                    };
                    map.insert(key, val);
                }
                self.push(Value::Map(Gc::new(map)));
            }
            Instruction::IndexGet => {
                let collection = self.pop();
                let index = self.pop();
                match collection {
                    Value::List(items) => match index {
                        Value::Number(n) => {
                            let items = items.borrow();
                            let i = n as usize;
                            if i < items.len() {
                                self.push(items[i].clone());
                            } else {
                                self.push(Value::Null);
                            }
                        }
                        _ => self.push(Value::Null),
                    },
                    Value::Map(map) => {
                        let key = index.to_string();
                        self.push(map.borrow().get(&key).cloned().unwrap_or(Value::Null));
                    }
                    _ => self.push(Value::Null),
                }
            }
            Instruction::MapToList => {
                let val = self.pop();
                match val {
                    Value::Map(map) => {
                        let keys: Vec<Value> =
                            map.borrow().keys().cloned().map(Value::String).collect();
                        self.push(Value::List(Gc::new(keys)));
                    }
                    other => self.push(other),
                }
            }
            Instruction::ListLen => {
                let val = self.pop();
                match val {
                    Value::List(items) => self.push(Value::Number(items.borrow().len() as f64)),
                    Value::Map(map) => self.push(Value::Number(map.borrow().len() as f64)),
                    Value::String(s) => self.push(Value::Number(s.len() as f64)),
                    _ => self.push(Value::Number(0.0)),
                }
            }
            Instruction::AddToList => {
                let list = self.pop();
                let element = self.pop();
                match list {
                    Value::List(items) => {
                        items.borrow_mut().push(element);
                        self.push(Value::List(items));
                    }
                    _ => return Err("cannot add to non-list".into()),
                }
            }
            Instruction::RemoveFromList => {
                let list = self.pop();
                let element = self.pop();
                match list {
                    Value::List(items) => {
                        let elem_str = element.to_string();
                        items.borrow_mut().retain(|i| i.to_string() != elem_str);
                        self.push(Value::List(items));
                    }
                    _ => return Err("cannot remove from non-list".into()),
                }
            }
            Instruction::Say => {
                let val = self.pop();
                println!("{}", val.to_string());
            }
            Instruction::Ask(var) => {
                let prompt = self.pop();
                print!("{}", prompt.to_string());
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                self.set_var(&var, Value::String(input.trim().to_string()));
            }
            Instruction::ReadFile(var) => {
                let filename = self.pop();
                match fs::read_to_string(filename.to_string()) {
                    Ok(content) => {
                        self.set_var(&var, Value::String(content));
                    }
                    Err(e) => {
                        self.set_var(&var, Value::Null);
                        return Err(format!("cannot read file: {}", e));
                    }
                }
            }
            Instruction::WriteFile => {
                let filename = self.pop();
                let content = self.pop();
                match fs::write(filename.to_string(), content.to_string()) {
                    Ok(_) => {}
                    Err(e) => return Err(format!("cannot write file: {}", e)),
                }
            }
            Instruction::Raise => {
                let err_val = self.pop();
                if let Some(info) = self.try_stack.last() {
                    if info.try_start <= self.ip - 1 && self.ip - 1 < info.try_end {
                        let info = self.try_stack.pop().unwrap();
                        while self.call_stack.len() > info.call_stack_depth {
                            self.call_stack.pop();
                        }
                        self.vars = info.saved_vars;
                        while self.stack.len() > info.stack_depth {
                            self.pop();
                        }
                        self.push(err_val);
                        self.ip = info.catch_ip;
                        return Ok(());
                    }
                }
                return Err(format!("error: {}", err_val.to_string()));
            }
            Instruction::TryCatch(catch_offset, finally_offset) => {
                let try_start = self.ip - 1;
                let try_end = try_start + catch_offset;
                let info = TryInfo {
                    stack_depth: self.stack.len(),
                    call_stack_depth: self.call_stack.len(),
                    catch_ip: self.ip + catch_offset - 1,
                    finally_ip: self.ip + catch_offset + finally_offset - 1,
                    try_start,
                    try_end,
                    saved_vars: self.vars.clone(),
                };
                self.try_stack.push(info);
            }
            Instruction::EndTry => {}
            Instruction::TypeOf => {
                let val = self.pop();
                self.push(Value::String(val.type_name().to_string()));
            }
            Instruction::Convert(target_type) => {
                let val = self.pop();
                match target_type.as_str() {
                    "number" => match val {
                        Value::Number(n) => self.push(Value::Number(n)),
                        Value::String(s) => {
                            if let Ok(n) = s.parse::<f64>() {
                                self.push(Value::Number(n));
                            } else {
                                self.push(Value::Null);
                            }
                        }
                        _ => self.push(Value::Null),
                    },
                    "string" => self.push(Value::String(val.to_string())),
                    "bool" => self.push(Value::Bool(val.is_truthy())),
                    _ => self.push(Value::Null),
                }
            }
            Instruction::Dup => {
                let val = self.peek();
                self.push(val);
            }
            Instruction::Pop => {
                self.pop();
            }
            Instruction::Stop => {
                self.halted = true;
            }
            Instruction::Exit => {
                self.pop();
                self.halted = true;
            }
            Instruction::Halt => {
                self.halted = true;
            }
        }
        Ok(())
    }

    fn process_drops(&mut self) -> Result<(), String> {
        let queue = crate::value::drain_destroy_queue();
        for pd in queue {
            let instance_val =
                Value::SharedInstance(Gc::new(SharedInstanceData {
                    class: pd.class,
                    fields: pd.fields,
                    methods: pd.methods.clone(),
                    has_destroy: false,
                }));
            if let Some(destroy) = pd.methods.iter().find(|m| m.name == "destroy") {
                let mut new_vars = HashMap::new();
                new_vars.insert("self".into(), instance_val);
                let frame = Frame {
                    ip: self.ip,
                    code: self.code.clone(),
                    vars: self.vars.clone(),
                    stack_depth: self.stack.len(),
                    try_stack_depth: self.try_stack.len(),
                };
                self.call_stack.push(frame);
                self.code = destroy.code.clone();
                self.ip = 0;
                self.vars = new_vars;
                loop {
                    if self.ip as usize >= self.code.len() {
                        break;
                    }
                    let inst = self.code[self.ip as usize].clone();
                    self.ip += 1;
                    self.exec_inst(inst)?;
                    if self.halted {
                        break;
                    }
                }
                if let Some(frame) = self.call_stack.pop() {
                    self.code = frame.code;
                    self.ip = frame.ip;
                    self.vars = frame.vars;
                }
            }
        }
        Ok(())
    }

}
