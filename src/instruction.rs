pub const OP_CONST: u8 = 0;
pub const OP_LOAD_VAR: u8 = 1;
pub const OP_STORE_VAR: u8 = 2;
pub const OP_LOAD_LOCAL: u8 = 3;
pub const OP_STORE_LOCAL: u8 = 4;
pub const OP_ADD: u8 = 5;
pub const OP_SUB: u8 = 6;
pub const OP_MUL: u8 = 7;
pub const OP_DIV: u8 = 8;
pub const OP_MOD: u8 = 9;
pub const OP_POW: u8 = 10;
pub const OP_NEG: u8 = 11;
pub const OP_AND: u8 = 12;
pub const OP_OR: u8 = 13;
pub const OP_NOT: u8 = 14;
pub const OP_EQ: u8 = 15;
pub const OP_NE: u8 = 16;
pub const OP_GT: u8 = 17;
pub const OP_LT: u8 = 18;
pub const OP_GE: u8 = 19;
pub const OP_LE: u8 = 20;
pub const OP_JUMP: u8 = 21;
pub const OP_JUMP_IF_FALSE: u8 = 22;
pub const OP_JUMP_IF_TRUE: u8 = 23;
pub const OP_CALL: u8 = 24;
pub const OP_RETURN: u8 = 25;
pub const OP_MAKE_FUNC: u8 = 26;
pub const OP_MAKE_CLOSURE: u8 = 27;
pub const OP_MAKE_CLASS: u8 = 28;
pub const OP_INSTANTIATE: u8 = 29;
pub const OP_LOAD_METHOD: u8 = 30;
pub const OP_MAKE_LIST: u8 = 31;
pub const OP_MAKE_MAP: u8 = 32;
pub const OP_INDEX_GET: u8 = 33;
pub const OP_LIST_LEN: u8 = 34;
pub const OP_ADD_TO_LIST: u8 = 35;
pub const OP_REMOVE_FROM_LIST: u8 = 36;
pub const OP_MAP_TO_LIST: u8 = 37;
pub const OP_SAY: u8 = 38;
pub const OP_ASK: u8 = 39;
pub const OP_READ_FILE: u8 = 40;
pub const OP_WRITE_FILE: u8 = 41;
pub const OP_RAISE: u8 = 42;
pub const OP_TRY_CATCH: u8 = 43;
pub const OP_END_TRY: u8 = 44;
pub const OP_TYPE_OF: u8 = 45;
pub const OP_CONVERT: u8 = 46;
pub const OP_CAPITALIZE: u8 = 47;
pub const OP_INPUT: u8 = 48;
pub const OP_DUP: u8 = 49;
pub const OP_POP: u8 = 50;
pub const OP_STOP: u8 = 51;
pub const OP_EXIT: u8 = 52;
pub const OP_HALT: u8 = 53;
pub const OP_RETURN_FRAME_AS_MAP: u8 = 54;
pub const OP_FILTER_MAP: u8 = 55;
pub const OP_MAKE_STD_MODULE: u8 = 56;

#[derive(Debug, Clone)]
pub enum Instruction {
    Const(u32),
    LoadVar(u32),
    StoreVar(u32),
    LoadLocal(u32),
    StoreLocal(u32),
    Add, Sub, Mul, Div, Mod, Pow,
    Neg,
    And, Or, Not,
    Eq, Ne, Gt, Lt, Ge, Le,
    Jump(i32),
    JumpIfFalse(i32),
    JumpIfTrue(i32),
    Call(u32),
    Return,
    MakeFunc(u32),
    MakeClosure(u32, Vec<u32>),
    MakeClass(u32),
    Instantiate(u32),
    LoadMethod(u32),
    MakeList(u32),
    MakeMap(u32),
    IndexGet, ListLen, AddToList, RemoveFromList, MapToList,
    Say,
    Ask(u32),
    ReadFile(u32),
    WriteFile,
    Raise,
    TryCatch(i32, i32),
    EndTry,
    TypeOf,
    Convert(u32),
    Capitalize,
    Input,
    Dup, Pop,
    Stop, Exit, Halt,
    ReturnFrameAsMap,
    FilterMap(Vec<u32>),
    MakeStdModule(u32),
}

