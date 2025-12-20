// Marina Code Formatter (marina-fmt)
// Formats Clipper-2025 (.prg) files with consistent style

use marina::{formatter, Lexer, Parser};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }
    
    let mut check_only = false;
    let mut files = Vec::new();
    
    for arg in &args[1..] {
        match arg.as_str() {
            "--check" => check_only = true,
            "--help" | "-h" => {
                print_usage(&args[0]);
                std::process::exit(0);
            }
            file if file.ends_with(".prg") => files.push(file.to_string()),
            _ => {
                eprintln!("Error: Unknown option or invalid file: {}", arg);
                std::process::exit(1);
            }
        }
    }
    
    if files.is_empty() {
        eprintln!("Error: No .prg files specified");
        print_usage(&args[0]);
        std::process::exit(1);
    }
    
    for file in files {
        if let Err(e) = format_file(&file, check_only) {
            eprintln!("Error formatting {}: {}", file, e);
            std::process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    println!("Marina Code Formatter");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: {} [options] <file.prg>...", program);
    println!();
    println!("Options:");
    println!("  --check        Check if files are formatted without modifying them");
    println!("  --help, -h     Show this help message");
    println!();
    println!("Examples:");
    println!("  {} program.prg              Format program.prg", program);
    println!("  {} --check program.prg      Check if program.prg is formatted", program);
    println!("  {} examples/*.prg           Format all .prg files in examples/", program);
}

fn format_file(filename: &str, check_only: bool) -> Result<(), String> {
    let source = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Parse the file to ensure it's valid
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let _program = parser.parse()?;
    
    let formatted = formatter::format_source(&source, formatter::FormatOptions::default());

    // Normalize before comparing to avoid \r\n differences.
    let original_norm = source.replace("\r\n", "\n").replace('\r', "\n");

    if check_only {
        if formatted != original_norm {
            return Err("needs formatting".to_string());
        }
        println!("✓ {} is formatted", filename);
        return Ok(());
    }

    if formatted != original_norm {
        fs::write(filename, formatted).map_err(|e| format!("Failed to write file: {}", e))?;
        println!("Formatted {}", filename);
    } else {
        println!("Already formatted {}", filename);
    }
    
    Ok(())
}
