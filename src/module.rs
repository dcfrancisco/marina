use crate::bytecode::Value;
use std::collections::HashMap;

/// A Marina module containing exported functions and constants
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub functions: HashMap<String, NativeFunction>,
}

/// Native function type - implemented in Rust
pub type NativeFunction = fn(&[Value]) -> Result<Value, String>;

impl Module {
    pub fn new(name: &str) -> Self {
        Module {
            name: name.to_string(),
            functions: HashMap::new(),
        }
    }
    
    pub fn register_function(&mut self, name: &str, func: NativeFunction) {
        self.functions.insert(name.to_string(), func);
    }
    
    pub fn get_function(&self, name: &str) -> Option<NativeFunction> {
        self.functions.get(name).copied()
    }
}

/// Built-in module loader
pub struct ModuleLoader {
    modules: HashMap<String, Module>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        let mut loader = ModuleLoader {
            modules: HashMap::new(),
        };
        
        // Register built-in modules
        loader.register_console_module();
        loader.register_tui_module();
        loader.register_math_module();
        loader.register_str_module();
        
        loader
    }
    
    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }
    
    fn register_console_module(&mut self) {
        let mut module = Module::new("console");
        
        module.register_function("print", console_print);
        module.register_function("setPos", console_set_pos);
        module.register_function("setColor", console_set_color);
        module.register_function("resetColor", console_reset_color);
        module.register_function("hideCursor", console_hide_cursor);
        module.register_function("showCursor", console_show_cursor);
        
        self.modules.insert("console".to_string(), module);
    }
    
    fn register_tui_module(&mut self) {
        let mut module = Module::new("tui");
        
        module.register_function("box", tui_box);
        module.register_function("readInt", tui_read_int);
        
        self.modules.insert("tui".to_string(), module);
    }
    
    fn register_math_module(&mut self) {
        let mut module = Module::new("math");
        
        module.register_function("max", math_max);
        module.register_function("min", math_min);
        module.register_function("int", math_int);
        module.register_function("abs", math_abs);
        
        self.modules.insert("math".to_string(), module);
    }
    
    fn register_str_module(&mut self) {
        let mut module = Module::new("str");
        
        module.register_function("repeat", str_repeat);
        module.register_function("len", str_len);
        
        self.modules.insert("str".to_string(), module);
    }
}

// ==================== Console Module Functions ====================

fn console_print(args: &[Value]) -> Result<Value, String> {
    for arg in args {
        print!("{}", arg.to_string());
    }
    Ok(Value::Nil)
}

fn console_set_pos(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("setPos requires 2 arguments (row, col)".to_string());
    }
    
    let row = match &args[0] {
        Value::Number(n) => *n as u16,
        _ => return Err("Row must be a number".to_string()),
    };
    
    let col = match &args[1] {
        Value::Number(n) => *n as u16,
        _ => return Err("Column must be a number".to_string()),
    };
    
    print!("\x1b[{};{}H", row, col);
    Ok(Value::Nil)
}

fn console_set_color(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("setColor requires a color code".to_string());
    }
    
    let color = match &args[0] {
        Value::Number(n) => *n as u8,
        _ => return Err("Color must be a number".to_string()),
    };
    
    // Map Clipper-style color codes to ANSI
    let ansi_code = match color {
        0 => 30,  // Black
        1 => 34,  // Blue
        2 => 32,  // Green
        3 => 36,  // Cyan
        4 => 31,  // Red
        5 => 35,  // Magenta
        6 => 33,  // Brown/Yellow
        7 => 37,  // White
        8 => 90,  // Gray
        9 => 94,  // Light Blue
        10 => 92, // Light Green
        11 => 96, // Light Cyan
        12 => 91, // Light Red
        13 => 95, // Light Magenta
        14 => 93, // Yellow
        15 => 97, // Bright White
        _ => 37,  // Default to white
    };
    
    print!("\x1b[{}m", ansi_code);
    Ok(Value::Nil)
}

fn console_reset_color(_args: &[Value]) -> Result<Value, String> {
    print!("\x1b[0m");
    Ok(Value::Nil)
}