impl Instruction {
    pub fn opcode(&self) -> u8 {
        match self {
            Instruction::Const(_) => OP_CONST,
            Instruction::LoadVar(_) => OP_LOAD_VAR,
            Instruction::StoreVar(_) => OP_STORE_VAR,
            Instruction::LoadLocal(_) => OP_LOAD_LOCAL,
            Instruction::StoreLocal(_) => OP_STORE_LOCAL,
            Instruction::Add => OP_ADD,
            Instruction::Sub => OP_SUB,
            Instruction::Mul => OP_MUL,
            Instruction::Div => OP_DIV,
            Instruction::Mod => OP_MOD,
            Instruction::Pow => OP_POW,
            Instruction::Neg => OP_NEG,
            Instruction::And => OP_AND,
            Instruction::Or => OP_OR,
            Instruction::Not => OP_NOT,
            Instruction::Eq => OP_EQ,
            Instruction::Ne => OP_NE,
            Instruction::Gt => OP_GT,
            Instruction::Lt => OP_LT,
            Instruction::Ge => OP_GE,
            Instruction::Le => OP_LE,
            Instruction::Jump(_) => OP_JUMP,
            Instruction::JumpIfFalse(_) => OP_JUMP_IF_FALSE,
            Instruction::JumpIfTrue(_) => OP_JUMP_IF_TRUE,
            Instruction::Call(_) => OP_CALL,
            Instruction::Return => OP_RETURN,
            Instruction::MakeFunc(_) => OP_MAKE_FUNC,
            Instruction::MakeClosure(_, _) => OP_MAKE_CLOSURE,
            Instruction::MakeClass(_) => OP_MAKE_CLASS,
            Instruction::Instantiate(_) => OP_INSTANTIATE,
            Instruction::LoadMethod(_) => OP_LOAD_METHOD,
            Instruction::MakeList(_) => OP_MAKE_LIST,
            Instruction::MakeMap(_) => OP_MAKE_MAP,
            Instruction::IndexGet => OP_INDEX_GET,
            Instruction::ListLen => OP_LIST_LEN,
            Instruction::AddToList => OP_ADD_TO_LIST,
            Instruction::RemoveFromList => OP_REMOVE_FROM_LIST,
            Instruction::MapToList => OP_MAP_TO_LIST,
            Instruction::Say => OP_SAY,
            Instruction::Ask(_) => OP_ASK,
            Instruction::ReadFile(_) => OP_READ_FILE,
            Instruction::WriteFile => OP_WRITE_FILE,
            Instruction::Raise => OP_RAISE,
            Instruction::TryCatch(_, _) => OP_TRY_CATCH,
            Instruction::EndTry => OP_END_TRY,
            Instruction::TypeOf => OP_TYPE_OF,
            Instruction::Convert(_) => OP_CONVERT,
            Instruction::Capitalize => OP_CAPITALIZE,
            Instruction::Input => OP_INPUT,
            Instruction::Dup => OP_DUP,
            Instruction::Pop => OP_POP,
            Instruction::Stop => OP_STOP,
            Instruction::Exit => OP_EXIT,
            Instruction::Halt => OP_HALT,
            Instruction::ReturnFrameAsMap => OP_RETURN_FRAME_AS_MAP,
            Instruction::FilterMap(_) => OP_FILTER_MAP,
            Instruction::MakeStdModule(_) => OP_MAKE_STD_MODULE,
        }
    }

    pub fn encode_all(instructions: &[Instruction]) -> Vec<u8> {
        let mut buf = Vec::new();
        for inst in instructions {
            inst.encode_into(&mut buf);
        }
        buf
    }

