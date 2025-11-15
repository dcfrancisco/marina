/// Abstract Syntax Tree nodes for Clipper language

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    
    // Variables and identifiers
    Variable(String),
    
    // Binary operations
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    
    // Unary operations
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },
    
    // Function call
    Call {
        name: String,
        args: Vec<Expr>,
    },
    
    // Array/field access
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    
    // Assignment
    Assign {
        name: String,
        value: Box<Expr>,
    },
    
    // Array literal
    Array(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    // Variable declaration
    VarDecl {
        name: String,
        initializer: Option<Expr>,
        scope: VarScope,
    },
    
    // Expression statement
    Expression(Expr),
    
    // Function/Procedure definition
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        is_procedure: bool,
    },
    
    // Return statement
    Return(Option<Expr>),
    
    // If statement
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    
    // While loop
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    
    // For loop
    For {
        variable: String,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        body: Vec<Stmt>,
    },
    
    // Do-While loop
    DoWhile {
        body: Vec<Stmt>,
        condition: Expr,
    },
    
    // Exit loop
    Exit,
    
    // Loop (infinite loop with Exit)
    Loop {
        body: Vec<Stmt>,
    },
    
    // Database operations
    DbUse {
        filename: String,
        alias: Option<String>,
    },
    
    DbSkip(Option<Expr>),
    DbGoTop,
    DbGoBottom,
    
    DbSeek {
        key: Expr,
    },
    
    Replace {
        field: String,
        value: Expr,
    },
    
    // Block of statements
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarScope {
    Local,
    Static,
    Private,
    Public,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}
