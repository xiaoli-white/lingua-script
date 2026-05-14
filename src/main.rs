use std::fs;
use clap::Parser;

#[derive(Parser)]
#[command(name = "lingua-script", version, about = "LinguaScript interpreter - A narrative programming language")]
struct Cli {
    #[arg(help = "Path to a .ls source file")]
    file: Option<String>,

    #[arg(short = 't', long = "tokens", help = "Print token list after lexing")]
    show_tokens: bool,

    #[arg(short = 'a', long = "ast", help = "Print AST after parsing")]
    show_ast: bool,

    #[arg(short = 'c', long = "code", help = "Print compiled bytecode")]
    show_code: bool,

    #[arg(short = 'i', long = "inspect", help = "Print tokens + AST + bytecode")]
    inspect: bool,

    #[arg(short = 'r', long = "repl", help = "Start interactive REPL")]
    repl: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.repl {
        run_repl();
    } else if let Some(path) = &cli.file {
        let source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: cannot read file {}: {}", path, e);
                std::process::exit(1);
            }
        };
        run_file(path, &source, &cli);
    } else {
        eprintln!("usage: lingua-script <source.ls>");
        eprintln!("       lingua-script --repl");
        eprintln!("For more info: lingua-script --help");
        std::process::exit(1);
    }
}

fn run_repl() {
    use std::io::{self, Write};

    println!("LinguaScript REPL (enter a blank line to execute, 'exit' to quit)");
    let mut buffer = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }

        let trimmed = line.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }

        if trimmed.is_empty() && !buffer.is_empty() {
            if let Err(e) = run_repl_input(&buffer) {
                eprintln!("error: {}", e);
            }
            buffer.clear();
            continue;
        }

        if !trimmed.is_empty() {
            buffer.push_str(line.as_str());
        }
    }
    if !buffer.is_empty() {
        if let Err(e) = run_repl_input(&buffer) {
            eprintln!("error: {}", e);
        }
    }
    println!("bye.");
}

fn run_repl_input(source: &str) -> Result<(), String> {
    use lingua_script::lexer::{Lexer, Token};
    use lingua_script::parser::Parser;
    use lingua_script::compiler::Compiler;
    use lingua_script::vm::VM;

    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        if tok == Token::EOF { break; }
        tokens.push(tok);
    }
    if tokens.is_empty() { return Ok(()); }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    let compiler = Compiler::new();
    let (code, func_table, class_table) = compiler.compile(&program);

    let mut vm = VM::new(code, func_table, class_table);
    vm.run()
}

fn run_file(path: &str, source: &str, cli: &Cli) {
    use lingua_script::lexer::{Lexer, Token};
    use lingua_script::parser::Parser;
    use lingua_script::compiler::Compiler;
    use lingua_script::vm::VM;

    let show_all = cli.inspect;

    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        if tok == Token::EOF { break; }
        tokens.push(tok);
    }

    if cli.show_tokens || show_all {
        println!("--- tokens ---");
        for (i, t) in tokens.iter().enumerate() {
            println!("{:>4}: {:?}", i, t);
        }
        println!();
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    if cli.show_ast || show_all {
        println!("--- ast ({} stmts) ---", program.stmts.len());
        for (i, s) in program.stmts.iter().enumerate() {
            println!("{:>4}: {:?}", i, s);
        }
        println!();
    }

    let compiler = if let Some(parent) = std::path::Path::new(path).parent() {
        let dir = parent.to_string_lossy().to_string();
        if dir.is_empty() {
            Compiler::new()
        } else {
            Compiler::with_source_dir(dir)
        }
    } else {
        Compiler::new()
    };
    let (code, func_table, class_table) = compiler.compile(&program);

    if cli.show_code || show_all {
        println!("--- bytecode ({} instrs) ---", code.len());
        for (i, inst) in code.iter().enumerate() {
            println!("{:>4}: {:?}", i, inst);
        }
        if !func_table.is_empty() {
            println!();
            for (fi, (name, params, fcode, _)) in func_table.iter().enumerate() {
                println!("--- function {}: {}({:?}) {} instrs ---", fi, name, params, fcode.len());
                for (i, inst) in fcode.iter().enumerate() {
                    println!("{:>4}: {:?}", i, inst);
                }
            }
        }
        println!();
    }

    let mut vm = VM::new(code, func_table, class_table);
    if let Err(e) = vm.run() {
        eprintln!("runtime error: {}", e);
        std::process::exit(1);
    }
}