    fn encode_into(&self, buf: &mut Vec<u8>) {
        fn push_u32(b: &mut Vec<u8>, v: u32) { b.extend_from_slice(&v.to_le_bytes()); }
        fn push_i32(b: &mut Vec<u8>, v: i32) { b.extend_from_slice(&v.to_le_bytes()); }

        match self {
            Instruction::Const(v) => { buf.push(OP_CONST); push_u32(buf, *v); }
            Instruction::LoadVar(v) => { buf.push(OP_LOAD_VAR); push_u32(buf, *v); }
            Instruction::StoreVar(v) => { buf.push(OP_STORE_VAR); push_u32(buf, *v); }
            Instruction::LoadLocal(v) => { buf.push(OP_LOAD_LOCAL); push_u32(buf, *v); }
            Instruction::StoreLocal(v) => { buf.push(OP_STORE_LOCAL); push_u32(buf, *v); }
            Instruction::Add => buf.push(OP_ADD),
            Instruction::Sub => buf.push(OP_SUB),
            Instruction::Mul => buf.push(OP_MUL),
            Instruction::Div => buf.push(OP_DIV),
            Instruction::Mod => buf.push(OP_MOD),
            Instruction::Pow => buf.push(OP_POW),
            Instruction::Neg => buf.push(OP_NEG),
            Instruction::And => buf.push(OP_AND),
            Instruction::Or => buf.push(OP_OR),
            Instruction::Not => buf.push(OP_NOT),
            Instruction::Eq => buf.push(OP_EQ),
            Instruction::Ne => buf.push(OP_NE),
            Instruction::Gt => buf.push(OP_GT),
            Instruction::Lt => buf.push(OP_LT),
            Instruction::Ge => buf.push(OP_GE),
            Instruction::Le => buf.push(OP_LE),
            Instruction::Jump(v) => { buf.push(OP_JUMP); push_i32(buf, *v); }
            Instruction::JumpIfFalse(v) => { buf.push(OP_JUMP_IF_FALSE); push_i32(buf, *v); }
            Instruction::JumpIfTrue(v) => { buf.push(OP_JUMP_IF_TRUE); push_i32(buf, *v); }
            Instruction::Call(v) => { buf.push(OP_CALL); push_u32(buf, *v); }
            Instruction::Return => buf.push(OP_RETURN),
            Instruction::MakeFunc(v) => { buf.push(OP_MAKE_FUNC); push_u32(buf, *v); }
            Instruction::MakeClosure(idx, caps) => {
                buf.push(OP_MAKE_CLOSURE);
                push_u32(buf, *idx);
                push_u32(buf, caps.len() as u32);
                for &c in caps { push_u32(buf, c); }
            }
            Instruction::MakeClass(v) => { buf.push(OP_MAKE_CLASS); push_u32(buf, *v); }
            Instruction::Instantiate(v) => { buf.push(OP_INSTANTIATE); push_u32(buf, *v); }
            Instruction::LoadMethod(v) => { buf.push(OP_LOAD_METHOD); push_u32(buf, *v); }
            Instruction::MakeList(v) => { buf.push(OP_MAKE_LIST); push_u32(buf, *v); }
            Instruction::MakeMap(v) => { buf.push(OP_MAKE_MAP); push_u32(buf, *v); }
            Instruction::IndexGet => buf.push(OP_INDEX_GET),
            Instruction::ListLen => buf.push(OP_LIST_LEN),
            Instruction::AddToList => buf.push(OP_ADD_TO_LIST),
            Instruction::RemoveFromList => buf.push(OP_REMOVE_FROM_LIST),
            Instruction::MapToList => buf.push(OP_MAP_TO_LIST),
            Instruction::Say => buf.push(OP_SAY),
            Instruction::Ask(v) => { buf.push(OP_ASK); push_u32(buf, *v); }
            Instruction::ReadFile(v) => { buf.push(OP_READ_FILE); push_u32(buf, *v); }
            Instruction::WriteFile => buf.push(OP_WRITE_FILE),
            Instruction::Raise => buf.push(OP_RAISE),
            Instruction::TryCatch(a, b) => { buf.push(OP_TRY_CATCH); push_i32(buf, *a); push_i32(buf, *b); }
            Instruction::EndTry => buf.push(OP_END_TRY),
            Instruction::TypeOf => buf.push(OP_TYPE_OF),
            Instruction::Convert(v) => { buf.push(OP_CONVERT); push_u32(buf, *v); }
            Instruction::Capitalize => buf.push(OP_CAPITALIZE),
            Instruction::Input => buf.push(OP_INPUT),
            Instruction::Dup => buf.push(OP_DUP),
            Instruction::Pop => buf.push(OP_POP),
            Instruction::Stop => buf.push(OP_STOP),
            Instruction::Exit => buf.push(OP_EXIT),
            Instruction::Halt => buf.push(OP_HALT),
            Instruction::ReturnFrameAsMap => buf.push(OP_RETURN_FRAME_AS_MAP),
            Instruction::FilterMap(keys) => {
                buf.push(OP_FILTER_MAP);
                push_u32(buf, keys.len() as u32);
                for &k in keys { push_u32(buf, k); }
            }
            Instruction::MakeStdModule(idx) => {
                buf.push(OP_MAKE_STD_MODULE);
                push_u32(buf, *idx);
            }
        }
    }

