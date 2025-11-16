use crate::ast::*;
use crate::bytecode::*;
use std::collections::HashMap;

mod statements;
mod expressions;

pub struct Compiler {
    chunk: Chunk,
    locals: Vec<String>,
    scope_depth: usize,
    globals: HashMap<String, usize>,
    functions: HashMap<String, usize>, // function name -> bytecode address
    loop_stack: Vec<LoopContext>, // Track loop contexts for EXIT/BREAK
}

#[derive(Debug, Clone)]
pub(crate) struct LoopContext {
    pub break_jumps: Vec<usize>, // Positions that need patching to loop end
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
    
    pub(crate) fn resolve_local(&self, name: &str) -> Option<usize> {
        self.locals.iter().rposition(|l| l == name)
    }
    
    pub(crate) fn get_or_create_global(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.globals.get(name) {
            idx
        } else {
            let idx = self.globals.len();
            self.globals.insert(name.to_string(), idx);
            idx
        }
    }
}
