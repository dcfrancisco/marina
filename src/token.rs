/// Token types for the Clipper language lexer
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number,
    String,
    True,
    False,
    Nil,
    
    // Identifiers and keywords
    Identifier,
    
    // Keywords
    Function,
    Procedure,
    Return,
    Local,
    Static,
    Private,
    Public,
    If,
    Else,
    ElseIf,
    EndIf,
    Do,
    While,
    EndDo,
    For,
    To,
    Step,
    Next,
    Exit,
    Loop,
    Case,
    EndCase,
    Otherwise,
    
    // Database keywords (Clipper-specific)
    Use,
    Select,
    DbCreate,
    DbAppend,
    DbSkip,
    DbGoTop,
    DbGoBottom,
    DbSeek,
    Index,
    Close,
    Replace,
    Delete,
    Recall,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Assign,
    PlusAssign,
    MinusAssign,
    Increment,
    Decrement,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Arrow,
    
    // Special
    QuestionMark,  // ? (print shorthand in Clipper)
    Eof,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}
