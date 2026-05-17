use std::fs;
use clap::Parser as ClapParser;

#[derive(ClapParser)]
#[command(name = "lingua-script", version, about = "LinguaScript interpreter - A narrative programming language")]
struct Cli {
    #[arg(help = "Path to a .ls or .lsbc file")]
    file: Option<String>,

    #[arg(short = 't', long = "tokens", help = "Print token list after lexing")]
    show_tokens: bool,

    #[arg(short = 'a', long = "ast", help = "Print AST after parsing")]
    show_ast: bool,

    #[arg(short = 'c', long = "code", help = "Print compiled bytecode")]
    show_code: bool,

    #[arg(short = 'i', long = "inspect", help = "Print tokens + AST + bytecode")]
    inspect: bool,

    #[arg(short = 'o', long = "bytecode", help = "Export bytecode to file")]
    bytecode_output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(path) = &cli.file {
        if path.ends_with(".lsbc") {
            run_bytecode_file(path);
        } else {
            let source = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: cannot read file {}: {}", path, e);
                    std::process::exit(1);
                }
            };
            run_file(path, &source, &cli);
        }
    } else {
        eprintln!("usage: lingua-script <source.ls>");
        eprintln!("       lingua-script --repl");
        eprintln!("       lingua-script <source.ls> -o output.lsbc");
        eprintln!("       lingua-script <source.lsbc>");
        eprintln!("For more info: lingua-script --help");
        std::process::exit(1);
    }
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
            println!("{:>4}: {:#?}", i, s);
        }
        println!();
    }

    let compiler = if let Some(parent) = std::path::Path::new(path).parent() {
        let dir = parent.to_string_lossy().to_string();
        if dir.is_empty() { Compiler::new() } else { Compiler::with_source_dir(dir) }
    } else {
        Compiler::new()
    };
    let module = compiler.compile(&program);

    if let Some(output_path) = &cli.bytecode_output {
        let bytes = module.encode();
        match fs::write(output_path, bytes) {
            Ok(_) => println!("bytecode written to {}", output_path),
            Err(e) => eprintln!("error writing bytecode: {}", e),
        }
        return;
    }

    if cli.show_code || show_all {
        println!("--- bytecode ({} instrs) ---", module.main_code.len());
        println!("constants ({}):", module.constants.len());
        for (i, entry) in module.constants.items.iter().enumerate() {
            println!("  {}: {:?}", i, entry);
        }
        println!();
        for (i, inst) in module.main_code.iter().enumerate() {
            println!("{:>4}: {:?}", i, inst);
        }
        if !module.func_entries.is_empty() {
            println!();
            for (fi, fe) in module.func_entries.iter().enumerate() {
                let resolved = fe.resolve(&module.constants);
                println!("--- function {}: {}({:?}) {} instrs ---", fi, resolved.name, resolved.params, resolved.code.len());
                for (i, inst) in resolved.code.iter().enumerate() {
                    println!("{:>4}: {:?}", i, inst);
                }
            }
        }
        println!();
    }

    let (func_defs, class_defs) = module.resolve();
    let mut vm = VM::new(module.main_code, module.constants, func_defs, class_defs);
    if let Err(e) = vm.run() {
        eprintln!("runtime error: {}", e);
        std::process::exit(1);
    }
}

fn run_bytecode_file(path: &str) {
    use lingua_script::bytecode::BytecodeModule;
    use lingua_script::vm::VM;

    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: cannot read bytecode file {}: {}", path, e);
            std::process::exit(1);
        }
    };
    let module = BytecodeModule::decode(&bytes);
    let (func_defs, class_defs) = module.resolve();
    let mut vm = VM::new(module.main_code, module.constants, func_defs, class_defs);
    if let Err(e) = vm.run() {
        eprintln!("runtime error: {}", e);
        std::process::exit(1);
    }
}
