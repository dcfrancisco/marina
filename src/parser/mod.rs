use crate::ast::*;
use crate::diagnostics::{Diagnostic, Span};
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

    pub fn parse_with_diagnostics(&mut self) -> ParseResult {
        let mut program = Program::new();
        let mut diagnostics = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => program.statements.push(stmt),
                Err(message) => {
                    let token = self.peek().clone();
                    let span = span_from_message_or_token(&message, &token);
                    diagnostics.push(Diagnostic::error(
                        message,
                        span,
                    ));
                    self.synchronize();
                }
            }
        }

        ParseResult { program, diagnostics }
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

    pub(crate) fn error_at(&self, token: &Token, message: &str) -> String {
        format!("{} at line {}, column {}", message, token.line, token.column)
    }

    pub(crate) fn error_at_current(&self, message: &str) -> String {
        self.error_at(self.peek(), message)
    }

    pub(crate) fn error_at_previous(&self, message: &str) -> String {
        if self.current == 0 {
            return self.error_at_current(message);
        }
        self.error_at(self.previous(), message)
    }

    fn synchronize(&mut self) {
        // Since we currently drop newline tokens in the lexer, we approximate
        // "statement boundaries" by scanning for keywords that typically start
        // a new statement or are common block terminators.
        if self.is_at_end() {
            return;
        }

        let start_index = self.current;

        while !self.is_at_end() {
            match self.peek().token_type {
                // Statement separators: consume and stop.
                TokenType::Semicolon => {
                    self.advance();
                    return;
                }

                // Common block terminators: consume and stop to avoid re-erroring on the terminator.
                TokenType::EndIf
                | TokenType::EndDo
                | TokenType::Next
                | TokenType::Loop
                | TokenType::EndCase => {
                    self.advance();
                    return;
                }

                // CASE branch keywords: stop here so the surrounding CASE parser can pick it up.
                TokenType::Otherwise | TokenType::Case => return,

                // IF branch keywords: stop here so the surrounding IF parser can pick it up.
                TokenType::Else | TokenType::ElseIf => return,

                // Likely statement starters
                TokenType::Function
                | TokenType::Procedure
                | TokenType::If
                | TokenType::Do
                | TokenType::While
                | TokenType::For
                | TokenType::Return
                | TokenType::Local
                | TokenType::Static
                | TokenType::Private
                | TokenType::Public
                | TokenType::Use
                | TokenType::Select => return,

                _ => {
                    self.advance();
                }
            }
        }

        // If we somehow didn't move, advance one token to prevent an infinite loop.
        if self.current == start_index && !self.is_at_end() {
            self.advance();
        }
    }
}

fn span_from_message_or_token(message: &str, token: &Token) -> Span {
    if let Some((line, col)) = extract_line_col(message) {
        let token_len = token.lexeme.chars().count().max(1);
        // If the location doesn't match the current token, avoid claiming a long range.
        let len = if token.line == line as usize && token.column == col as usize {
            token_len
        } else {
            1
        };
        return Span::new(line as usize, col as usize, len);
    }

    let len = token.lexeme.chars().count().max(1);
    Span::new(token.line, token.column, len)
}

fn extract_line_col(message: &str) -> Option<(u32, u32)> {
    let line = extract_number_after(message, "at line ")?;
    let col = extract_number_after(message, "column ").unwrap_or(1);
    Some((line, col))
}

fn extract_number_after(message: &str, needle: &str) -> Option<u32> {
    let start = message.find(needle)? + needle.len();
    let digits: String = message[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse::<u32>().ok()
}

pub struct ParseResult {
    pub program: Program,
    pub diagnostics: Vec<Diagnostic>,
}
