use crate::ast::*;
use crate::bytecode::*;
use super::Compiler;

impl Compiler {
    pub(crate) fn compile_statement(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::VarDecl { name, initializer, scope } => {
                if let Some(init) = initializer {
                    self.compile_expression(init)?;
                } else {
                    let nil_idx = self.chunk.add_constant(Value::Nil);
                    self.chunk.write(OpCode::Push, Some(nil_idx));
                }
                
                match scope {
                    VarScope::Local => {
                        // Check if variable already exists in locals
                        if !self.locals.contains(name) {
                            self.locals.push(name.clone());
                        }
                        let local_idx = self.locals.iter().position(|l| l == name).unwrap();
                        self.chunk.write(OpCode::SetLocal, Some(local_idx));
                        self.chunk.write(OpCode::Pop, None); // Pop the assigned value
                    }
                    _ => {
                        // Global, Static, Private, Public all treated as globals for now
                        let global_idx = self.get_or_create_global(name);
                        self.chunk.write(OpCode::SetGlobal, Some(global_idx));
                        self.chunk.write(OpCode::Pop, None); // Pop the assigned value
                    }
                }
            }
            
            Stmt::Expression(expr) => {
                self.compile_expression(expr)?;
                // Don't pop if it's a print statement (it already consumes its values)
                match expr {
                    Expr::Call { name, .. } if name == "?" || name.to_uppercase() == "PRINT" => {
                        // Print already pops, don't pop again
                    }
                    _ => {
                        self.chunk.write(OpCode::Pop, None);
                    }
                }
            }
            
            Stmt::Function { name, params, body, .. } => {
                // Jump over function body
                let jump_idx = self.chunk.code.len();
                self.chunk.write(OpCode::Jump, Some(0)); // Placeholder
                
                // Store function start address in function table
                let func_start = self.chunk.code.len();
                self.functions.insert(name.clone(), func_start);
                
                // Save and clear locals for function scope
                let saved_locals = self.locals.clone();
                self.locals.clear();
                
                // Parameters become local variables (pushed in reverse by caller)
                for param in params {
                    self.locals.push(param.clone());
                }
                
                // Compile function body
                self.scope_depth += 1;
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                // Implicit return nil
                let nil_idx = self.chunk.add_constant(Value::Nil);
                self.chunk.write(OpCode::Push, Some(nil_idx));
                self.chunk.write(OpCode::Return, None);
                
                // Patch jump
                let end_address = self.chunk.code.len();
                self.chunk.code[jump_idx].operand = Some(end_address);
                
                self.scope_depth -= 1;
                self.locals = saved_locals;
            }
            
            Stmt::Return(expr) => {
                if let Some(value) = expr {
                    self.compile_expression(value)?;
                } else {
                    let nil_idx = self.chunk.add_constant(Value::Nil);
                    self.chunk.write(OpCode::Push, Some(nil_idx));
                }
                self.chunk.write(OpCode::Return, None);
            }
            
            Stmt::If { condition, then_branch, else_branch } => {
                self.compile_expression(condition)?;
                
                let then_jump = self.chunk.code.len();
                self.chunk.write(OpCode::JumpIfFalse, Some(0)); // Placeholder
                
                for stmt in then_branch {
                    self.compile_statement(stmt)?;
                }
                
                if let Some(else_stmts) = else_branch {
                    let else_jump = self.chunk.code.len();
                    self.chunk.write(OpCode::Jump, Some(0)); // Placeholder
                    
                    // Patch then jump to else
                    let else_start = self.chunk.code.len();
                    self.chunk.code[then_jump].operand = Some(else_start);
                    
                    for stmt in else_stmts {
                        self.compile_statement(stmt)?;
                    }
                    
                    // Patch else jump to end
                    let end_address = self.chunk.code.len();
                    self.chunk.code[else_jump].operand = Some(end_address);
                } else {
                    // Patch then jump to end
                    let end_address = self.chunk.code.len();
                    self.chunk.code[then_jump].operand = Some(end_address);
                }
            }
            
            Stmt::While { condition, body } => {
                let loop_start = self.chunk.code.len();
                
                self.compile_expression(condition)?;
                
                let exit_jump = self.chunk.code.len();
                self.chunk.write(OpCode::JumpIfFalse, Some(0)); // Placeholder
                
                // Push loop context for EXIT handling
                self.loop_stack.push(super::LoopContext { break_jumps: Vec::new() });
                
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                self.chunk.write(OpCode::Jump, Some(loop_start));
                
                // Patch exit jump
                let end_address = self.chunk.code.len();
                self.chunk.code[exit_jump].operand = Some(end_address);
                
                // Patch all EXIT jumps to end of loop
                if let Some(loop_ctx) = self.loop_stack.pop() {
                    for break_pos in loop_ctx.break_jumps {
                        self.chunk.code[break_pos].operand = Some(end_address);
                    }
                }
            }
            
            Stmt::For { variable, start, end, step, body } => {
                // Initialize loop variable
                self.compile_expression(start)?;
                self.locals.push(variable.clone());
                let var_idx = self.locals.len() - 1;
                self.chunk.write(OpCode::SetLocal, Some(var_idx));
                
                let loop_start = self.chunk.code.len();
                
                // Check condition
                self.chunk.write(OpCode::GetLocal, Some(var_idx));
                self.compile_expression(end)?;
                self.chunk.write(OpCode::LessEqual, None);
                
                let exit_jump = self.chunk.code.len();
                self.chunk.write(OpCode::JumpIfFalse, Some(0)); // Placeholder
                
                // Push loop context
                self.loop_stack.push(super::LoopContext { break_jumps: Vec::new() });
                
                // Execute body
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                // Increment variable
                self.chunk.write(OpCode::GetLocal, Some(var_idx));
                if let Some(step_expr) = step {
                    self.compile_expression(step_expr)?;
                } else {
                    let one_idx = self.chunk.add_constant(Value::Number(1.0));
                    self.chunk.write(OpCode::Push, Some(one_idx));
                }
                self.chunk.write(OpCode::Add, None);
                self.chunk.write(OpCode::SetLocal, Some(var_idx));
                
                self.chunk.write(OpCode::Jump, Some(loop_start));
                
                // Patch exit jump
                let end_address = self.chunk.code.len();
                self.chunk.code[exit_jump].operand = Some(end_address);
                
                // Patch all EXIT jumps
                if let Some(loop_ctx) = self.loop_stack.pop() {
                    for break_pos in loop_ctx.break_jumps {
                        self.chunk.code[break_pos].operand = Some(end_address);
                    }
                }
                
                self.locals.pop();
            }
            
            Stmt::DoWhile { body, condition } => {
                let loop_start = self.chunk.code.len();
                
                // Push loop context
                self.loop_stack.push(super::LoopContext { break_jumps: Vec::new() });
                
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                self.compile_expression(condition)?;
                self.chunk.write(OpCode::JumpIfTrue, Some(loop_start));
                
                // Patch EXIT jumps
                let end_address = self.chunk.code.len();
                if let Some(loop_ctx) = self.loop_stack.pop() {
                    for break_pos in loop_ctx.break_jumps {
                        self.chunk.code[break_pos].operand = Some(end_address);
                    }
                }
            }
            
            Stmt::Loop { body } => {
                let loop_start = self.chunk.code.len();
                
                // Push loop context
                self.loop_stack.push(super::LoopContext { break_jumps: Vec::new() });
                
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                self.chunk.write(OpCode::Jump, Some(loop_start));
                
                // Patch EXIT jumps to end of loop
                let end_address = self.chunk.code.len();
                if let Some(loop_ctx) = self.loop_stack.pop() {
                    for break_pos in loop_ctx.break_jumps {
                        self.chunk.code[break_pos].operand = Some(end_address);
                    }
                }
            }
            
            Stmt::Exit => {
                // Emit jump to be patched at end of loop
                if let Some(loop_ctx) = self.loop_stack.last_mut() {
                    let jump_pos = self.chunk.code.len();
                    self.chunk.write(OpCode::Jump, Some(0)); // Placeholder
                    loop_ctx.break_jumps.push(jump_pos);
                } else {
                    return Err("EXIT statement outside of loop".to_string());
                }
            }
            
            Stmt::Case { expr, cases, otherwise } => {
                // Compile the expression to match against
                self.compile_expression(expr)?;
                
                let mut end_jumps: Vec<usize> = Vec::new();
                let mut next_case_jumps: Vec<usize> = Vec::new();
                
                // Compile each case
                for (value_expr, statements) in cases {
                    // Patch previous case's jump to next case
                    for jump_pos in &next_case_jumps {
                        self.chunk.code[*jump_pos].operand = Some(self.chunk.code.len());
                    }
                    next_case_jumps.clear();
                    
                    // Duplicate the match expression on stack
                    self.chunk.write(OpCode::Dup, None);
                    
                    // Compile the case value
                    self.compile_expression(value_expr)?;
                    
                    // Compare
                    self.chunk.write(OpCode::Equal, None);
                    
                    // Jump to next case if not equal
                    let jump_to_next = self.chunk.code.len();
                    self.chunk.write(OpCode::JumpIfFalse, Some(0)); // Placeholder
                    next_case_jumps.push(jump_to_next);
                    
                    // If we matched, pop the original expression before executing statements
                    self.chunk.write(OpCode::Pop, None);
                    
                    // Execute case statements
                    for stmt in statements {
                        self.compile_statement(stmt)?;
                    }
                    
                    // Jump to end after executing case (skip popping again)
                    let jump_to_end = self.chunk.code.len();
                    self.chunk.write(OpCode::Jump, Some(0)); // Placeholder
                    end_jumps.push(jump_to_end);
                }
                
                // Patch jumps to otherwise/end
                for jump_pos in &next_case_jumps {
                    self.chunk.code[*jump_pos].operand = Some(self.chunk.code.len());
                }
                
                // If we reach here (no case matched), pop the original expression
                if !next_case_jumps.is_empty() {
                    self.chunk.write(OpCode::Pop, None);
                }
                
                // Compile otherwise clause
                if let Some(otherwise_stmts) = otherwise {
                    for stmt in otherwise_stmts {
                        self.compile_statement(stmt)?;
                    }
                }
                
                // Patch all end jumps (from matched cases)
                let end_address = self.chunk.code.len();
                for jump_pos in end_jumps {
                    self.chunk.code[jump_pos].operand = Some(end_address);
                }
            }
            
            Stmt::DbUse { filename, .. } => {
                let filename_idx = self.chunk.add_constant(Value::String(filename.clone()));
                self.chunk.write(OpCode::Push, Some(filename_idx));
                self.chunk.write(OpCode::DbUse, None);
            }
            
            Stmt::DbSkip(count) => {
                if let Some(expr) = count {
                    self.compile_expression(expr)?;
                } else {
                    let one_idx = self.chunk.add_constant(Value::Number(1.0));
                    self.chunk.write(OpCode::Push, Some(one_idx));
                }
                self.chunk.write(OpCode::DbSkip, None);
            }
            
            Stmt::DbGoTop => {
                self.chunk.write(OpCode::DbGoTop, None);
            }
            
            Stmt::DbGoBottom => {
                self.chunk.write(OpCode::DbGoBottom, None);
            }
            
            Stmt::DbSeek { key } => {
                self.compile_expression(key)?;
                self.chunk.write(OpCode::DbSeek, None);
            }
            
            Stmt::Replace { field, value } => {
                let field_idx = self.chunk.add_constant(Value::String(field.clone()));
                self.chunk.write(OpCode::Push, Some(field_idx));
                self.compile_expression(value)?;
                self.chunk.write(OpCode::DbReplace, None);
            }
            
            Stmt::Block(statements) => {
                for stmt in statements {
                    self.compile_statement(stmt)?;
                }
            }
        }
        
        Ok(())
    }
}
