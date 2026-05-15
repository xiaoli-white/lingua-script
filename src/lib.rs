pub mod number;
pub mod gc;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod instruction;
pub mod value;
pub mod bytecode;
pub mod compiler;
pub mod vm;

use lexer::{Lexer, Token};
use parser::Parser;
use compiler::Compiler;

pub fn execute(source: &str) -> Result<(), String> {
    let _ = value::drain_destroy_queue();
    let module = compile(source)?;
    let (func_defs, class_defs) = module.resolve();
    let mut vm = vm::VM::new(module.main_code, module.constants, func_defs, class_defs);
    vm.run()
}

pub fn compile(source: &str) -> Result<bytecode::BytecodeModule, String> {
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
    Ok(compiler.compile(&program))
}
