use marina::{lexer::Lexer, token::TokenType};

#[test]
fn test_basic_tokens() {
    let source = "LOCAL x := 10".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer should succeed");
    
    assert_eq!(tokens.len(), 5); // LOCAL, x, :=, 10, EOF
    assert_eq!(tokens[0].token_type, TokenType::Local);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].lexeme, "x");
    assert_eq!(tokens[2].token_type, TokenType::Assign);
    assert_eq!(tokens[3].token_type, TokenType::Number);
    assert_eq!(tokens[3].lexeme, "10");
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}

#[test]
fn test_operators() {
    let source = "+ - * / % ^ := ==".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer should succeed");
    
    assert_eq!(tokens[0].token_type, TokenType::Plus);
    assert_eq!(tokens[1].token_type, TokenType::Minus);
    assert_eq!(tokens[2].token_type, TokenType::Star);
    assert_eq!(tokens[3].token_type, TokenType::Slash);
    assert_eq!(tokens[4].token_type, TokenType::Percent);
    assert_eq!(tokens[5].token_type, TokenType::Power);
    assert_eq!(tokens[6].token_type, TokenType::Assign);
    assert_eq!(tokens[7].token_type, TokenType::Equal);
}

#[test]
fn test_keywords() {
    let source = "IF ELSE ENDIF WHILE ENDDO FOR TO NEXT FUNCTION RETURN".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer should succeed");
    
    assert_eq!(tokens[0].token_type, TokenType::If);
    assert_eq!(tokens[1].token_type, TokenType::Else);
    assert_eq!(tokens[2].token_type, TokenType::EndIf);
    assert_eq!(tokens[3].token_type, TokenType::While);
    assert_eq!(tokens[4].token_type, TokenType::EndDo);
    assert_eq!(tokens[5].token_type, TokenType::For);
    assert_eq!(tokens[6].token_type, TokenType::To);
    assert_eq!(tokens[7].token_type, TokenType::Next);
    assert_eq!(tokens[8].token_type, TokenType::Function);
    assert_eq!(tokens[9].token_type, TokenType::Return);
}

#[test]
fn test_string_literals() {
    let source = r#""Hello, World!""#.to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer should succeed");
    
    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].lexeme, "Hello, World!");
}

#[test]
fn test_numbers() {
    let source = "42 3.14 0.5".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer should succeed");
    
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].lexeme, "42");
    assert_eq!(tokens[1].token_type, TokenType::Number);
    assert_eq!(tokens[1].lexeme, "3.14");
    assert_eq!(tokens[2].token_type, TokenType::Number);
    assert_eq!(tokens[2].lexeme, "0.5");
}

#[test]
fn test_comments() {
    let source = "// This is a comment\nLOCAL x".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer should succeed");
    
    // Comment should be skipped
    assert_eq!(tokens[0].token_type, TokenType::Local);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
}

#[test]
fn test_multiline_comment() {
    let source = "/* Multi\nline\ncomment */LOCAL x".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer should succeed");
    
    assert_eq!(tokens[0].token_type, TokenType::Local);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
}
