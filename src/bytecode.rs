use std::collections::HashMap;
use crate::instruction::Instruction;
use crate::value::{ClassMethodDef, Value};

#[derive(Debug, Clone)]
pub enum PoolEntry {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl PoolEntry {
    pub fn to_value(&self) -> Value {
        match self {
            PoolEntry::Number(n) => Value::Number(*n),
            PoolEntry::String(s) => Value::String(s.clone()),
            PoolEntry::Bool(b) => Value::Bool(*b),
            PoolEntry::Null => Value::Null,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            PoolEntry::String(s) => s.clone(),
            _ => panic!("not a string"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstantPool {
    pub items: Vec<PoolEntry>,
    string_map: HashMap<String, u32>,
    number_map: HashMap<u64, u32>,
}

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool { items: Vec::new(), string_map: HashMap::new(), number_map: HashMap::new() }
    }

    pub fn add_string(&mut self, s: &str) -> u32 {
        if let Some(&idx) = self.string_map.get(s) { return idx; }
        let idx = self.items.len() as u32;
        self.items.push(PoolEntry::String(s.to_string()));
        self.string_map.insert(s.to_string(), idx);
        idx
    }

    pub fn add_number(&mut self, n: f64) -> u32 {
        let bits = n.to_bits();
        if let Some(&idx) = self.number_map.get(&bits) { return idx; }
        let idx = self.items.len() as u32;
        self.items.push(PoolEntry::Number(n));
        self.number_map.insert(bits, idx);
        idx
    }

    pub fn add_bool(&mut self, b: bool) -> u32 {
        for (i, item) in self.items.iter().enumerate() {
            if let PoolEntry::Bool(v) = item {
                if *v == b { return i as u32; }
            }
        }
        let idx = self.items.len() as u32;
        self.items.push(PoolEntry::Bool(b));
        idx
    }

    pub fn add_null(&mut self) -> u32 {
        for (i, item) in self.items.iter().enumerate() {
            if let PoolEntry::Null = item { return i as u32; }
        }
        let idx = self.items.len() as u32;
        self.items.push(PoolEntry::Null);
        idx
    }

    pub fn add_value(&mut self, v: &Value) -> u32 {
        match v {
            Value::Number(n) => self.add_number(*n),
            Value::String(s) => self.add_string(s),
            Value::Bool(b) => self.add_bool(*b),
            Value::Null => self.add_null(),
            _ => panic!("cannot add complex value to constant pool"),
        }
    }

    pub fn get(&self, idx: u32) -> &PoolEntry {
        &self.items[idx as usize]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&(self.items.len() as u32).to_le_bytes());
        for item in &self.items {
            match item {
                PoolEntry::Number(n) => { buf.push(0); buf.extend_from_slice(&n.to_le_bytes()); }
                PoolEntry::String(s) => {
                    buf.push(1);
                    let bytes = s.as_bytes();
                    buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                    buf.extend_from_slice(bytes);
                }
                PoolEntry::Bool(false) => buf.push(2),
                PoolEntry::Bool(true) => buf.push(3),
                PoolEntry::Null => buf.push(4),
            }
        }
        buf
    }

    pub fn decode(data: &[u8], offset: &mut usize) -> Self {
        let count = u32::from_le_bytes(data[*offset..*offset + 4].try_into().unwrap()) as usize;
        *offset += 4;
        let mut pool = ConstantPool::new();
        for _ in 0..count {
            let tag = data[*offset];
            *offset += 1;
            match tag {
                0 => {
                    let n = f64::from_le_bytes(data[*offset..*offset + 8].try_into().unwrap());
                    *offset += 8;
                    pool.items.push(PoolEntry::Number(n));
                }
                1 => {
                    let len = u32::from_le_bytes(data[*offset..*offset + 4].try_into().unwrap()) as usize;
                    *offset += 4;
                    let s = String::from_utf8(data[*offset..*offset + len].to_vec()).unwrap();
                    *offset += len;
                    pool.items.push(PoolEntry::String(s));
                }
                2 => pool.items.push(PoolEntry::Bool(false)),
                3 => pool.items.push(PoolEntry::Bool(true)),
                4 => pool.items.push(PoolEntry::Null),
                _ => panic!("unknown pool entry type {}", tag),
            }
        }
        pool
    }
}

#[derive(Debug, Clone)]
pub struct FuncEntry {
    pub name_idx: u32,
    pub params: Vec<u32>,
    pub captures: Vec<u32>,
    pub code: Vec<Instruction>,
}

impl FuncEntry {
    pub fn encode_into(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.name_idx.to_le_bytes());
        buf.extend_from_slice(&(self.params.len() as u32).to_le_bytes());
        for &p in &self.params { buf.extend_from_slice(&p.to_le_bytes()); }
        buf.extend_from_slice(&(self.captures.len() as u32).to_le_bytes());
        for &c in &self.captures { buf.extend_from_slice(&c.to_le_bytes()); }
        let code_bytes = Instruction::encode_all(&self.code);
        buf.extend_from_slice(&(code_bytes.len() as u32).to_le_bytes());
        buf.extend_from_slice(&code_bytes);
    }