fn console_hide_cursor(_args: &[Value]) -> Result<Value, String> {
    print!("\x1b[?25l");
    Ok(Value::Nil)
}

fn console_show_cursor(_args: &[Value]) -> Result<Value, String> {
    print!("\x1b[?25h");
    Ok(Value::Nil)
}

// ==================== TUI Module Functions ====================

fn tui_box(args: &[Value]) -> Result<Value, String> {
    if args.len() < 4 {
        return Err("box requires at least 4 arguments (row, col, height, width)".to_string());
    }
    
    let row = match &args[0] {
        Value::Number(n) => *n as u16,
        _ => return Err("Row must be a number".to_string()),
    };
    
    let col = match &args[1] {
        Value::Number(n) => *n as u16,
        _ => return Err("Column must be a number".to_string()),
    };
    
    let height = match &args[2] {
        Value::Number(n) => *n as u16,
        _ => return Err("Height must be a number".to_string()),
    };
    
    let width = match &args[3] {
        Value::Number(n) => *n as u16,
        _ => return Err("Width must be a number".to_string()),
    };
    
    let title = if args.len() > 4 {
        args[4].to_string()
    } else {
        String::new()
    };
    
    // Draw box using box-drawing characters
    // Top border
    print!("\x1b[{};{}H┌", row, col);
    for _ in 0..width - 2 {
        print!("─");
    }
    print!("┐");
    
    // Title if provided
    if !title.is_empty() {
        let title_pos = col + (width / 2) - (title.len() as u16 / 2);
        print!("\x1b[{};{}H {}", row, title_pos, title);
    }
    
    // Sides
    for i in 1..height - 1 {
        print!("\x1b[{};{}H│", row + i, col);
        print!("\x1b[{};{}H│", row + i, col + width - 1);
    }
    
    // Bottom border
    print!("\x1b[{};{}H└", row + height - 1, col);
    for _ in 0..width - 2 {
        print!("─");
    }
    print!("┘");
    
    Ok(Value::Nil)
}

fn tui_read_int(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("readInt requires min and max arguments".to_string());
    }
    
    let min = match &args[0] {
        Value::Number(n) => *n as i32,
        _ => return Err("Min must be a number".to_string()),
    };
    
    let max = match &args[1] {
        Value::Number(n) => *n as i32,
        _ => return Err("Max must be a number".to_string()),
    };
    
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;
        
        match input.trim().parse::<i32>() {
            Ok(num) if num >= min && num <= max => {
                return Ok(Value::Number(num as f64));
            }
            _ => {
                println!("Please enter a number between {} and {}", min, max);
            }
        }
    }
}

// ==================== Math Module Functions ====================

fn math_max(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("max requires at least 2 arguments".to_string());
    }
    
    let mut max_val = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err("Arguments must be numbers".to_string()),
    };
    
    for arg in &args[1..] {
        if let Value::Number(n) = arg {
            if *n > max_val {
                max_val = *n;
            }
        } else {
            return Err("Arguments must be numbers".to_string());
        }
    }
    
    Ok(Value::Number(max_val))
}

fn math_min(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("min requires at least 2 arguments".to_string());
    }
    
    let mut min_val = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err("Arguments must be numbers".to_string()),
    };
    
    for arg in &args[1..] {
        if let Value::Number(n) = arg {
            if *n < min_val {
                min_val = *n;
            }
        } else {
            return Err("Arguments must be numbers".to_string());
        }
    }
    
    Ok(Value::Number(min_val))
}

fn math_int(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("int requires a number argument".to_string());
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.trunc())),
        _ => Err("Argument must be a number".to_string()),
    }
}

fn math_abs(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("abs requires a number argument".to_string());
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err("Argument must be a number".to_string()),
    }
}

// ==================== String Module Functions ====================

fn str_repeat(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("repeat requires a string and count".to_string());
    }
    
    let s = args[0].to_string();
    let count = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err("Count must be a number".to_string()),
    };
    
    Ok(Value::String(s.repeat(count)))
}

fn str_len(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("len requires a string argument".to_string());
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        Value::Array(a) => Ok(Value::Number(a.len() as f64)),
        _ => Err("Argument must be a string or array".to_string()),
    }
}
