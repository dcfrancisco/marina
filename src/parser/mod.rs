use crate::ast::*;
use crate::token::{Token, TokenType};

mod statements;
mod expressions;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut program = Program::new();
        
        while !self.is_at_end() {
            program.statements.push(self.declaration()?);
        }
        
        Ok(program)
    }
    
    // Utility methods
    pub(crate) fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    pub(crate) fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }
    
    pub(crate) fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    pub(crate) fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
    
    pub(crate) fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    pub(crate) fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    pub(crate) fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(format!(
                "{} at line {}, column {}",
                message,
                self.peek().line,
                self.peek().column
            ))
        }
    }
    
    pub(crate) fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        if self.check(&TokenType::Identifier) {
            Ok(self.advance().lexeme.clone())
        } else {
            Err(format!(
                "{} at line {}, column {}",
                message,
                self.peek().line,
                self.peek().column
            ))
        }
    }
    
    pub(crate) fn skip_newlines(&mut self) {
        while self.match_token(&[TokenType::Newline]) {
            // Skip all newlines
        }
    }
}