    pub fn decode(data: &[u8], offset: &mut usize) -> Self {
        let name_idx = read_u32(data, offset);
        let pc = read_u32(data, offset) as usize;
        let mut params = Vec::with_capacity(pc);
        for _ in 0..pc { params.push(read_u32(data, offset)); }
        let cc = read_u32(data, offset) as usize;
        let mut captures = Vec::with_capacity(cc);
        for _ in 0..cc { captures.push(read_u32(data, offset)); }
        let cl = read_u32(data, offset) as usize;
        let code = Instruction::decode_all(&data[*offset..*offset + cl]);
        *offset += cl;
        FuncEntry { name_idx, params, captures, code }
    }
}

#[derive(Debug, Clone)]
pub struct ClassEntry {
    pub name_idx: u32,
    pub field_names: Vec<u32>,
    pub methods: Vec<FuncEntry>,
    pub publics: Vec<u32>,
    pub constructor_params: Vec<u32>,
}

impl ClassEntry {
    pub fn encode_into(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.name_idx.to_le_bytes());
        buf.extend_from_slice(&(self.field_names.len() as u32).to_le_bytes());
        for &f in &self.field_names { buf.extend_from_slice(&f.to_le_bytes()); }
        buf.extend_from_slice(&(self.methods.len() as u32).to_le_bytes());
        for m in &self.methods { m.encode_into(buf); }
        buf.extend_from_slice(&(self.publics.len() as u32).to_le_bytes());
        for &p in &self.publics { buf.extend_from_slice(&p.to_le_bytes()); }
        buf.extend_from_slice(&(self.constructor_params.len() as u32).to_le_bytes());
        for &p in &self.constructor_params { buf.extend_from_slice(&p.to_le_bytes()); }
    }

    pub fn decode(data: &[u8], offset: &mut usize) -> Self {
        let name_idx = read_u32(data, offset);
        let fc = read_u32(data, offset) as usize;
        let mut field_names = Vec::with_capacity(fc);
        for _ in 0..fc { field_names.push(read_u32(data, offset)); }
        let mc = read_u32(data, offset) as usize;
        let mut methods = Vec::with_capacity(mc);
        for _ in 0..mc { methods.push(FuncEntry::decode(data, offset)); }
        let pc = read_u32(data, offset) as usize;
        let mut publics = Vec::with_capacity(pc);
        for _ in 0..pc { publics.push(read_u32(data, offset)); }
        let cc = read_u32(data, offset) as usize;
        let mut constructor_params = Vec::with_capacity(cc);
        for _ in 0..cc { constructor_params.push(read_u32(data, offset)); }
        ClassEntry { name_idx, field_names, methods, publics, constructor_params }
    }
}

