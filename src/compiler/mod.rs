use crate::ast::*;
use crate::bytecode::*;
use std::collections::{HashMap, HashSet};

mod expressions;
mod statements;

pub struct Compiler {
    chunk: Chunk,
    locals: Vec<String>,
    scope_depth: usize,
    globals: HashMap<String, usize>,
    functions: HashMap<String, usize>, // function name -> bytecode address
    loop_stack: Vec<LoopContext>,      // Track loop contexts for EXIT/BREAK
    imported_modules: HashSet<String>,
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
            imported_modules: HashSet::new(),
        }
    }

    pub fn compile(mut self, program: Program) -> Result<(Chunk, HashMap<String, usize>), String> {
        // First pass: scan for function definitions and reserve addresses
        // We'll insert placeholder jumps and patch them later
        let mut func_placeholders = HashMap::new();

        for (idx, stmt) in program.statements.iter().enumerate() {
            match stmt {
                Stmt::Function { name, .. } => {
                    // Reserve a placeholder - we'll update this with actual address later
                    func_placeholders.insert(name.clone(), idx);
                }
                Stmt::Import { module } => {
                    let module_name = module.to_ascii_lowercase();
                    if !self.is_supported_builtin_module(&module_name) {
                        return Err(format!(
                            "Unsupported module import '{}'. Supported modules: console, input, math, string, system",
                            module
                        ));
                    }
                    self.imported_modules.insert(module_name);
                }
                _ => {}
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

    pub(crate) fn is_supported_builtin_module(&self, module: &str) -> bool {
        matches!(module, "console" | "input" | "math" | "string" | "system")
    }

    pub(crate) fn validate_imported_call(&self, name: &str) -> Result<(), String> {
        let Some((module, function)) = name.split_once('.') else {
            return Ok(());
        };

        let module_name = module.to_ascii_lowercase();
        let function_name = function.to_ascii_lowercase();

        if !self.imported_modules.contains(&module_name) {
            return Err(format!(
                "Module '{}' must be imported before calling '{}'",
                module, name
            ));
        }

        let supported = match module_name.as_str() {
            "console" => matches!(
                function_name.as_str(),
                "clearscreen"
                    | "setpos"
                    | "devpos"
                    | "gotoxy"
                    | "outstd"
                    | "setcolor"
                    | "setcursor"
                    | "savepos"
                    | "restorepos"
            ),
            "input" => matches!(function_name.as_str(), "inkey" | "getinput" | "getsecret"),
            "math" => matches!(
                function_name.as_str(),
                "abs" | "sqrt" | "round" | "int" | "min" | "max" | "sin" | "cos" | "tan"
            ),
            "string" => matches!(
                function_name.as_str(),
                "replicate"
                    | "space"
                    | "len"
                    | "substr"
                    | "trim"
                    | "alltrim"
                    | "ltrim"
                    | "rtrim"
                    | "chr"
                    | "asc"
                    | "val"
                    | "str"
            ),
            "system" => matches!(function_name.as_str(), "sleep"),
            _ => false,
        };

        if supported {
            Ok(())
        } else {
            Err(format!(
                "Unsupported function '{}' for module '{}'",
                function, module
            ))
        }
    }
}
