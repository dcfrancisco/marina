/// Bytecode instructions for the Clipper VM

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // Stack operations
    Push,           // Push constant onto stack
    Pop,            // Pop value from stack
    Dup,            // Duplicate top of stack
    
    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Negate,
    
    // Comparison operations
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    // Logical operations
    And,
    Or,
    Not,
    
    // Variable operations
    GetLocal,       // Get local variable
    SetLocal,       // Set local variable
    GetGlobal,      // Get global variable
    SetGlobal,      // Set global variable
    
    // Control flow
    Jump,           // Unconditional jump
    JumpIfFalse,    // Jump if top of stack is false
    JumpIfTrue,     // Jump if top of stack is true
    Call,           // Call function
    Return,         // Return from function
    
    // Array operations
    MakeArray,      // Create array from stack values
    GetIndex,       // Get array element
    SetIndex,       // Set array element
    
    // Database operations
    DbUse,
    DbSkip,
    DbGoTop,
    DbGoBottom,
    DbSeek,
    DbReplace,
    
    // Module operations
    Import,         // Load module into VM module table
    CallModule,     // Call a module function
    
    // Built-in functions
    Print,          // Print value
    
    // Special
    Halt,           // Stop execution
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Array(Vec<Value>),
    Function {
        name: String,
        arity: usize,
        address: usize,
    },
    ModuleFunction {
        module: String,
        function: String,
    },
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Function { .. } => true,
            Value::ModuleFunction { .. } => true,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
            Value::Nil => "NIL".to_string(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("{{{}}}", elements.join(", "))
            }
            Value::Function { name, arity, .. } => {
                format!("<function {}({})>", name, arity)
            }
            Value::ModuleFunction { module, function } => {
                format!("<module function {}.{}>", module, function)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub operand: Option<usize>,
}

impl Instruction {
    pub fn new(opcode: OpCode, operand: Option<usize>) -> Self {
        Instruction { opcode, operand }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<Instruction>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }
    
    pub fn write(&mut self, opcode: OpCode, operand: Option<usize>) {
        self.code.push(Instruction::new(opcode, operand));
    }
    
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
    
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        for (offset, instruction) in self.code.iter().enumerate() {
            self.disassemble_instruction(offset, instruction);
        }
    }
    
    fn disassemble_instruction(&self, offset: usize, instruction: &Instruction) {
        print!("{:04} ", offset);
        
        match &instruction.opcode {
            OpCode::Push => {
                if let Some(constant_idx) = instruction.operand {
                    println!("PUSH           {} '{}'", 
                        constant_idx, 
                        self.constants[constant_idx].to_string());
                } else {
                    println!("PUSH");
                }
            }
            OpCode::Pop => println!("POP"),
            OpCode::Dup => println!("DUP"),
            OpCode::Add => println!("ADD"),
            OpCode::Subtract => println!("SUBTRACT"),
            OpCode::Multiply => println!("MULTIPLY"),
            OpCode::Divide => println!("DIVIDE"),
            OpCode::Modulo => println!("MODULO"),
            OpCode::Power => println!("POWER"),
            OpCode::Negate => println!("NEGATE"),
            OpCode::Equal => println!("EQUAL"),
            OpCode::NotEqual => println!("NOT_EQUAL"),
            OpCode::Greater => println!("GREATER"),
            OpCode::GreaterEqual => println!("GREATER_EQUAL"),
            OpCode::Less => println!("LESS"),
            OpCode::LessEqual => println!("LESS_EQUAL"),
            OpCode::And => println!("AND"),
            OpCode::Or => println!("OR"),
            OpCode::Not => println!("NOT"),
            OpCode::GetLocal => println!("GET_LOCAL      {}", instruction.operand.unwrap_or(0)),
            OpCode::SetLocal => println!("SET_LOCAL      {}", instruction.operand.unwrap_or(0)),
            OpCode::GetGlobal => println!("GET_GLOBAL     {}", instruction.operand.unwrap_or(0)),
            OpCode::SetGlobal => println!("SET_GLOBAL     {}", instruction.operand.unwrap_or(0)),
            OpCode::Jump => println!("JUMP           {}", instruction.operand.unwrap_or(0)),
            OpCode::JumpIfFalse => println!("JUMP_IF_FALSE  {}", instruction.operand.unwrap_or(0)),
            OpCode::JumpIfTrue => println!("JUMP_IF_TRUE   {}", instruction.operand.unwrap_or(0)),
            OpCode::Call => println!("CALL           {}", instruction.operand.unwrap_or(0)),
            OpCode::Return => println!("RETURN"),
            OpCode::MakeArray => println!("MAKE_ARRAY     {}", instruction.operand.unwrap_or(0)),
            OpCode::GetIndex => println!("GET_INDEX"),
            OpCode::SetIndex => println!("SET_INDEX"),
            OpCode::DbUse => println!("DB_USE"),
            OpCode::DbSkip => println!("DB_SKIP"),
            OpCode::DbGoTop => println!("DB_GO_TOP"),
            OpCode::DbGoBottom => println!("DB_GO_BOTTOM"),
            OpCode::DbSeek => println!("DB_SEEK"),
            OpCode::DbReplace => println!("DB_REPLACE"),
            OpCode::Import => println!("IMPORT         {}", instruction.operand.unwrap_or(0)),
            OpCode::CallModule => println!("CALL_MODULE    {}", instruction.operand.unwrap_or(0)),
            OpCode::Print => println!("PRINT"),
            OpCode::Halt => println!("HALT"),
        }
    }
}
