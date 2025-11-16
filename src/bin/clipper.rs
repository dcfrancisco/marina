use marina::run;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Clipper Compiler & VM");
        println!("Usage: {} [options] <file.prg>", args[0]);
        println!("\nOptions:");
        println!("  -d, --disassemble   Show disassembled bytecode");
        println!("  -t, --tokens        Show tokens");
        println!("  -a, --ast           Show AST");
        println!("  repl                Start REPL mode");
        std::process::exit(1);
    }
    
    if args[1] == "repl" {
        run_repl();
        return;
    }
    
    let mut show_tokens = false;
    let mut show_ast = false;
    let mut show_disassembly = false;
    let mut filename = None;
    
    for arg in &args[1..] {
        match arg.as_str() {
            "-t" | "--tokens" => show_tokens = true,
            "-a" | "--ast" => show_ast = true,
            "-d" | "--disassemble" => show_disassembly = true,
            _ => filename = Some(arg.clone()),
        }
    }
    
    let filename = filename.expect("No input file specified");
    
    let source = fs::read_to_string(&filename)
        .unwrap_or_else(|err| {
            eprintln!("Error reading file '{}': {}", filename, err);
            std::process::exit(1);
        });
    
    if let Err(e) = run(&source, show_tokens, show_ast, show_disassembly) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_repl() {
    println!("Clipper REPL - Type 'exit' to quit");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input == "exit" || input == "quit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match run(input, false, false, false) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