fn read_u32(data: &[u8], offset: &mut usize) -> u32 {
    let val = u32::from_le_bytes(data[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    val
}

pub fn read_u32_from(data: &[u8], offset: &mut usize) -> u32 {
    read_u32(data, offset)
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: String,
    pub params: Vec<String>,
    pub code: Vec<Instruction>,
    pub captures: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub name: String,
    pub fields: Vec<(String, Value)>,
    pub methods: Vec<ClassMethodDef>,
    pub publics: Vec<String>,
    pub constructor_params: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BytecodeModule {
    pub constants: ConstantPool,
    pub func_entries: Vec<FuncEntry>,
    pub class_entries: Vec<ClassEntry>,
    pub main_code: Vec<Instruction>,
}

impl BytecodeModule {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"LSBC");
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&self.constants.encode());
        buf.extend_from_slice(&(self.func_entries.len() as u32).to_le_bytes());
        for fe in &self.func_entries { fe.encode_into(&mut buf); }
        buf.extend_from_slice(&(self.class_entries.len() as u32).to_le_bytes());
        for ce in &self.class_entries { ce.encode_into(&mut buf); }
        let code_bytes = Instruction::encode_all(&self.main_code);
        buf.extend_from_slice(&(code_bytes.len() as u32).to_le_bytes());
        buf.extend_from_slice(&code_bytes);
        buf
    }

    pub fn decode(data: &[u8]) -> Self {
        let mut offset = 0;
        assert!(&data[0..4] == b"LSBC", "invalid bytecode magic");
        offset += 4;
        let _version = read_u32(data, &mut offset);
        let constants = ConstantPool::decode(data, &mut offset);
        let fc = read_u32(data, &mut offset) as usize;
        let mut func_entries = Vec::with_capacity(fc);
        for _ in 0..fc { func_entries.push(FuncEntry::decode(data, &mut offset)); }
        let cc = read_u32(data, &mut offset) as usize;
        let mut class_entries = Vec::with_capacity(cc);
        for _ in 0..cc { class_entries.push(ClassEntry::decode(data, &mut offset)); }
        let cl = read_u32(data, &mut offset) as usize;
        let main_code = Instruction::decode_all(&data[offset..offset + cl]);
        BytecodeModule { constants, func_entries, class_entries, main_code }
    }

    pub fn resolve(&self) -> (Vec<FuncDef>, Vec<ClassDef>) {
        let func_defs: Vec<FuncDef> = self.func_entries.iter()
            .map(|fe| fe.resolve(&self.constants))
            .collect();
        let class_defs: Vec<ClassDef> = self.class_entries.iter()
            .map(|ce| ce.resolve(&self.constants))
            .collect();
        (func_defs, class_defs)
    }
}

impl FuncEntry {
    pub fn resolve(&self, constants: &ConstantPool) -> FuncDef {
        FuncDef {
            name: constants.get(self.name_idx).as_string(),
            params: self.params.iter().map(|&i| constants.get(i).as_string()).collect(),
            code: self.code.clone(),
            captures: self.captures.iter().map(|&i| constants.get(i).as_string()).collect(),
        }
    }
}

impl ClassEntry {
    pub fn resolve(&self, constants: &ConstantPool) -> ClassDef {
        ClassDef {
            name: constants.get(self.name_idx).as_string(),
            fields: self.field_names.iter().map(|&i| (constants.get(i).as_string(), Value::Null)).collect(),
            methods: self.methods.iter().map(|m| {
                let resolved = m.resolve(constants);
                ClassMethodDef {
                    name: resolved.name,
                    params: resolved.params,
                    code: resolved.code,
                    captures: resolved.captures,
                }
            }).collect(),
            publics: self.publics.iter().map(|&i| constants.get(i).as_string()).collect(),
            constructor_params: self.constructor_params.iter().map(|&i| constants.get(i).as_string()).collect(),
        }
    }
}