    pub fn decode_all(data: &[u8]) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let mut offset = 0;
        while offset < data.len() {
            instructions.push(Self::decode_one(data, &mut offset));
        }
        instructions
    }

    fn decode_one(data: &[u8], offset: &mut usize) -> Instruction {
        fn read_u32(d: &[u8], o: &mut usize) -> u32 {
            let v = u32::from_le_bytes(d[*o..*o + 4].try_into().unwrap());
            *o += 4;
            v
        }
        fn read_i32(d: &[u8], o: &mut usize) -> i32 {
            let v = i32::from_le_bytes(d[*o..*o + 4].try_into().unwrap());
            *o += 4;
            v
        }

        let op = data[*offset];
        *offset += 1;
        match op {
            OP_CONST => Instruction::Const(read_u32(data, offset)),
            OP_LOAD_VAR => Instruction::LoadVar(read_u32(data, offset)),
            OP_STORE_VAR => Instruction::StoreVar(read_u32(data, offset)),
            OP_LOAD_LOCAL => Instruction::LoadLocal(read_u32(data, offset)),
            OP_STORE_LOCAL => Instruction::StoreLocal(read_u32(data, offset)),
            OP_ADD => Instruction::Add,
            OP_SUB => Instruction::Sub,
            OP_MUL => Instruction::Mul,
            OP_DIV => Instruction::Div,
            OP_MOD => Instruction::Mod,
            OP_POW => Instruction::Pow,
            OP_NEG => Instruction::Neg,
            OP_AND => Instruction::And,
            OP_OR => Instruction::Or,
            OP_NOT => Instruction::Not,
            OP_EQ => Instruction::Eq,
            OP_NE => Instruction::Ne,
            OP_GT => Instruction::Gt,
            OP_LT => Instruction::Lt,
            OP_GE => Instruction::Ge,
            OP_LE => Instruction::Le,
            OP_JUMP => Instruction::Jump(read_i32(data, offset)),
            OP_JUMP_IF_FALSE => Instruction::JumpIfFalse(read_i32(data, offset)),
            OP_JUMP_IF_TRUE => Instruction::JumpIfTrue(read_i32(data, offset)),
            OP_CALL => Instruction::Call(read_u32(data, offset)),
            OP_RETURN => Instruction::Return,
            OP_MAKE_FUNC => Instruction::MakeFunc(read_u32(data, offset)),
            OP_MAKE_CLOSURE => {
                let idx = read_u32(data, offset);
                let cc = read_u32(data, offset) as usize;
                let mut caps = Vec::with_capacity(cc);
                for _ in 0..cc { caps.push(read_u32(data, offset)); }
                Instruction::MakeClosure(idx, caps)
            }
            OP_MAKE_CLASS => Instruction::MakeClass(read_u32(data, offset)),
            OP_INSTANTIATE => Instruction::Instantiate(read_u32(data, offset)),
            OP_LOAD_METHOD => Instruction::LoadMethod(read_u32(data, offset)),
            OP_MAKE_LIST => Instruction::MakeList(read_u32(data, offset)),
            OP_MAKE_MAP => Instruction::MakeMap(read_u32(data, offset)),
            OP_INDEX_GET => Instruction::IndexGet,
            OP_LIST_LEN => Instruction::ListLen,
            OP_ADD_TO_LIST => Instruction::AddToList,
            OP_REMOVE_FROM_LIST => Instruction::RemoveFromList,
            OP_MAP_TO_LIST => Instruction::MapToList,
            OP_SAY => Instruction::Say,
            OP_ASK => Instruction::Ask(read_u32(data, offset)),
            OP_READ_FILE => Instruction::ReadFile(read_u32(data, offset)),
            OP_WRITE_FILE => Instruction::WriteFile,
            OP_RAISE => Instruction::Raise,
            OP_TRY_CATCH => Instruction::TryCatch(read_i32(data, offset), read_i32(data, offset)),
            OP_END_TRY => Instruction::EndTry,
            OP_TYPE_OF => Instruction::TypeOf,
            OP_CONVERT => Instruction::Convert(read_u32(data, offset)),
            OP_CAPITALIZE => Instruction::Capitalize,
            OP_INPUT => Instruction::Input,
            OP_DUP => Instruction::Dup,
            OP_POP => Instruction::Pop,
            OP_STOP => Instruction::Stop,
            OP_EXIT => Instruction::Exit,
            OP_HALT => Instruction::Halt,
            OP_RETURN_FRAME_AS_MAP => Instruction::ReturnFrameAsMap,
            OP_FILTER_MAP => {
                let count = read_u32(data, offset) as usize;
                let mut keys = Vec::with_capacity(count);
                for _ in 0..count { keys.push(read_u32(data, offset)); }
                Instruction::FilterMap(keys)
            }
            OP_MAKE_STD_MODULE => Instruction::MakeStdModule(read_u32(data, offset)),
            _ => panic!("unknown opcode {}", op),
        }
    }
}
