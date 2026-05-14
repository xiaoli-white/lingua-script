use std::collections::HashMap;
use std::fmt;

use crate::gc::Gc;
use crate::instruction::Instruction;

pub struct PendingDestroy {
    pub class: String,
    pub methods: Vec<ClassMethodDef>,
    pub fields: HashMap<String, Value>,
}

thread_local! {
    static DESTROY_QUEUE: std::cell::RefCell<Vec<PendingDestroy>> = std::cell::RefCell::new(Vec::new());
}

pub fn drain_destroy_queue() -> Vec<PendingDestroy> {
    DESTROY_QUEUE.with(|q| q.borrow_mut().drain(..).collect())
}

#[derive(Debug, Clone)]
pub struct SharedInstanceData {
    pub class: String,
    pub fields: HashMap<String, Value>,
    pub methods: Vec<ClassMethodDef>,
    pub has_destroy: bool,
}

impl Drop for SharedInstanceData {
    fn drop(&mut self) {
        if self.has_destroy {
            DESTROY_QUEUE.with(|q| {
                q.borrow_mut().push(PendingDestroy {
                    class: self.class.clone(),
                    methods: self.methods.clone(),
                    fields: self.fields.clone(),
                });
            });
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    List(Gc<Vec<Value>>),
    Map(Gc<HashMap<String, Value>>),
    Func {
        name: String,
        code: Vec<Instruction>,
        params: Vec<String>,
        captures: Vec<String>,
    },
    Class {
        name: String,
        fields: Vec<(String, Value)>,
        methods: Vec<ClassMethodDef>,
        publics: Vec<String>,
        constructor_params: Vec<String>,
    },
    SharedInstance(Gc<SharedInstanceData>),
    NativeFunc(FnPtr),
}

pub type FnPtr = fn(&[Value]) -> Result<Value, String>;

#[derive(Debug, Clone)]
pub struct ClassMethodDef {
    pub name: String,
    pub params: Vec<String>,
    pub code: Vec<Instruction>,
    pub captures: Vec<String>,
}

impl Value {
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Null => "null",
            Value::List(_) => "list",
            Value::Map(_) => "map",
            Value::Func { .. } => "function",
            Value::Class { .. } => "class",
            Value::SharedInstance(_) => "instance",
            Value::NativeFunc(_) => "native_function",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.borrow().is_empty(),
            Value::Map(m) => !m.borrow().is_empty(),
            _ => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if *n == n.floor() && n.is_finite() {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".into(),
            Value::List(l) => {
                let items: Vec<String> = l.borrow().iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Map(m) => {
                let items: Vec<String> = m.borrow().iter().map(|(k, v)| format!("{}: {}", k, v.to_string())).collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Func { name, .. } => format!("<function {}>", name),
            Value::Class { name, .. } => format!("<class {}>", name),
            Value::SharedInstance(data) => format!("<instance of {}>", data.borrow().class),
            Value::NativeFunc(_) => "<native_function>".into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
