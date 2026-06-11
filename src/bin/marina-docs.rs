use marina::docs::{self, DocsConfig};
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let command = args[1].as_str();
    if matches!(command, "--help" | "-h") {
        print_usage(&args[0]);
        return;
    }

    let input = PathBuf::from(&args[2]);
    let mut output = None;
    let mut title = None;
    let mut theme = "marina".to_string();

    let mut index = 3usize;
    while index < args.len() {
        match args[index].as_str() {
            "-o" | "--output" => {
                index += 1;
                let value = args.get(index).unwrap_or_else(|| {
                    eprintln!("Error: Missing path after --output");
                    std::process::exit(1);
                });
                output = Some(PathBuf::from(value));
            }
            "--title" => {
                index += 1;
                let value = args.get(index).unwrap_or_else(|| {
                    eprintln!("Error: Missing value after --title");
                    std::process::exit(1);
                });
                title = Some(value.clone());
            }
            "--theme" => {
                index += 1;
                let value = args.get(index).unwrap_or_else(|| {
                    eprintln!("Error: Missing value after --theme");
                    std::process::exit(1);
                });
                theme = value.clone();
            }
            "--help" | "-h" => {
                print_usage(&args[0]);
                return;
            }
            other => {
                eprintln!("Error: Unknown option '{}'", other);
                print_usage(&args[0]);
                std::process::exit(1);
            }
        }

        index += 1;
    }

    if !input.exists() {
        eprintln!("Error: Input path '{}' does not exist", input.display());
        std::process::exit(1);
    }

    let config = DocsConfig { title, theme };

    let result = match command {
        "html" => docs::render_html(&input, output.as_deref(), &config),
        "pdf" => docs::render_pdf(&input, output.as_deref(), &config),
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage(&args[0]);
            std::process::exit(1);
        }
    };

    match result {
        Ok(path) => println!("Created {}", path.display()),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    println!("Marina Documentation Tools");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: {} <html|pdf> <input> [options]", program);
    println!();
    println!("Options:");
    println!("  -o, --output <path>   Write output to a specific file");
    println!("  --title <title>       Override the generated document title");
    println!("  --theme <theme>       Theme name (marina, paper)");
    println!("  --help, -h            Show this help message");
    println!();
    println!("Examples:");
    println!("  {} pdf README.md", program);
    println!("  {} html README.md", program);
    println!("  {} pdf docs/", program);
    println!("  {} html docs/ --output docs/index.html", program);
}
