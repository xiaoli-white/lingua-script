use crate::ast::*;
use crate::instruction::Instruction;
use crate::value::Value;

pub struct Compiler {
    code: Vec<Instruction>,
    func_table: Vec<(String, Vec<String>, Vec<Instruction>, Vec<String>)>,
    class_table: Vec<(String, Vec<(String, Vec<String>, Vec<Instruction>, Vec<String>)>)>,
    class_fields: Vec<(String, Vec<(String, crate::value::Value)>)>,
    interface_table: Vec<(String, Vec<crate::ast::InterfaceMethod>)>,
    loops: Vec<Vec<usize>>,
    source_dir: Option<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            code: Vec::new(),
            func_table: Vec::new(),
            class_table: Vec::new(),
            class_fields: Vec::new(),
            interface_table: Vec::new(),
            loops: Vec::new(),
            source_dir: None,
        }
    }

    pub fn with_source_dir(dir: String) -> Self {
        Compiler {
            code: Vec::new(),
            func_table: Vec::new(),
            class_table: Vec::new(),
            class_fields: Vec::new(),
            interface_table: Vec::new(),
            loops: Vec::new(),
            source_dir: Some(dir),
        }
    }

    fn emit(&mut self, inst: Instruction) {
        self.code.push(inst);
    }

    fn emit_at(&mut self, index: usize, inst: Instruction) {
        self.code[index] = inst;
    }

    fn current_pos(&self) -> usize {
        self.code.len()
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => self.emit(Instruction::Const(Value::Number(*n))),
            Expr::String(s) => self.emit(Instruction::Const(Value::String(s.clone()))),
            Expr::Bool(b) => self.emit(Instruction::Const(Value::Bool(*b))),
            Expr::Null => self.emit(Instruction::Const(Value::Null)),
            Expr::Identifier(name) => self.emit(Instruction::LoadVar(name.clone())),
            Expr::BinaryOp { op, left, right } => {
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod | BinOp::Pow => {
                        self.compile_expr(left);
                        self.compile_expr(right);
                        let inst = match op {
                            BinOp::Add => Instruction::Add,
                            BinOp::Sub => Instruction::Sub,
                            BinOp::Mul => Instruction::Mul,
                            BinOp::Div => Instruction::Div,
                            BinOp::Mod => Instruction::Mod,
                            BinOp::Pow => Instruction::Pow,
                            _ => unreachable!(),
                        };
                        self.emit(inst);
                    }
                    BinOp::And => {
                        self.compile_expr(left);
                        self.emit(Instruction::Dup);
                        let jump = self.current_pos();
                        self.emit(Instruction::JumpIfFalse(0));
                        self.emit(Instruction::Pop);
                        self.compile_expr(right);
                        let end = self.current_pos();
                        self.emit_at(jump, Instruction::JumpIfFalse((end - jump) as isize));
                    }
                    BinOp::Or => {
                        self.compile_expr(left);
                        self.emit(Instruction::Dup);
                        let jump = self.current_pos();
                        self.emit(Instruction::JumpIfTrue(0));
                        self.emit(Instruction::Pop);
                        self.compile_expr(right);
                        let end = self.current_pos();
                        self.emit_at(jump, Instruction::JumpIfTrue((end - jump) as isize));
                    }
                    BinOp::Eq => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Eq); }
                    BinOp::Ne => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Ne); }
                    BinOp::Gt => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Gt); }
                    BinOp::Lt => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Lt); }
                    BinOp::Ge => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Ge); }
                    BinOp::Le => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Le); }
                }
            }
            Expr::UnaryOp { op: UnaryOp::Neg, expr } => {
                self.compile_expr(expr);
                self.emit(Instruction::Neg);
            }
            Expr::UnaryOp { op: UnaryOp::Not, expr } => {
                self.compile_expr(expr);
                self.emit(Instruction::Not);
            }
            Expr::Call { callee, args } => {
                for arg in args {
                    self.compile_expr(arg);
                }
                if let Expr::Identifier(name) = callee.as_ref() {
                    if let Some(rest) = name.strip_prefix("__instantiate__") {
                        self.emit(Instruction::Instantiate(rest.to_string()));
                        return;
                    }
                }
                self.compile_expr(callee);
                self.emit(Instruction::Call(args.len()));
            }
            Expr::MethodCall { object, method, args } => {
                self.compile_expr(object);
                for arg in args {
                    self.compile_expr(arg);
                }
                self.emit(Instruction::LoadMethod(method.clone()));
                self.emit(Instruction::Call(args.len() + 1));
            }
            Expr::ListLiteral(items) => {
                for item in items {
                    self.compile_expr(item);
                }
                self.emit(Instruction::MakeList(items.len()));
            }
            Expr::MapLiteral(pairs) => {
                for (k, v) in pairs {
                    self.emit(Instruction::Const(Value::String(k.clone())));
                    self.compile_expr(v);
                }
                self.emit(Instruction::MakeMap(pairs.len()));
            }
            Expr::TypeOf(expr) => {
                self.compile_expr(expr);
                self.emit(Instruction::TypeOf);
            }
            Expr::Capitalize(expr) => {
                self.compile_expr(expr);
                self.emit(Instruction::Capitalize);
            }
            Expr::Input => {
                self.emit(Instruction::Input);
            }
            Expr::Index { object, index } => {
                self.compile_expr(object);
                self.compile_expr(index);
                self.emit(Instruction::Call(1));
            }
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::StartHere(body) => {
                for s in body {
                    self.compile_stmt(s);
                }
            }
            Stmt::VarDef { name, value } | Stmt::VarDecl { name, value } => {
                self.compile_expr(value);
                self.emit(Instruction::StoreVar(name.clone()));
            }
            Stmt::Assign { name, value } => {
                self.compile_expr(value);
                self.emit(Instruction::StoreVar(name.clone()));
            }
            Stmt::Say(expr) => {
                self.compile_expr(expr);
                self.emit(Instruction::Say);
            }
            Stmt::Ask { prompt, var } => {
                self.compile_expr(prompt);
                self.emit(Instruction::Ask(var.clone()));
            }
            Stmt::ReadFile { filename, var } => {
                self.compile_expr(filename);
                self.emit(Instruction::ReadFile(var.clone()));
            }
            Stmt::WriteFile { content, filename } => {
                self.compile_expr(content);
                self.compile_expr(filename);
                self.emit(Instruction::WriteFile);
            }
            Stmt::If { condition, body, otherwise } => {
                self.compile_expr(condition);
                let else_jump = self.current_pos();
                self.emit(Instruction::JumpIfFalse(0));
                for s in body {
                    self.compile_stmt(s);
                }
                let end_jump = self.current_pos();
                self.emit(Instruction::Jump(0));
                let else_start = self.current_pos();
                self.emit_at(else_jump, Instruction::JumpIfFalse((else_start - else_jump) as isize));
                for s in otherwise {
                    self.compile_stmt(s);
                }
                let end = self.current_pos();
                self.emit_at(end_jump, Instruction::Jump((end - end_jump) as isize));
            }
            Stmt::Repeat { times, body } => {
                let loop_id = self.loops.len();
                self.loops.push(Vec::new());
                self.compile_expr(times);
                let limit_var = format!("__repeat_limit_{}", loop_id);
                let counter_var = format!("__repeat_counter_{}", loop_id);
                self.emit(Instruction::StoreVar(limit_var.clone()));
                self.emit(Instruction::Const(Value::Number(0.0)));
                self.emit(Instruction::StoreVar(counter_var.clone()));
                let loop_start = self.current_pos();
                self.emit(Instruction::LoadVar(counter_var.clone()));
                self.emit(Instruction::LoadVar(limit_var.clone()));
                self.emit(Instruction::Lt);
                let exit_jump = self.current_pos();
                self.emit(Instruction::JumpIfFalse(0));
                for s in body {
                    self.compile_stmt(s);
                }
                self.emit(Instruction::LoadVar(counter_var.clone()));
                self.emit(Instruction::Const(Value::Number(1.0)));
                self.emit(Instruction::Add);
                self.emit(Instruction::StoreVar(counter_var.clone()));
                let after_jump = self.current_pos();
                self.emit(Instruction::Jump(-((after_jump - loop_start) as isize)));
                let exit = self.current_pos();
                self.emit_at(exit_jump, Instruction::JumpIfFalse((exit - exit_jump) as isize));
                self.loops.pop();
            }
            Stmt::ForEach { var, collection, body } => {
                let loop_id = self.loops.len();
                self.compile_expr(collection);
                self.emit(Instruction::MapToList);
                self.emit(Instruction::Dup);
                self.emit(Instruction::ListLen);
                self.emit(Instruction::StoreVar(format!("__foreach_len_{}", loop_id)));
                self.emit(Instruction::StoreVar(format!("__foreach_list_{}", loop_id)));
                self.emit(Instruction::Const(Value::Number(0.0)));
                self.emit(Instruction::StoreVar(format!("__foreach_idx_{}", loop_id)));

                let loop_start = self.current_pos();
                self.emit(Instruction::LoadVar(format!("__foreach_idx_{}", loop_id)));
                self.emit(Instruction::LoadVar(format!("__foreach_len_{}", loop_id)));
                self.emit(Instruction::Lt);
                let exit_jump = self.current_pos();
                self.emit(Instruction::JumpIfFalse(0));

                self.emit(Instruction::LoadVar(format!("__foreach_idx_{}", loop_id)));
                self.emit(Instruction::LoadVar(format!("__foreach_list_{}", loop_id)));
                self.emit(Instruction::IndexGet);
                self.emit(Instruction::StoreVar(var.clone()));

                for s in body {
                    self.compile_stmt(s);
                }

                self.emit(Instruction::LoadVar(format!("__foreach_idx_{}", loop_id)));
                self.emit(Instruction::Const(Value::Number(1.0)));
                self.emit(Instruction::Add);
                self.emit(Instruction::StoreVar(format!("__foreach_idx_{}", loop_id)));
                self.emit(Instruction::Jump(-((self.current_pos() - loop_start) as isize)));
                let exit = self.current_pos();
                self.emit_at(exit_jump, Instruction::JumpIfFalse((exit - exit_jump) as isize));
            }
            Stmt::While { condition, body } => {
                let loop_start = self.current_pos();
                self.compile_expr(condition);
                let exit_jump = self.current_pos();
                self.emit(Instruction::JumpIfFalse(0));
                for s in body {
                    self.compile_stmt(s);
                }
                self.emit(Instruction::Jump(-((self.current_pos() - loop_start) as isize)));
                let exit = self.current_pos();
                self.emit_at(exit_jump, Instruction::JumpIfFalse((exit - exit_jump) as isize));
            }
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.compile_stmt(s);
                }
            }
            Stmt::Return(Some(expr)) => {
                self.compile_expr(expr);
                self.emit(Instruction::Return);
            }
            Stmt::Return(None) => {
                self.emit(Instruction::Const(Value::Null));
                self.emit(Instruction::Return);
            }
            Stmt::Stop => {
                self.emit(Instruction::Stop);
            }
            Stmt::Exit(Some(expr)) => {
                self.compile_expr(expr);
                self.emit(Instruction::Exit);
            }
            Stmt::Exit(None) => {
                self.emit(Instruction::Const(Value::Number(0.0)));
                self.emit(Instruction::Exit);
            }
            Stmt::Raise(expr) => {
                self.compile_expr(expr);
                self.emit(Instruction::Raise);
            }
            Stmt::Try { body, catch_body, finally_body, .. } => {
                let try_pos = self.current_pos();
                self.emit(Instruction::TryCatch(0, 0));
                for s in body {
                    self.compile_stmt(s);
                }
                let jump_over_catch = self.current_pos();
                self.emit(Instruction::Jump(0));
                let catch_start = self.current_pos();
                for s in catch_body {
                    self.compile_stmt(s);
                }
                let finally_start = self.current_pos();
                for s in finally_body {
                    self.compile_stmt(s);
                }
                self.emit_at(try_pos, Instruction::TryCatch(
                    (catch_start - try_pos) as isize,
                    (finally_start - catch_start) as isize,
                ));
                self.emit_at(jump_over_catch, Instruction::Jump((finally_start - jump_over_catch) as isize));
                self.emit(Instruction::EndTry);
            }
            Stmt::Expression(expr) => {
                self.compile_expr(expr);
                self.emit(Instruction::Pop);
            }
            Stmt::AddToList { element, list } => {
                self.compile_expr(element);
                self.emit(Instruction::LoadVar(list.clone()));
                self.emit(Instruction::AddToList);
                self.emit(Instruction::StoreVar(list.clone()));
            }
            Stmt::RemoveFromList { element, list } => {
                self.compile_expr(element);
                self.emit(Instruction::LoadVar(list.clone()));
                self.emit(Instruction::RemoveFromList);
                self.emit(Instruction::StoreVar(list.clone()));
            }
            Stmt::Convert { expr, target_type, var } => {
                self.compile_expr(expr);
                self.emit(Instruction::Convert(target_type.clone()));
                if !var.is_empty() {
                    self.emit(Instruction::StoreVar(var.clone()));
                }
            }
            Stmt::FuncDef { name, params, body } => {
                let func_idx = self.func_table.len();
                let mut fcompiler = Compiler::new();
                for s in body {
                    fcompiler.compile_stmt(s);
                }
                fcompiler.emit(Instruction::Const(Value::Null));
                fcompiler.emit(Instruction::Return);
                let func_code = fcompiler.code;
                self.func_table.push((name.clone(), params.clone(), func_code, vec![]));
                self.emit(Instruction::MakeFunc(name.clone(), params.clone(), func_idx));
                self.emit(Instruction::StoreVar(name.clone()));
            }
            Stmt::FuncCall { func, args, result_var } => {
                if let Expr::Call { .. } = func {
                    self.compile_expr(func);
                } else {
                    for arg in args {
                        self.compile_expr(arg);
                    }
                    self.compile_expr(func);
                    self.emit(Instruction::Call(args.len()));
                }
                if let Some(var) = result_var {
                    self.emit(Instruction::StoreVar(var.clone()));
                } else {
                    self.emit(Instruction::Pop);
                }
            }
            Stmt::ClassDef { name, parent, implements, fields, constructor, destructor, methods, publics, .. } => {
                let mut compiled_methods = Vec::new();
                if let Some(ctor) = constructor {
                    let mut ccompiler = Compiler::new();
                    for s in &ctor.body {
                        ccompiler.compile_stmt(s);
                    }
                    ccompiler.emit(Instruction::Const(Value::Null));
                    ccompiler.emit(Instruction::Return);
                    compiled_methods.push((ctor.name.clone(), ctor.params.clone(), ccompiler.code, vec![]));
                }
                if let Some(dt) = destructor {
                    let mut dcompiler = Compiler::new();
                    for s in &dt.body {
                        dcompiler.compile_stmt(s);
                    }
                    dcompiler.emit(Instruction::Const(Value::Null));
                    dcompiler.emit(Instruction::Return);
                    compiled_methods.push((dt.name.clone(), dt.params.clone(), dcompiler.code, vec![]));
                }
                for m in methods {
                    let mut mcompiler = Compiler::new();
                    for s in &m.body {
                        mcompiler.compile_stmt(s);
                    }
                    mcompiler.emit(Instruction::Const(Value::Null));
                    mcompiler.emit(Instruction::Return);
                    compiled_methods.push((m.name.clone(), m.params.clone(), mcompiler.code, vec![]));
                }

                let mut field_values: Vec<(String, Value)> = fields.iter()
                    .map(|(n, _)| (n.clone(), Value::Null))
                    .collect();

                if let Some(ref parent_name) = parent {
                    if let Some(pf) = self.class_fields.iter().find(|(n, _)| n == parent_name) {
                        for (pf_name, pf_val) in &pf.1 {
                            if !field_values.iter().any(|(n, _)| n == pf_name) {
                                field_values.push((pf_name.clone(), pf_val.clone()));
                            }
                        }
                    }
                    if let Some(pm) = self.class_table.iter().find(|(n, _)| n == parent_name) {
                        for pm_entry in &pm.1 {
                            if !compiled_methods.iter().any(|(n, _, _, _)| n == &pm_entry.0) {
                                compiled_methods.push(pm_entry.clone());
                            }
                        }
                    }
                }

                for iface_name in implements {
                    if let Some(iface) = self.interface_table.iter().find(|(n, _)| n == iface_name) {
                        for im in &iface.1 {
                            if !compiled_methods.iter().any(|(mn, mp, _, _)| mn == &im.name && mp.len() == im.params.len()) {
                                panic!("class {} does not implement interface {}: method {}({}) not found",
                                    name, iface_name, im.name, im.params.join(", "));
                            }
                        }
                    } else {
                        panic!("interface {} not found", iface_name);
                    }
                }

                self.class_table.push((name.clone(), compiled_methods.clone()));
                self.class_fields.push((name.clone(), field_values.clone()));

                self.emit(Instruction::Const(Value::Class {
                    name: name.clone(),
                    fields: field_values,
                    methods: compiled_methods.iter().map(|(n, p, c, cap)| {
                        crate::value::ClassMethodDef {
                            name: n.clone(),
                            params: p.clone(),
                            code: c.clone(),
                            captures: cap.clone(),
                        }
                    }).collect(),
                    publics: publics.clone(),
                    constructor_params: if let Some(ctor) = &constructor { ctor.params.clone() } else { vec![] },
                }));
                self.emit(Instruction::StoreVar(name.clone()));
            }
            Stmt::Instantiate { class_name, args, var } => {
                for arg in args {
                    self.compile_expr(arg);
                }
                self.emit(Instruction::Instantiate(class_name.clone()));
                if let Some(v) = var {
                    self.emit(Instruction::StoreVar(v.clone()));
                } else {
                    self.emit(Instruction::Pop);
                }
            }
            Stmt::Chapter { stmts, .. } => {
                for s in stmts {
                    self.compile_stmt(s);
                }
            }
            Stmt::InterfaceDef { name, methods } => {
                self.interface_table.push((name.clone(), methods.clone()));
            }
            Stmt::Refer { module, symbols: _ } => {
                let mod_path = self.resolve_module_path(&module);
                if let Ok(source) = std::fs::read_to_string(&mod_path) {
                    let mut lexer = crate::lexer::Lexer::new(&source);
                    let mut tokens = Vec::new();
                    loop {
                        let tok = lexer.next_token();
                        if tok == crate::lexer::Token::EOF { break; }
                        if matches!(tok, crate::lexer::Token::Illegal(_)) { break; }
                        tokens.push(tok);
                    }
                    let mut parser = crate::parser::Parser::new(tokens);
                    let program = parser.parse_program();
                    for s in &program.stmts {
                        self.compile_stmt(s);
                    }
                }
            }
        }
    }

    fn resolve_module_path(&self, module: &str) -> String {
        let candidates = vec![
            module.to_string(),
            format!("{}.ls", module),
        ];
        let prefixed: Vec<String> = if let Some(ref dir) = self.source_dir {
            candidates.iter()
                .map(|p| format!("{}/{}", dir, p))
                .chain(candidates.clone())
                .collect()
        } else {
            candidates
        };
        for path in &prefixed {
            if std::path::Path::new(path).exists() {
                return path.clone();
            }
        }
        format!("{}.ls", module)
    }

    pub fn compile(mut self, program: &Program) -> (Vec<Instruction>, Vec<(String, Vec<String>, Vec<Instruction>, Vec<String>)>, Vec<(String, Vec<(String, Vec<String>, Vec<Instruction>, Vec<String>)>)>) {
        let has_start = program.stmts.iter().any(|s| matches!(s, Stmt::StartHere(_)));
        if has_start {
            for stmt in &program.stmts {
                if let Stmt::StartHere(body) = stmt {
                    for s in body {
                        self.compile_stmt(s);
                    }
                }
            }
        } else {
            for stmt in &program.stmts {
                self.compile_stmt(stmt);
            }
        }
        self.emit(Instruction::Halt);
        (self.code, self.func_table, self.class_table)
    }
}
