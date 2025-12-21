use marina::{formatter, run, Lexer, Parser};
#[cfg(windows)]
fn enable_vt_mode() {
    use windows::Win32::System::Console::{GetConsoleMode, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING};
    use windows::Win32::System::Console::GetStdHandle;
    use windows::Win32::System::Console::STD_OUTPUT_HANDLE;
    use windows::core::Result as WinResult;
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle.0 == 0 {
            return;
        }
        let mut mode = 0u32;
        if GetConsoleMode(handle, &mut mode).is_err() {
            return;
        }
        if mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0 {
            let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            let _ = SetConsoleMode(handle, new_mode);
        }
    }
}
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
        #[cfg(windows)]
        enable_vt_mode();
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Clipper Compiler & VM");
        println!("Usage: {} [options] <file.prg>", args[0]);
        println!("\nOptions:");
        println!("  -d, --disassemble   Show disassembled bytecode");
        println!("  -t, --tokens        Show tokens");
        println!("  -a, --ast           Show AST");
        println!("  repl                Start REPL mode");
        println!("  fmt                 Format .prg files (see: {} fmt --help)", args[0]);
        std::process::exit(1);
    }
    
    if args[1] == "repl" {
        run_repl();
        return;
    }

    if args[1] == "fmt" {
        run_fmt(&args);
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

fn run_fmt(args: &[String]) {
    let program = args.first().map(|s| s.as_str()).unwrap_or("clipper");

    if args.len() < 3 {
        print_fmt_usage(program);
        std::process::exit(1);
    }

    let mut check_only = false;
    let mut files: Vec<String> = Vec::new();

    for arg in &args[2..] {
        match arg.as_str() {
            "--check" => check_only = true,
            "--help" | "-h" => {
                print_fmt_usage(program);
                return;
            }
            file if file.ends_with(".prg") => files.push(file.to_string()),
            _ => {
                eprintln!("Error: Unknown option or invalid file: {}", arg);
                print_fmt_usage(program);
                std::process::exit(1);
            }
        }
    }

    if files.is_empty() {
        eprintln!("Error: No .prg files specified");
        print_fmt_usage(program);
        std::process::exit(1);
    }

    for file in files {
        if let Err(e) = format_file(&file, check_only) {
            eprintln!("Error formatting {}: {}", file, e);
            std::process::exit(1);
        }
    }
}

fn print_fmt_usage(program: &str) {
    println!("Clipper Formatter (clipper fmt)");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: {} fmt [options] <file.prg>...", program);
    println!();
    println!("Options:");
    println!("  --check        Check if files are formatted without modifying them");
    println!("  --help, -h     Show this help message");
    println!();
    println!("Examples:");
    println!("  {} fmt program.prg", program);
    println!("  {} fmt --check program.prg", program);
    println!("  {} fmt examples/*.prg", program);
}

fn format_file(filename: &str, check_only: bool) -> Result<(), String> {
    let source = fs::read_to_string(filename).map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse the file first so formatting can't hide syntax errors.
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
