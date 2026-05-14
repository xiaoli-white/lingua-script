pub mod number;
pub mod gc;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod instruction;
pub mod value;
pub mod compiler;
pub mod vm;

use lexer::{Lexer, Token};
use parser::Parser;
use compiler::Compiler;

pub fn execute(source: &str) -> Result<(), String> {
    let _ = value::drain_destroy_queue();
    let (code, func_table, class_table, _) = compile(source)?;
    let mut vm = vm::VM::new(code, func_table, class_table);
    vm.run()
}

pub fn compile(source: &str) -> Result<(Vec<instruction::Instruction>, Vec<(String, Vec<String>, Vec<instruction::Instruction>, Vec<String>)>, Vec<(String, Vec<(String, Vec<String>, Vec<instruction::Instruction>, Vec<String>)>)>, std::collections::HashMap<String, value::Value>), String> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        if tok == Token::EOF { break; }
        tokens.push(tok);
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    let compiler = Compiler::new();
    let (code, func_table, class_table) = compiler.compile(&program);
    Ok((code, func_table, class_table, std::collections::HashMap::new()))
}
