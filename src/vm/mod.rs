use crate::bytecode::*;
use std::collections::HashMap;

mod opcodes;
mod stack;

pub struct VM {
    stack: Vec<Value>,
    pub(crate) globals: HashMap<usize, Value>,
    pub(crate) locals: Vec<Value>,  // Top-level locals (not in functions)
    pub(crate) call_frames: Vec<CallFrame>,
    pub(crate) functions: HashMap<String, usize>,  // Function name -> address
    pub(crate) ip: usize,
    pub(crate) cursor_row: usize,
    pub(crate) cursor_col: usize,
    pub(crate) saved_row: usize,
    pub(crate) saved_col: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct CallFrame {
    pub return_ip: usize,
    pub locals_start: usize,  // Where this frame's locals start
    pub locals_count: usize,  // How many locals this frame has
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            globals: HashMap::new(),
            locals: Vec::new(),
            call_frames: Vec::new(),
            functions: HashMap::new(),
            ip: 0,
            cursor_row: 0,
            cursor_col: 0,
            saved_row: 0,
            saved_col: 0,
        }
    }
    
    pub fn run(&mut self, chunk: &Chunk, functions: HashMap<String, usize>) -> Result<(), String> {
        self.ip = 0;
        self.functions = functions.clone();
        
        // Check if Main() or main() procedure exists
        let main_addr = functions.get("Main")
            .or_else(|| functions.get("main"))
            .copied();
        
        if let Some(addr) = main_addr {
            // If Main() exists, execute top-level code first (which includes global initialization
            // and will jump over the Main procedure body), then call Main()
            while self.ip < chunk.code.len() {
                let instruction = &chunk.code[self.ip];
                
                // Stop when we hit HALT (end of top-level code)
                if matches!(instruction.opcode, OpCode::Halt) {
                    break;
                }
                
                self.execute_instruction(chunk, instruction)?;
            }
            
            // Now call Main() procedure
            self.call_frames.push(CallFrame {
                return_ip: usize::MAX, // Signal to stop after Main() returns
                locals_start: self.locals.len(),
                locals_count: 0,
            });
            
            self.ip = addr;
            
            // Execute Main() - loop will exit when Main() returns (ip set to usize::MAX)
            while self.ip < chunk.code.len() {
                let instruction = &chunk.code[self.ip];
                self.execute_instruction(chunk, instruction)?;
            }
        } else {
            // No Main(), just execute all top-level code
            while self.ip < chunk.code.len() {
                let instruction = &chunk.code[self.ip];
                self.execute_instruction(chunk, instruction)?;
            }
        }
        
        Ok(())
    }
    
    // Stack operations (re-exported from stack module)
    pub(crate) fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow".to_string())
    }
    
    pub(crate) fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    
    pub(crate) fn pop_number(&mut self) -> Result<f64, String> {
        match self.pop()? {
            Value::Number(n) => Ok(n),
            _ => Err("Expected number".to_string()),
        }
    }
    
    pub(crate) fn add_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
            (Value::String(x), Value::String(y)) => Ok(Value::String(format!("{}{}", x, y))),
            (Value::String(x), Value::Number(y)) => Ok(Value::String(format!("{}{}", x, y))),
            (Value::Number(x), Value::String(y)) => Ok(Value::String(format!("{}{}", x, y))),
            _ => Err("Cannot add these types".to_string()),
        }
    }
    
    pub(crate) fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}
