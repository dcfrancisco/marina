use crate::bytecode::*;
use super::{VM, CallFrame};
use std::io::Write;

impl VM {
    pub(crate) fn execute_instruction(&mut self, chunk: &Chunk, instruction: &Instruction) -> Result<(), String> {
        match &instruction.opcode {
            OpCode::Push => {
                let idx = instruction.operand.ok_or("PUSH requires constant index")?;
                let value = chunk.constants[idx].clone();
                self.push(value);
                self.ip += 1;
            }
            
            OpCode::Pop => {
                self.pop()?;
                self.ip += 1;
            }
            
            OpCode::Dup => {
                let value = self.stack.last().ok_or("Stack underflow")?.clone();
                self.push(value);
                self.ip += 1;
            }
            
            OpCode::Add => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(self.add_values(a, b)?);
                self.ip += 1;
            }
            
            OpCode::Subtract => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.push(Value::Number(a - b));
                self.ip += 1;
            }
            
            OpCode::Multiply => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.push(Value::Number(a * b));
                self.ip += 1;
            }
            
            OpCode::Divide => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                self.push(Value::Number(a / b));
                self.ip += 1;
            }
            
            OpCode::Modulo => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.push(Value::Number(a % b));
                self.ip += 1;
            }
            
            OpCode::Power => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.push(Value::Number(a.powf(b)));
                self.ip += 1;
            }
            
            OpCode::Negate => {
                let value = self.pop_number()?;
                self.push(Value::Number(-value));
                self.ip += 1;
            }
            
            OpCode::Equal => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(self.values_equal(&a, &b)));
                self.ip += 1;
            }
            
            OpCode::NotEqual => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(!self.values_equal(&a, &b)));
                self.ip += 1;
            }
            
            OpCode::Greater => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.push(Value::Boolean(a > b));
                self.ip += 1;
            }
            
            OpCode::GreaterEqual => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.push(Value::Boolean(a >= b));
                self.ip += 1;
            }
            
            OpCode::Less => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.push(Value::Boolean(a < b));
                self.ip += 1;
            }
            
            OpCode::LessEqual => {
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                self.push(Value::Boolean(a <= b));
                self.ip += 1;
            }
            
            OpCode::And => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(a.is_truthy() && b.is_truthy()));
                self.ip += 1;
            }
            
            OpCode::Or => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(a.is_truthy() || b.is_truthy()));
                self.ip += 1;
            }
            
            OpCode::Not => {
                let value = self.pop()?;
                self.push(Value::Boolean(!value.is_truthy()));
                self.ip += 1;
            }
            
            OpCode::GetLocal => self.execute_get_local(instruction)?,
            OpCode::SetLocal => self.execute_set_local(instruction)?,
            OpCode::GetGlobal => self.execute_get_global(instruction)?,
            OpCode::SetGlobal => self.execute_set_global(instruction)?,
            
            OpCode::Jump => {
                let target = instruction.operand.ok_or("JUMP requires target")?;
                self.ip = target;
            }
            
            OpCode::JumpIfFalse => {
                let target = instruction.operand.ok_or("JUMP_IF_FALSE requires target")?;
                let condition = self.pop()?;
                if !condition.is_truthy() {
                    self.ip = target;
                } else {
                    self.ip += 1;
                }
            }
            
            OpCode::JumpIfTrue => {
                let target = instruction.operand.ok_or("JUMP_IF_TRUE requires target")?;
                let condition = self.pop()?;
                if condition.is_truthy() {
                    self.ip = target;
                } else {
                    self.ip += 1;
                }
            }
            
            OpCode::Call => self.execute_call(instruction)?,
            OpCode::Return => self.execute_return()?,
            OpCode::MakeArray => self.execute_make_array(instruction)?,
            OpCode::GetIndex => self.execute_get_index()?,
            OpCode::SetIndex => self.execute_set_index()?,
            OpCode::Print => self.execute_print()?,
            
            OpCode::DbUse | OpCode::DbSkip | OpCode::DbGoTop | 
            OpCode::DbGoBottom | OpCode::DbSeek | OpCode::DbReplace => {
                // Database operations - stub for now
                println!("Database operation: {:?}", instruction.opcode);
                self.ip += 1;
            }
            
            OpCode::Halt => {
                // Set IP beyond code length to exit the run loop
                self.ip = usize::MAX;
            }
        }
        
        Ok(())
    }
    
    fn execute_get_local(&mut self, instruction: &Instruction) -> Result<(), String> {
        let idx = instruction.operand.ok_or("GET_LOCAL requires index")?;
        
        if self.call_frames.is_empty() {
            // Top-level locals - use separate locals storage
            let value = self.locals.get(idx)
                .ok_or(format!("Local variable {} not found", idx))?
                .clone();
            self.push(value);
        } else {
            // Function locals - use frame's locals
            let frame = self.call_frames.last().unwrap();
            let local_idx = frame.locals_start + idx;
            let value = self.locals.get(local_idx)
                .ok_or(format!("Local variable {} not found", idx))?
                .clone();
            self.push(value);
        }
        self.ip += 1;
        Ok(())
    }
    
    fn execute_set_local(&mut self, instruction: &Instruction) -> Result<(), String> {
        let idx = instruction.operand.ok_or("SET_LOCAL requires index")?;
        let value = self.peek(0).ok_or("Stack underflow")?.clone();
        
        if self.call_frames.is_empty() {
            // Top-level locals - use separate locals storage
            while self.locals.len() <= idx {
                self.locals.push(Value::Nil);
            }
            self.locals[idx] = value;
        } else {
            // Function locals - use frame's locals
            let frame = self.call_frames.last().unwrap();
            let local_idx = frame.locals_start + idx;
            while self.locals.len() <= local_idx {
                self.locals.push(Value::Nil);
            }
            self.locals[local_idx] = value;
        }
        self.ip += 1;
        Ok(())
    }
    
    fn execute_get_global(&mut self, instruction: &Instruction) -> Result<(), String> {
        let idx = instruction.operand.ok_or("GET_GLOBAL requires index")?;
        let value = self.globals.get(&idx)
            .ok_or(format!("Global variable {} not defined", idx))?
            .clone();
        self.push(value);
        self.ip += 1;
        Ok(())
    }
    
    fn execute_set_global(&mut self, instruction: &Instruction) -> Result<(), String> {
        let idx = instruction.operand.ok_or("SET_GLOBAL requires index")?;
        let value = self.peek(0).ok_or("Stack underflow")?.clone();
        self.globals.insert(idx, value);
        self.ip += 1;
        Ok(())
    }
    
    fn execute_call(&mut self, instruction: &Instruction) -> Result<(), String> {
        let arity = instruction.operand.ok_or("CALL requires arity")?;
        
        // Get function name from stack (always a String now)
        let stack_len = self.stack_len();
        let func_value = self.peek(arity)
            .ok_or("Stack underflow getting function")?;
        
        let func_name = match func_value {
            Value::String(name) => name.clone(),
            _ => return Err("Function name must be a string".to_string()),
        };
        
        // Check if it's a user-defined function
        if let Some(&func_addr) = self.functions.get(&func_name) {
            // Remove function name from stack
            let pos = stack_len - arity - 1;
            self.stack.remove(pos);
            
            // Create call frame
            let frame = CallFrame {
                return_ip: self.ip + 1,
                locals_start: self.locals.len(),
                locals_count: arity,
            };
            
            // Move arguments from stack to locals
            for _ in 0..arity {
                let arg = self.pop()?;
                self.locals.insert(self.locals.len(), arg);
            }
            // Reverse to match parameter order
            let start = self.locals.len() - arity;
            self.locals[start..].reverse();
            
            self.call_frames.push(frame);
            self.ip = func_addr;
            return Ok(()); // Don't increment IP
        }
        
        // Handle built-in functions
        self.execute_builtin_function(&func_name, arity)?;
        self.ip += 1;
        Ok(())
    }
    
    fn execute_builtin_function(&mut self, func_name: &str, arity: usize) -> Result<(), String> {
        match func_name {
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
                self.push(Value::Nil); // Return nil
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
                self.push(Value::Nil); // Return nil
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
                self.push(Value::Nil);
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
                self.push(Value::Nil);
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
                self.push(Value::Nil);
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
                self.push(Value::Nil);
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
                self.push(Value::String(result));
            }
            "Space" => {
                if arity != 1 {
                    return Err("Space requires 1 argument (count)".to_string());
                }
                let count = self.pop_number()? as usize;
                let _func = self.pop()?; // Pop function name
                
                let result = " ".repeat(count);
                self.push(Value::String(result));
            }
            "Len" => {
                if arity != 1 {
                    return Err("Len requires 1 argument (string)".to_string());
                }
                let text = self.pop()?;
                let _func = self.pop()?; // Pop function name
                
                let length = match text {
                    Value::String(s) => s.len(),
                    Value::Array(arr) => arr.len(),
                    _ => return Err("Len requires a string or array".to_string()),
                };
                
                self.push(Value::Number(length as f64));
            }
            "SubStr" => {
                if arity != 3 {
                    return Err("SubStr requires 3 arguments (string, start, length)".to_string());
                }
                let length = self.pop_number()? as usize;
                let start = self.pop_number()? as usize;
                let text = self.pop()?;
                let _func = self.pop()?; // Pop function name
                
                let text_str = match text {
                    Value::String(s) => s,
                    _ => return Err("SubStr requires a string".to_string()),
                };
                
                // Clipper uses 1-based indexing for SubStr
                let start_idx = if start > 0 { start - 1 } else { 0 };
                let result: String = text_str.chars()
                    .skip(start_idx)
                    .take(length)
                    .collect();
                
                self.push(Value::String(result));
            }
            "Trim" | "AllTrim" => {
                if arity != 1 {
                    return Err(format!("{} requires 1 argument (string)", func_name));
                }
                let text = self.pop()?;
                let _func = self.pop()?; // Pop function name
                
                let result = match text {
                    Value::String(s) => s.trim().to_string(),
                    _ => return Err("Trim requires a string".to_string()),
                };
                
                self.push(Value::String(result));
            }
            "RTrim" => {
                if arity != 1 {
                    return Err("RTrim requires 1 argument (string)".to_string());
                }
                let text = self.pop()?;
                let _func = self.pop()?; // Pop function name
                
                let result = match text {
                    Value::String(s) => s.trim_end().to_string(),
                    _ => return Err("RTrim requires a string".to_string()),
                };
                
                self.push(Value::String(result));
            }
            "LTrim" => {
                if arity != 1 {
                    return Err("LTrim requires 1 argument (string)".to_string());
                }
                let text = self.pop()?;
                let _func = self.pop()?; // Pop function name
                
                let result = match text {
                    Value::String(s) => s.trim_start().to_string(),
                    _ => return Err("LTrim requires a string".to_string()),
                };
                
                self.push(Value::String(result));
            }
            "Chr" => {
                if arity != 1 {
                    return Err("Chr requires 1 argument (ASCII code)".to_string());
                }
                let code = self.pop_number()? as u8;
                let _func = self.pop()?; // Pop function name
                
                let result = String::from(code as char);
                self.push(Value::String(result));
            }
            "Asc" => {
                if arity != 1 {
                    return Err("Asc requires 1 argument (string)".to_string());
                }
                let text = self.pop()?;
                let _func = self.pop()?; // Pop function name
                
                let code = match text {
                    Value::String(s) => {
                        s.chars().next().map(|c| c as u32 as f64).unwrap_or(0.0)
                    }
                    _ => return Err("Asc requires a string".to_string()),
                };
                
                self.push(Value::Number(code));
            }
            "Inkey" => {
                if arity < 0 || arity > 1 {
                    return Err("Inkey requires 0 or 1 argument (optional timeout)".to_string());
                }
                
                let _timeout = if arity == 1 {
                    self.pop_number()? as u64
                } else {
                    0
                };
                let _func = self.pop()?; // Pop function name
                
                use std::io::Read;
                
                // Read single character without echo
                let mut buffer = [0; 1];
                let result = std::io::stdin().read(&mut buffer);
                
                let key_code = match result {
                    Ok(1) => buffer[0] as f64,
                    _ => 0.0, // No key or error
                };
                
                self.push(Value::Number(key_code));
            }
            "Val" => {
                // VAL(cString) -> nNumber
                // Converts string to number, returns 0 if invalid
                if arity != 1 {
                    return Err("Val requires 1 argument".to_string());
                }
                
                let str_val = match self.pop()? {
                    Value::String(s) => s,
                    Value::Number(n) => {
                        self.pop()?; // Pop function name
                        self.push(Value::Number(n));
                        return Ok(());
                    }
                    _ => String::new(),
                };
                let _func = self.pop()?; // Pop function name
                
                // Try to parse the string as a number
                let number = str_val.trim().parse::<f64>().unwrap_or(0.0);
                self.push(Value::Number(number));
            }
            "GetInput" => {
                // GETINPUT(cDefault, [nRow], [nColumn], [lSay], [cPrompt]) -> cInput
                if arity < 1 || arity > 5 {
                    return Err("GetInput requires 1-5 arguments".to_string());
                }
                
                // Get optional parameters (in reverse order from stack)
                let prompt = if arity >= 5 {
                    match self.pop()? {
                        Value::String(s) => s,
                        _ => String::new(),
                    }
                } else {
                    String::new()
                };
                
                let _say_flag = if arity >= 4 {
                    match self.pop()? {
                        Value::Boolean(b) => b,
                        _ => false,
                    }
                } else {
                    false
                };
                
                let col = if arity >= 3 {
                    self.pop_number()? as usize
                } else {
                    self.cursor_col
                };
                
                let row = if arity >= 2 {
                    self.pop_number()? as usize
                } else {
                    self.cursor_row
                };
                
                let default = match self.pop()? {
                    Value::String(s) => s,
                    _ => String::new(),
                };
                
                let _func = self.pop()?; // Pop function name
                
                // Position cursor if row/col specified
                if arity >= 2 {
                    print!("\x1B[{};{}H", row + 1, col + 1);
                    std::io::stdout().flush().unwrap();
                }
                
                // Display prompt if provided
                if !prompt.is_empty() {
                    print!("{}", prompt);
                    std::io::stdout().flush().unwrap();
                }
                
                // Display default value
                print!("{}", default);
                std::io::stdout().flush().unwrap();
                
                // Move cursor back to start of input field
                if !default.is_empty() {
                    print!("\x1B[{}D", default.len());
                    std::io::stdout().flush().unwrap();
                }
                
                // Simple input (for now, just read a line)
                // TODO: Implement full editing with arrow keys, insert/overwrite modes, etc.
                use std::io::BufRead;
                let mut input = String::new();
                let stdin = std::io::stdin();
                let mut handle = stdin.lock();
                
                match handle.read_line(&mut input) {
                    Ok(_) => {
                        // Remove trailing newline
                        if input.ends_with('\n') {
                            input.pop();
                            if input.ends_with('\r') {
                                input.pop();
                            }
                        }
                        
                        // Pad or truncate to default length
                        let max_len = default.len();
                        if input.len() < max_len {
                            input.push_str(&" ".repeat(max_len - input.len()));
                        } else if input.len() > max_len {
                            input.truncate(max_len);
                        }
                        
                        self.push(Value::String(input));
                    }
                    Err(_) => {
                        // Return default on error
                        self.push(Value::String(default));
                    }
                }
            }
            _ => {
                return Err(format!("Unknown function: {}", func_name));
            }
        }
        Ok(())
    }
    
    fn execute_return(&mut self) -> Result<(), String> {
        if let Some(frame) = self.call_frames.pop() {
            let return_value = self.pop()?;
            
            // Remove this frame's locals
            self.locals.truncate(frame.locals_start);
            
            // Push return value onto stack
            self.push(return_value);
            self.ip = frame.return_ip;
        } else {
            // Return from main - halt (set ip beyond chunk length)
            self.ip = usize::MAX;
        }
        Ok(())
    }
    
    fn execute_make_array(&mut self, instruction: &Instruction) -> Result<(), String> {
        let size = instruction.operand.ok_or("MAKE_ARRAY requires size")?;
        let mut elements = Vec::new();
        for _ in 0..size {
            elements.insert(0, self.pop()?);
        }
        self.push(Value::Array(elements));
        self.ip += 1;
        Ok(())
    }
    
    fn execute_get_index(&mut self) -> Result<(), String> {
        let index = self.pop_number()? as usize;
        let array = self.pop()?;
        
        match array {
            Value::Array(arr) => {
                let value = arr.get(index)
                    .ok_or(format!("Index {} out of bounds", index))?
                    .clone();
                self.push(value);
            }
            Value::String(s) => {
                let ch = s.chars().nth(index)
                    .ok_or(format!("Index {} out of bounds", index))?;
                self.push(Value::String(ch.to_string()));
            }
            _ => return Err("Cannot index non-array value".to_string()),
        }
        self.ip += 1;
        Ok(())
    }
    
    fn execute_set_index(&mut self) -> Result<(), String> {
        let value = self.pop()?;
        let index = self.pop_number()? as usize;
        let mut array = self.pop()?;
        
        if let Value::Array(ref mut arr) = array {
            if index < arr.len() {
                arr[index] = value.clone();
            } else {
                return Err(format!("Index {} out of bounds", index));
            }
            self.push(array);
        } else {
            return Err("Cannot index non-array value".to_string());
        }
        self.ip += 1;
        Ok(())
    }
    
    fn execute_print(&mut self) -> Result<(), String> {
        let value = self.pop()?;
        print!("{}", value.to_string());
        std::io::stdout().flush().unwrap();
        self.ip += 1;
        Ok(())
    }
}
