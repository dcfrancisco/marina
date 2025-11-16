use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        
        // Keywords
        keywords.insert("FUNCTION".to_string(), TokenType::Function);
        keywords.insert("PROCEDURE".to_string(), TokenType::Procedure);
        keywords.insert("RETURN".to_string(), TokenType::Return);
        keywords.insert("LOCAL".to_string(), TokenType::Local);
        keywords.insert("STATIC".to_string(), TokenType::Static);
        keywords.insert("PRIVATE".to_string(), TokenType::Private);
        keywords.insert("PUBLIC".to_string(), TokenType::Public);
        keywords.insert("IF".to_string(), TokenType::If);
        keywords.insert("ELSE".to_string(), TokenType::Else);
        keywords.insert("ELSEIF".to_string(), TokenType::ElseIf);
        keywords.insert("ENDIF".to_string(), TokenType::EndIf);
        keywords.insert("DO".to_string(), TokenType::Do);
        keywords.insert("WHILE".to_string(), TokenType::While);
        keywords.insert("ENDDO".to_string(), TokenType::EndDo);
        keywords.insert("FOR".to_string(), TokenType::For);
        keywords.insert("TO".to_string(), TokenType::To);
        keywords.insert("STEP".to_string(), TokenType::Step);
        keywords.insert("NEXT".to_string(), TokenType::Next);
        keywords.insert("EXIT".to_string(), TokenType::Exit);
        keywords.insert("LOOP".to_string(), TokenType::Loop);
        keywords.insert("CASE".to_string(), TokenType::Case);
        keywords.insert("ENDCASE".to_string(), TokenType::EndCase);
        keywords.insert("OTHERWISE".to_string(), TokenType::Otherwise);
        keywords.insert("TRUE".to_string(), TokenType::True);
        keywords.insert("FALSE".to_string(), TokenType::False);
        keywords.insert("NIL".to_string(), TokenType::Nil);
        
        // Database keywords
        keywords.insert("USE".to_string(), TokenType::Use);
        keywords.insert("SELECT".to_string(), TokenType::Select);
        keywords.insert("DBCREATE".to_string(), TokenType::DbCreate);
        keywords.insert("DBAPPEND".to_string(), TokenType::DbAppend);
        keywords.insert("DBSKIP".to_string(), TokenType::DbSkip);
        keywords.insert("DBGOTOP".to_string(), TokenType::DbGoTop);
        keywords.insert("DBGOBOTTOM".to_string(), TokenType::DbGoBottom);
        keywords.insert("DBSEEK".to_string(), TokenType::DbSeek);
        keywords.insert("INDEX".to_string(), TokenType::Index);
        keywords.insert("CLOSE".to_string(), TokenType::Close);
        keywords.insert("REPLACE".to_string(), TokenType::Replace);
        keywords.insert("DELETE".to_string(), TokenType::Delete);
        keywords.insert("RECALL".to_string(), TokenType::Recall);
        
        // Logical operators
        keywords.insert("AND".to_string(), TokenType::And);
        keywords.insert("OR".to_string(), TokenType::Or);
        keywords.insert("NOT".to_string(), TokenType::Not);
        
        Lexer {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
            keywords,
        }
    }
    
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.is_at_end() {
                break;
            }
            
            let token = self.scan_token()?;
            if token.token_type != TokenType::Newline {
                tokens.push(token);
            }
        }
        
        tokens.push(Token::new(TokenType::Eof, String::new(), self.line, self.column));
        Ok(tokens)
    }
    
    fn scan_token(&mut self) -> Result<Token, String> {
        let start_column = self.column;
        let c = self.advance();
        
        let token_type = match c {
            '+' => {
                if self.match_char('+') {
                    TokenType::Increment
                } else if self.match_char('=') {
                    TokenType::PlusAssign
                } else {
                    TokenType::Plus
                }
            }
            '-' => {
                if self.match_char('-') {
                    TokenType::Decrement
                } else if self.match_char('=') {
                    TokenType::MinusAssign
                } else if self.match_char('>') {
                    TokenType::Arrow
                } else {
                    TokenType::Minus
                }
            }
            '*' => {
                if self.match_char('=') {
                    TokenType::MultiplyAssign
                } else {
                    TokenType::Star
                }
            }
            '/' => {
                if self.match_char('=') {
                    TokenType::DivideAssign
                } else {
                    TokenType::Slash
                }
            }
            '%' => TokenType::Percent,
            '^' => TokenType::Power,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            ';' => TokenType::Semicolon,
            ':' => {
                if self.match_char('=') {
                    TokenType::Assign
                } else {
                    TokenType::Colon
                }
            }
            '=' => {
                if self.match_char('=') {
                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            }
            '!' => {
                if self.match_char('=') {
                    TokenType::NotEqual
                } else {
                    return Err(format!("Unexpected character '!' at line {}, column {}", self.line, start_column));
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenType::LessEqual
                } else if self.match_char('>') {
                    TokenType::NotEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '?' => TokenType::QuestionMark,
            '"' | '\'' => return self.scan_string(c),
            '\n' => {
                self.line += 1;
                self.column = 1;
                TokenType::Newline
            }
            _ => {
                if c.is_ascii_digit() {
                    return self.scan_number(c);
                } else if c.is_alphabetic() || c == '_' {
                    return self.scan_identifier(c);
                } else {
                    return Err(format!("Unexpected character '{}' at line {}, column {}", c, self.line, start_column));
                }
            }
        };
        
        Ok(Token::new(token_type, c.to_string(), self.line, start_column))
    }
    
    fn scan_string(&mut self, quote: char) -> Result<Token, String> {
        let start_line = self.line;
        let start_column = self.column - 1;
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != quote {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            value.push(self.advance());
        }
        
        if self.is_at_end() {
            return Err(format!("Unterminated string at line {}, column {}", start_line, start_column));
        }
        
        self.advance(); // Closing quote
        Ok(Token::new(TokenType::String, value, start_line, start_column))
    }
    
    fn scan_number(&mut self, first: char) -> Result<Token, String> {
        let start_column = self.column - 1;
        let mut value = String::from(first);
        
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            value.push(self.advance());
        }
        
        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            value.push(self.advance()); // '.'
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }
        
        Ok(Token::new(TokenType::Number, value, self.line, start_column))
    }
    
    fn scan_identifier(&mut self, first: char) -> Result<Token, String> {
        let start_column = self.column - 1;
        let mut value = String::from(first);
        
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            value.push(self.advance());
        }
        
        let upper_value = value.to_uppercase();
        let token_type = self.keywords.get(&upper_value)
            .cloned()
            .unwrap_or(TokenType::Identifier);
        
        Ok(Token::new(token_type, value, self.line, start_column))
    }
    
    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        // Single-line comment
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else if self.peek_next() == '*' {
                        // Multi-line comment
                        self.advance(); // '/'
                        self.advance(); // '*'
                        while !self.is_at_end() {
                            if self.peek() == '*' && self.peek_next() == '/' {
                                self.advance(); // '*'
                                self.advance(); // '/'
                                break;
                            }
                            if self.peek() == '\n' {
                                self.line += 1;
                                self.column = 0;
                            }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }
    
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }
    
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
