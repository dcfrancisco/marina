use crate::ast::*;
use crate::bytecode::*;
use super::Compiler;

impl Compiler {
    pub(crate) fn compile_expression(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Number(n) => {
                let idx = self.chunk.add_constant(Value::Number(*n));
                self.chunk.write(OpCode::Push, Some(idx));
            }
            
            Expr::String(s) => {
                let idx = self.chunk.add_constant(Value::String(s.clone()));
                self.chunk.write(OpCode::Push, Some(idx));
            }
            
            Expr::Boolean(b) => {
                let idx = self.chunk.add_constant(Value::Boolean(*b));
                self.chunk.write(OpCode::Push, Some(idx));
            }
            
            Expr::Nil => {
                let idx = self.chunk.add_constant(Value::Nil);
                self.chunk.write(OpCode::Push, Some(idx));
            }
            
            Expr::Variable(name) => {
                if let Some(local_idx) = self.resolve_local(name) {
                    self.chunk.write(OpCode::GetLocal, Some(local_idx));
                } else {
                    let global_idx = self.get_or_create_global(name);
                    self.chunk.write(OpCode::GetGlobal, Some(global_idx));
                }
            }
            
            Expr::Binary { left, operator, right } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                
                match operator {
                    BinaryOp::Add => self.chunk.write(OpCode::Add, None),
                    BinaryOp::Subtract => self.chunk.write(OpCode::Subtract, None),
                    BinaryOp::Multiply => self.chunk.write(OpCode::Multiply, None),
                    BinaryOp::Divide => self.chunk.write(OpCode::Divide, None),
                    BinaryOp::Modulo => self.chunk.write(OpCode::Modulo, None),
                    BinaryOp::Power => self.chunk.write(OpCode::Power, None),
                    BinaryOp::Equal => self.chunk.write(OpCode::Equal, None),
                    BinaryOp::NotEqual => self.chunk.write(OpCode::NotEqual, None),
                    BinaryOp::Less => self.chunk.write(OpCode::Less, None),
                    BinaryOp::Greater => self.chunk.write(OpCode::Greater, None),
                    BinaryOp::LessEqual => self.chunk.write(OpCode::LessEqual, None),
                    BinaryOp::GreaterEqual => self.chunk.write(OpCode::GreaterEqual, None),
                    BinaryOp::And => self.chunk.write(OpCode::And, None),
                    BinaryOp::Or => self.chunk.write(OpCode::Or, None),
                }
            }
            
            Expr::Unary { operator, operand } => {
                self.compile_expression(operand)?;
                
                match operator {
                    UnaryOp::Negate => self.chunk.write(OpCode::Negate, None),
                    UnaryOp::Not => self.chunk.write(OpCode::Not, None),
                }
            }
            
            Expr::Call { name, args } => {
                // Handle indexed assignment (__SET_INDEX__)
                if name == "__SET_INDEX__" {
                    if args.len() != 3 {
                        return Err("Internal error: __SET_INDEX__ requires 3 args".to_string());
                    }
                    
                    // For indexed assignment arr[idx] := value, we need to:
                    // 1. Load the array
                    // 2. Push index
                    // 3. Push value
                    // 4. Call SetIndex (which modifies and returns the array)
                    // 5. Store the modified array back to the variable
                    // Note: Don't pop - let the statement handler do that (for expression semantics)
                    
                    // Check if args[0] is a simple variable
                    if let Expr::Variable(var_name) = &args[0] {
                        // Load array
                        self.compile_expression(&args[0])?;
                        // Push index and value
                        self.compile_expression(&args[1])?;
                        self.compile_expression(&args[2])?;
                        // SetIndex pops value, index, array and pushes modified array
                        self.chunk.write(OpCode::SetIndex, None);
                        // Store modified array back to variable
                        if let Some(local_idx) = self.resolve_local(var_name) {
                            self.chunk.write(OpCode::SetLocal, Some(local_idx));
                        } else {
                            let global_idx = self.get_or_create_global(var_name);
                            self.chunk.write(OpCode::SetGlobal, Some(global_idx));
                        }
                        // Leave value on stack for expression semantics (statement handler will pop)
                    } else {
                        // Complex expression - just do the operation (may not persist)
                        self.compile_expression(&args[0])?;
                        self.compile_expression(&args[1])?;
                        self.compile_expression(&args[2])?;
                        self.chunk.write(OpCode::SetIndex, None);
                        // Leave the modified array on stack
                    }
                    return Ok(());
                }
                
                // Special built-in functions
                if name.to_uppercase() == "PRINT" || name == "?" || name == "??" {
                    let with_newline = name.to_uppercase() == "PRINT" || name == "?";
                    
                    for (idx, arg) in args.iter().enumerate() {
                        self.compile_expression(arg)?;
                        self.chunk.write(OpCode::Print, None);
                        
                        // Add space between arguments (except after last)
                        if idx < args.len() - 1 {
                            let space_idx = self.chunk.add_constant(Value::String(" ".to_string()));
                            self.chunk.write(OpCode::Push, Some(space_idx));
                            self.chunk.write(OpCode::Print, None);
                        }
                    }
                    
                    // Print newline at end only if requested
                    if with_newline {
                        let newline_idx = self.chunk.add_constant(Value::String("\n".to_string()));
                        self.chunk.write(OpCode::Push, Some(newline_idx));
                        self.chunk.write(OpCode::Print, None);
                    }
                } else {
                    // All functions (user-defined and built-in) use name-based calling
                    // Push function name as string
                    let name_idx = self.chunk.add_constant(Value::String(name.clone()));
                    self.chunk.write(OpCode::Push, Some(name_idx));
                    
                    // Push all arguments
                    for arg in args {
                        self.compile_expression(arg)?;
                    }
                    self.chunk.write(OpCode::Call, Some(args.len()));
                }
            }
            
            Expr::Assign { name, value } => {
                self.compile_expression(value)?;
                
                if let Some(local_idx) = self.resolve_local(name) {
                    self.chunk.write(OpCode::SetLocal, Some(local_idx));
                } else {
                    let global_idx = self.get_or_create_global(name);
                    self.chunk.write(OpCode::SetGlobal, Some(global_idx));
                }
            }
            
            Expr::Array(elements) => {
                for elem in elements {
                    self.compile_expression(elem)?;
                }
                self.chunk.write(OpCode::MakeArray, Some(elements.len()));
            }
            
            Expr::Index { object, index } => {
                self.compile_expression(object)?;
                self.compile_expression(index)?;
                self.chunk.write(OpCode::GetIndex, None);
            }
        }
        
        Ok(())
    }
}
