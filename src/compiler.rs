use crate::ast::*;
use crate::bytecode::*;
use std::collections::HashMap;

pub struct Compiler {
    chunk: Chunk,
    locals: Vec<String>,
    scope_depth: usize,
    globals: HashMap<String, usize>,
    functions: HashMap<String, usize>, // function name -> bytecode address
    loop_stack: Vec<LoopContext>, // Track loop contexts for EXIT/BREAK
}

#[derive(Debug, Clone)]
struct LoopContext {
    break_jumps: Vec<usize>, // Positions that need patching to loop end
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
            globals: HashMap::new(),
            functions: HashMap::new(),
            loop_stack: Vec::new(),
        }
    }
    
    pub fn compile(mut self, program: Program) -> Result<(Chunk, HashMap<String, usize>), String> {
        // First pass: scan for function definitions and reserve addresses
        // We'll insert placeholder jumps and patch them later
        let mut func_placeholders = HashMap::new();
        
        for (idx, stmt) in program.statements.iter().enumerate() {
            if let Stmt::Function { name, .. } = stmt {
                // Reserve a placeholder - we'll update this with actual address later
                func_placeholders.insert(name.clone(), idx);
            }
        }
        
        // Second pass: compile statements
        for stmt in program.statements {
            self.compile_statement(&stmt)?;
        }
        
        self.chunk.write(OpCode::Halt, None);
        Ok((self.chunk, self.functions))
    }
    
    fn compile_statement(&mut self, stmt: &Stmt) -> Result<(), String> {
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
                self.loop_stack.push(LoopContext { break_jumps: Vec::new() });
                
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
                self.loop_stack.push(LoopContext { break_jumps: Vec::new() });
                
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
                self.loop_stack.push(LoopContext { break_jumps: Vec::new() });
                
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
                self.loop_stack.push(LoopContext { break_jumps: Vec::new() });
                
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
                    self.chunk.write(OpCode::GetLocal, Some(0)); // Temp hack - need DUP opcode
                    // Better: compile expr again for now
                    self.compile_expression(expr)?;
                    
                    // Compile the case value
                    self.compile_expression(value_expr)?;
                    
                    // Compare
                    self.chunk.write(OpCode::Equal, None);
                    
                    // Jump to next case if not equal
                    let jump_to_next = self.chunk.code.len();
                    self.chunk.write(OpCode::JumpIfFalse, Some(0)); // Placeholder
                    next_case_jumps.push(jump_to_next);
                    
                    // Pop the comparison result
                    self.chunk.write(OpCode::Pop, None);
                    
                    // Execute case statements
                    for stmt in statements {
                        self.compile_statement(stmt)?;
                    }
                    
                    // Jump to end after executing case
                    let jump_to_end = self.chunk.code.len();
                    self.chunk.write(OpCode::Jump, Some(0)); // Placeholder
                    end_jumps.push(jump_to_end);
                }
                
                // Patch jumps to otherwise/end
                for jump_pos in &next_case_jumps {
                    self.chunk.code[*jump_pos].operand = Some(self.chunk.code.len());
                }
                
                // Pop comparison result before otherwise
                if !next_case_jumps.is_empty() {
                    self.chunk.write(OpCode::Pop, None);
                }
                
                // Compile otherwise clause
                if let Some(otherwise_stmts) = otherwise {
                    for stmt in otherwise_stmts {
                        self.compile_statement(stmt)?;
                    }
                }
                
                // Pop the original match expression
                self.chunk.write(OpCode::Pop, None);
                
                // Patch all end jumps
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
    
    fn compile_expression(&mut self, expr: &Expr) -> Result<(), String> {
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
                if name.to_uppercase() == "PRINT" || name == "?" {
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
                    // Print newline at end
                    let newline_idx = self.chunk.add_constant(Value::String("\n".to_string()));
                    self.chunk.write(OpCode::Push, Some(newline_idx));
                    self.chunk.write(OpCode::Print, None);
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
    
    fn resolve_local(&self, name: &str) -> Option<usize> {
        self.locals.iter().rposition(|l| l == name)
    }
    
    fn get_or_create_global(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.globals.get(name) {
            idx
        } else {
            let idx = self.globals.len();
            self.globals.insert(name.to_string(), idx);
            idx
        }
    }
}
