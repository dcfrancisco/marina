use crate::bytecode::*;
use std::collections::HashMap;
use std::io::Write;

pub struct VM {
    stack: Vec<Value>,
    globals: HashMap<usize, Value>,
    locals: Vec<Value>,  // Top-level locals (not in functions)
    call_frames: Vec<CallFrame>,
    ip: usize,
    cursor_row: usize,
    cursor_col: usize,
    saved_row: usize,
    saved_col: usize,
}

#[derive(Debug, Clone)]
struct CallFrame {
    return_ip: usize,
    stack_start: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            globals: HashMap::new(),
            locals: Vec::new(),
            call_frames: Vec::new(),
            ip: 0,
            cursor_row: 0,
            cursor_col: 0,
            saved_row: 0,
            saved_col: 0,
        }
    }
    
    pub fn run(&mut self, chunk: &Chunk) -> Result<(), String> {
        self.ip = 0;
        
        while self.ip < chunk.code.len() {
            let instruction = &chunk.code[self.ip];
            
            // Debug output
            // println!("IP: {}, Stack: {:?}", self.ip, self.stack);
            
            match &instruction.opcode {
                OpCode::Push => {
                    let idx = instruction.operand.ok_or("PUSH requires constant index")?;
                    let value = chunk.constants[idx].clone();
                    self.stack.push(value);
                    self.ip += 1;
                }
                
                OpCode::Pop => {
                    self.stack.pop().ok_or("Stack underflow")?;
                    self.ip += 1;
                }
                
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(self.add_values(a, b)?);
                    self.ip += 1;
                }
                
                OpCode::Subtract => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Number(a - b));
                    self.ip += 1;
                }
                
                OpCode::Multiply => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Number(a * b));
                    self.ip += 1;
                }
                
                OpCode::Divide => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    if b == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    self.stack.push(Value::Number(a / b));
                    self.ip += 1;
                }
                
                OpCode::Modulo => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Number(a % b));
                    self.ip += 1;
                }
                
                OpCode::Power => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Number(a.powf(b)));
                    self.ip += 1;
                }
                
                OpCode::Negate => {
                    let value = self.pop_number()?;
                    self.stack.push(Value::Number(-value));
                    self.ip += 1;
                }
                
                OpCode::Equal => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(Value::Boolean(self.values_equal(&a, &b)));
                    self.ip += 1;
                }
                
                OpCode::NotEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(Value::Boolean(!self.values_equal(&a, &b)));
                    self.ip += 1;
                }
                
                OpCode::Greater => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Boolean(a > b));
                    self.ip += 1;
                }
                
                OpCode::GreaterEqual => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Boolean(a >= b));
                    self.ip += 1;
                }
                
                OpCode::Less => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Boolean(a < b));
                    self.ip += 1;
                }
                
                OpCode::LessEqual => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Boolean(a <= b));
                    self.ip += 1;
                }
                
                OpCode::And => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(Value::Boolean(a.is_truthy() && b.is_truthy()));
                    self.ip += 1;
                }
                
                OpCode::Or => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(Value::Boolean(a.is_truthy() || b.is_truthy()));
                    self.ip += 1;
                }
                
                OpCode::Not => {
                    let value = self.pop()?;
                    self.stack.push(Value::Boolean(!value.is_truthy()));
                    self.ip += 1;
                }
                
                OpCode::GetLocal => {
                    let idx = instruction.operand.ok_or("GET_LOCAL requires index")?;
                    
                    if self.call_frames.is_empty() {
                        // Top-level locals - use separate locals storage
                        let value = self.locals.get(idx)
                            .ok_or(format!("Local variable {} not found", idx))?
                            .clone();
                        self.stack.push(value);
                    } else {
                        let stack_start = self.call_frames.last().unwrap().stack_start;
                        let value = self.stack.get(stack_start + idx)
                            .ok_or(format!("Local variable {} not found", idx))?
                            .clone();
                        self.stack.push(value);
                    }
                    self.ip += 1;
                }
                
                OpCode::SetLocal => {
                    let idx = instruction.operand.ok_or("SET_LOCAL requires index")?;
                    let value = self.stack.last().ok_or("Stack underflow")?.clone();
                    
                    if self.call_frames.is_empty() {
                        // Top-level locals - use separate locals storage
                        while self.locals.len() <= idx {
                            self.locals.push(Value::Nil);
                        }
                        self.locals[idx] = value;
                    } else {
                        let stack_start = self.call_frames.last().unwrap().stack_start;
                        while self.stack.len() <= stack_start + idx {
                            self.stack.push(Value::Nil);
                        }
                        self.stack[stack_start + idx] = value;
                    }
                    self.ip += 1;
                }
                
                OpCode::GetGlobal => {
                    let idx = instruction.operand.ok_or("GET_GLOBAL requires index")?;
                    let value = self.globals.get(&idx)
                        .ok_or(format!("Global variable {} not defined", idx))?
                        .clone();
                    self.stack.push(value);
                    self.ip += 1;
                }
                
                OpCode::SetGlobal => {
                    let idx = instruction.operand.ok_or("SET_GLOBAL requires index")?;
                    let value = self.stack.last().ok_or("Stack underflow")?.clone();
                    self.globals.insert(idx, value);
                    self.ip += 1;
                }
                
                OpCode::Jump => {
                    let target = instruction.operand.ok_or("JUMP requires target")?;
                    self.ip = target;
                }
                
                OpCode::JumpIfFalse => {
                    let target = instruction.operand.ok_or("JUMP_IF_FALSE requires target")?;
                    let condition = self.stack.last().ok_or("Stack underflow")?;
                    if !condition.is_truthy() {
                        self.ip = target;
                    } else {
                        self.ip += 1;
                    }
                }
                
                OpCode::JumpIfTrue => {
                    let target = instruction.operand.ok_or("JUMP_IF_TRUE requires target")?;
                    let condition = self.stack.last().ok_or("Stack underflow")?;
                    if condition.is_truthy() {
                        self.ip = target;
                    } else {
                        self.ip += 1;
                    }
                }
                
                OpCode::Call => {
                    let arity = instruction.operand.ok_or("CALL requires arity")?;
                    
                    // Get function name from stack
                    let func_value = self.stack.get(self.stack.len() - arity - 1)
                        .ok_or("Stack underflow getting function")?;
                    
                    // Handle built-in functions
                    if let Value::String(func_name) = func_value {
                        match func_name.as_str() {
                            "SetPos" | "DevPos" => {
                                if arity != 2 {
                                    return Err(format!("{} requires 2 arguments (row, col)", func_name));
                                }
                                let col = self.pop_number()? as usize;
                                let row = self.pop_number()? as usize;
                                let _func = self.pop()?; // Pop function name
                                
                                // Store position and emit ANSI escape sequence
                                // ANSI: ESC[row;colH (1-based indexing)
                                self.cursor_row = row;
                                self.cursor_col = col;
                                print!("\x1B[{};{}H", row + 1, col + 1);
                                std::io::stdout().flush().unwrap();
                                self.stack.push(Value::Nil); // Return nil
                            }
                            "OutStd" => {
                                if arity != 1 {
                                    return Err("OutStd requires 1 argument (string)".to_string());
                                }
                                let text = self.pop()?;
                                let _func = self.pop()?; // Pop function name
                                
                                // Output text at current cursor position (no prefix)
                                let text_str = match text {
                                    Value::String(s) => s,
                                    Value::Number(n) => n.to_string(),
                                    Value::Boolean(b) => b.to_string().to_uppercase(),
                                    Value::Nil => "NIL".to_string(),
                                    Value::Array(arr) => format!("{:?}", arr),
                                    Value::Function { .. } => "<function>".to_string(),
                                };
                                print!("{}", text_str);
                                std::io::stdout().flush().unwrap();
                                self.stack.push(Value::Nil); // Return nil
                            }
                            "ClearScreen" => {
                                if arity != 0 {
                                    return Err("ClearScreen requires 0 arguments".to_string());
                                }
                                let _func = self.pop()?; // Pop function name
                                
                                // ANSI: ESC[2J (clear screen) + ESC[H (home cursor)
                                print!("\x1B[2J\x1B[H");
                                std::io::stdout().flush().unwrap();
                                self.cursor_row = 0;
                                self.cursor_col = 0;
                                self.stack.push(Value::Nil);
                            }
                            "SavePos" => {
                                if arity != 0 {
                                    return Err("SavePos requires 0 arguments".to_string());
                                }
                                let _func = self.pop()?; // Pop function name
                                
                                self.saved_row = self.cursor_row;
                                self.saved_col = self.cursor_col;
                                // ANSI: ESC[s (save cursor position)
                                print!("\x1B[s");
                                std::io::stdout().flush().unwrap();
                                self.stack.push(Value::Nil);
                            }
                            "RestorePos" => {
                                if arity != 0 {
                                    return Err("RestorePos requires 0 arguments".to_string());
                                }
                                let _func = self.pop()?; // Pop function name
                                
                                self.cursor_row = self.saved_row;
                                self.cursor_col = self.saved_col;
                                // ANSI: ESC[u (restore cursor position)
                                print!("\x1B[u");
                                std::io::stdout().flush().unwrap();
                                self.stack.push(Value::Nil);
                            }
                            "GotoXY" => {
                                // Alias for SetPos with col, row order (X, Y)
                                if arity != 2 {
                                    return Err("GotoXY requires 2 arguments (col, row)".to_string());
                                }
                                let row = self.pop_number()? as usize;
                                let col = self.pop_number()? as usize;
                                let _func = self.pop()?; // Pop function name
                                
                                self.cursor_row = row;
                                self.cursor_col = col;
                                print!("\x1B[{};{}H", row + 1, col + 1);
                                std::io::stdout().flush().unwrap();
                                self.stack.push(Value::Nil);
                            }
                            "Replicate" => {
                                if arity != 2 {
                                    return Err("Replicate requires 2 arguments (string, count)".to_string());
                                }
                                let count = self.pop_number()? as usize;
                                let text = self.pop()?;
                                let _func = self.pop()?; // Pop function name
                                
                                let text_str = match text {
                                    Value::String(s) => s,
                                    Value::Number(n) => n.to_string(),
                                    _ => return Err("Replicate requires a string or number".to_string()),
                                };
                                
                                let result = text_str.repeat(count);
                                self.stack.push(Value::String(result));
                            }
                            _ => {
                                return Err(format!("Unknown function: {}", func_name));
                            }
                        }
                    } else {
                        return Err("Function calls not yet fully implemented".to_string());
                    }
                    
                    self.ip += 1;
                }
                
                OpCode::Return => {
                    if let Some(frame) = self.call_frames.pop() {
                        let return_value = self.pop()?;
                        self.stack.truncate(frame.stack_start);
                        self.stack.push(return_value);
                        self.ip = frame.return_ip;
                    } else {
                        // Return from main - halt
                        break;
                    }
                }
                
                OpCode::MakeArray => {
                    let size = instruction.operand.ok_or("MAKE_ARRAY requires size")?;
                    let mut elements = Vec::new();
                    for _ in 0..size {
                        elements.insert(0, self.pop()?);
                    }
                    self.stack.push(Value::Array(elements));
                    self.ip += 1;
                }
                
                OpCode::GetIndex => {
                    let index = self.pop_number()? as usize;
                    let array = self.pop()?;
                    
                    match array {
                        Value::Array(arr) => {
                            let value = arr.get(index)
                                .ok_or(format!("Index {} out of bounds", index))?
                                .clone();
                            self.stack.push(value);
                        }
                        Value::String(s) => {
                            let ch = s.chars().nth(index)
                                .ok_or(format!("Index {} out of bounds", index))?;
                            self.stack.push(Value::String(ch.to_string()));
                        }
                        _ => return Err("Cannot index non-array value".to_string()),
                    }
                    self.ip += 1;
                }
                
                OpCode::SetIndex => {
                    let value = self.pop()?;
                    let index = self.pop_number()? as usize;
                    let mut array = self.pop()?;
                    
                    if let Value::Array(ref mut arr) = array {
                        if index < arr.len() {
                            arr[index] = value.clone();
                        } else {
                            return Err(format!("Index {} out of bounds", index));
                        }
                        self.stack.push(array);
                    } else {
                        return Err("Cannot index non-array value".to_string());
                    }
                    self.ip += 1;
                }
                
                OpCode::Print => {
                    let value = self.pop()?;
                    print!("{}", value.to_string());
                    std::io::stdout().flush().unwrap();
                    self.ip += 1;
                }
                
                OpCode::DbUse | OpCode::DbSkip | OpCode::DbGoTop | 
                OpCode::DbGoBottom | OpCode::DbSeek | OpCode::DbReplace => {
                    // Database operations - stub for now
                    println!("Database operation: {:?}", instruction.opcode);
                    self.ip += 1;
                }
                
                OpCode::Halt => {
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow".to_string())
    }
    
    fn pop_number(&mut self) -> Result<f64, String> {
        match self.pop()? {
            Value::Number(n) => Ok(n),
            _ => Err("Expected number".to_string()),
        }
    }
    
    fn add_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
            (Value::String(x), Value::String(y)) => Ok(Value::String(format!("{}{}", x, y))),
            (Value::String(x), Value::Number(y)) => Ok(Value::String(format!("{}{}", x, y))),
            (Value::Number(x), Value::String(y)) => Ok(Value::String(format!("{}{}", x, y))),
            _ => Err("Cannot add these types".to_string()),
        }
    }
    
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}
