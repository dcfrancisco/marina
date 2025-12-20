// Marina - Clipper-2025 compiler and VM library
// This library provides the core components for compiling and executing Clipper programs

pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod diagnostics;
pub mod bytecode;
pub mod compiler;
pub mod vm;
pub mod formatter;

// Re-export commonly used types for convenience
pub use lexer::Lexer;
pub use parser::Parser;
pub use compiler::Compiler;
pub use vm::VM;
pub use ast::Program;
pub use bytecode::{Chunk, Value};

/// Compile and run a Clipper program from source code
pub fn run(source: &str, show_tokens: bool, show_ast: bool, show_disassembly: bool) -> Result<(), String> {
    // Lexical analysis
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.scan_tokens()?;
    
    if show_tokens {
        println!("=== TOKENS ===");
        for token in &tokens {
            println!("{:?}", token);
        }
        println!();
    }
    
    // Parsing
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    if show_ast {
        println!("=== AST ===");
        for stmt in &program.statements {
            println!("{:#?}", stmt);
        }
        println!();
    }
    
    // Compilation
    let compiler = Compiler::new();
    let (chunk, functions) = compiler.compile(program)?;
    
    if show_disassembly {
        chunk.disassemble("main");
        println!("\n=== Function Table ===");
        for (name, addr) in &functions {
            println!("{}: {}", name, addr);
        }
        println!();
    }
    
    // Execution
    let mut vm = VM::new();
    vm.run(&chunk, functions)?;
    
    Ok(())
}
