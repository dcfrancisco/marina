use crate::ast::*;
use crate::token::{Token, TokenType};

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
    
    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(&[TokenType::Local]) {
            self.var_declaration(VarScope::Local)
        } else if self.match_token(&[TokenType::Static]) {
            self.var_declaration(VarScope::Static)
        } else if self.match_token(&[TokenType::Private]) {
            self.var_declaration(VarScope::Private)
        } else if self.match_token(&[TokenType::Public]) {
            self.var_declaration(VarScope::Public)
        } else if self.match_token(&[TokenType::Function]) {
            self.function_declaration(false)
        } else if self.match_token(&[TokenType::Procedure]) {
            self.function_declaration(true)
        } else {
            self.statement()
        }
    }
    
    fn var_declaration(&mut self, scope: VarScope) -> Result<Stmt, String> {
        let mut declarations = Vec::new();
        
        loop {
            let name = self.consume_identifier("Expected variable name")?;
            
            let initializer = if self.match_token(&[TokenType::Assign, TokenType::Colon]) {
                Some(self.expression()?)
            } else {
                None
            };
            
            declarations.push(Stmt::VarDecl {
                name,
                initializer,
                scope: scope.clone(),
            });
            
            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        
        if declarations.len() == 1 {
            Ok(declarations.into_iter().next().unwrap())
        } else {
            Ok(Stmt::Block(declarations))
        }
    }
    
    fn function_declaration(&mut self, is_procedure: bool) -> Result<Stmt, String> {
        let name = self.consume_identifier("Expected function name")?;
        
        self.consume(&TokenType::LeftParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                params.push(self.consume_identifier("Expected parameter name")?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;
        
        let mut body = Vec::new();
        while !self.is_at_end() {
            let current_lexeme = self.peek().lexeme.to_uppercase();
            
            // Stop at RETURN or ENDFUNC/ENDPROC
            if current_lexeme == "RETURN" || current_lexeme == "ENDFUNC" || current_lexeme == "ENDPROC" {
                break;
            }
            
            // Stop at next function
            if self.check(&TokenType::Function) || self.check(&TokenType::Procedure) {
                break;
            }
            
            body.push(self.statement()?);
        }
        
        // Handle RETURN if present
        if !self.is_at_end() && self.peek().lexeme.to_uppercase() == "RETURN" {
            self.advance();
            if !self.check(&TokenType::Semicolon) && !self.is_at_end() {
                let expr = self.expression()?;
                body.push(Stmt::Return(Some(expr)));
            } else {
                body.push(Stmt::Return(None));
            }
        }
        
        Ok(Stmt::Function {
            name,
            params,
            body,
            is_procedure,
        })
    }
    
    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&[TokenType::Local]) {
            self.var_declaration(VarScope::Local)
        } else if self.match_token(&[TokenType::Return]) {
            self.return_statement()
        } else if self.match_token(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_token(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_token(&[TokenType::Do]) {
            self.do_while_statement()
        } else if self.match_token(&[TokenType::For]) {
            self.for_statement()
        } else if self.match_token(&[TokenType::Loop]) {
            self.loop_statement()
        } else if self.match_token(&[TokenType::Exit]) {
            Ok(Stmt::Exit)
        } else if self.match_token(&[TokenType::Use]) {
            self.db_use_statement()
        } else if self.match_token(&[TokenType::DbSkip]) {
            self.db_skip_statement()
        } else if self.match_token(&[TokenType::DbGoTop]) {
            Ok(Stmt::DbGoTop)
        } else if self.match_token(&[TokenType::DbGoBottom]) {
            Ok(Stmt::DbGoBottom)
        } else if self.match_token(&[TokenType::DbSeek]) {
            self.db_seek_statement()
        } else if self.match_token(&[TokenType::Replace]) {
            self.replace_statement()
        } else if self.match_token(&[TokenType::QuestionMark]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }
    
    fn return_statement(&mut self) -> Result<Stmt, String> {
        let value = if !self.is_at_end() && !self.check(&TokenType::Newline) {
            Some(self.expression()?)
        } else {
            None
        };
        Ok(Stmt::Return(value))
    }
    
    fn if_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;
        
        let mut then_branch = Vec::new();
        while !self.check(&TokenType::EndIf) 
            && !self.check(&TokenType::Else) 
            && !self.check(&TokenType::ElseIf) 
            && !self.is_at_end() {
            then_branch.push(self.declaration()?);
        }
        
        let else_branch = if self.match_token(&[TokenType::Else]) {
            let mut else_stmts = Vec::new();
            while !self.check(&TokenType::EndIf) && !self.is_at_end() {
                else_stmts.push(self.declaration()?);
            }
            Some(else_stmts)
        } else {
            None
        };
        
        if self.match_token(&[TokenType::EndIf]) {
            // EndIf consumed
        }
        
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn while_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;
        
        let mut body = Vec::new();
        while !self.check(&TokenType::EndDo) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        
        if self.match_token(&[TokenType::EndDo]) {
            // EndDo consumed
        }
        
        Ok(Stmt::While { condition, body })
    }
    
    fn do_while_statement(&mut self) -> Result<Stmt, String> {
        let mut body = Vec::new();
        
        while !self.check(&TokenType::While) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        
        self.consume(&TokenType::While, "Expected WHILE after DO block")?;
        let condition = self.expression()?;
        
        Ok(Stmt::DoWhile { body, condition })
    }
    
    fn for_statement(&mut self) -> Result<Stmt, String> {
        let variable = self.consume_identifier("Expected loop variable")?;
        self.consume(&TokenType::Assign, "Expected '=' or ':=' after loop variable")?;
        let start = self.expression()?;
        self.consume(&TokenType::To, "Expected TO in FOR loop")?;
        let end = self.expression()?;
        
        let step = if self.match_token(&[TokenType::Step]) {
            Some(self.expression()?)
        } else {
            None
        };
        
        let mut body = Vec::new();
        while !self.check(&TokenType::Next) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        
        if self.match_token(&[TokenType::Next]) {
            // NEXT consumed
        }
        
        Ok(Stmt::For {
            variable,
            start,
            end,
            step,
            body,
        })
    }
    
    fn loop_statement(&mut self) -> Result<Stmt, String> {
        let mut body = Vec::new();
        
        // LOOP without condition - infinite loop until EXIT
        while !self.is_at_end() {
            // Check for end of loop (implementation-specific)
            if self.peek().lexeme.to_uppercase() == "ENDLOOP" {
                self.advance();
                break;
            }
            body.push(self.declaration()?);
        }
        
        Ok(Stmt::Loop { body })
    }
    
    fn db_use_statement(&mut self) -> Result<Stmt, String> {
        let filename = if self.check(&TokenType::String) {
            self.advance().lexeme.clone()
        } else {
            self.consume_identifier("Expected database filename")?
        };
        
        let alias = if self.peek().lexeme.to_uppercase() == "ALIAS" {
            self.advance();
            Some(self.consume_identifier("Expected alias name")?)
        } else {
            None
        };
        
        Ok(Stmt::DbUse { filename, alias })
    }
    
    fn db_skip_statement(&mut self) -> Result<Stmt, String> {
        let count = if !self.is_at_end() && !self.check(&TokenType::Newline) {
            Some(self.expression()?)
        } else {
            None
        };
        Ok(Stmt::DbSkip(count))
    }
    
    fn db_seek_statement(&mut self) -> Result<Stmt, String> {
        let key = self.expression()?;
        Ok(Stmt::DbSeek { key })
    }
    
    fn replace_statement(&mut self) -> Result<Stmt, String> {
        let field = self.consume_identifier("Expected field name")?;
        
        if self.peek().lexeme.to_uppercase() == "WITH" {
            self.advance();
        }
        
        let value = self.expression()?;
        
        Ok(Stmt::Replace { field, value })
    }
    
    fn print_statement(&mut self) -> Result<Stmt, String> {
        // ? can print multiple expressions separated by commas
        let mut args = Vec::new();
        
        if !self.is_at_end() && !self.check(&TokenType::Newline) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        
        // Convert to a PRINT function call
        Ok(Stmt::Expression(Expr::Call {
            name: "?".to_string(),
            args,
        }))
    }
    
    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        Ok(Stmt::Expression(expr))
    }
    
    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.logical_or()?;
        
        if self.match_token(&[TokenType::Assign]) {
            if let Expr::Variable(name) = expr {
                let value = self.assignment()?;
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else {
                return Err("Invalid assignment target".to_string());
            }
        }
        
        Ok(expr)
    }
    
    fn logical_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.logical_and()?;
        
        while self.match_token(&[TokenType::Or]) {
            let right = self.logical_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn logical_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        
        while self.match_token(&[TokenType::And]) {
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        
        while let Some(op) = self.match_binary_op(&[TokenType::Equal, TokenType::NotEqual]) {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        
        while let Some(op) = self.match_binary_op(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        
        while let Some(op) = self.match_binary_op(&[TokenType::Plus, TokenType::Minus]) {
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.power()?;
        
        while let Some(op) = self.match_binary_op(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let right = self.power()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn power(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        
        while let Some(op) = self.match_binary_op(&[TokenType::Power]) {
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::Not]) {
            let operand = self.unary()?;
            return Ok(Expr::Unary {
                operator: UnaryOp::Not,
                operand: Box::new(operand),
            });
        }
        
        if self.match_token(&[TokenType::Minus]) {
            let operand = self.unary()?;
            return Ok(Expr::Unary {
                operator: UnaryOp::Negate,
                operand: Box::new(operand),
            });
        }
        
        self.call()
    }
    
    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&[TokenType::LeftBracket]) {
                let index = self.expression()?;
                self.consume(&TokenType::RightBracket, "Expected ']' after index")?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let name = match callee {
            Expr::Variable(n) => n,
            _ => return Err("Invalid function call".to_string()),
        };
        
        let mut args = Vec::new();
        
        if !self.check(&TokenType::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after arguments")?;
        
        Ok(Expr::Call { name, args })
    }
    
    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Boolean(true));
        }
        
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Boolean(false));
        }
        
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Nil);
        }
        
        if self.match_token(&[TokenType::Number]) {
            let value = self.previous().lexeme.parse::<f64>()
                .map_err(|_| "Invalid number")?;
            return Ok(Expr::Number(value));
        }
        
        if self.match_token(&[TokenType::String]) {
            return Ok(Expr::String(self.previous().lexeme.clone()));
        }
        
        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous().lexeme.clone()));
        }
        
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(expr);
        }
        
        if self.match_token(&[TokenType::LeftBrace]) {
            let mut elements = Vec::new();
            if !self.check(&TokenType::RightBrace) {
                loop {
                    elements.push(self.expression()?);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(&TokenType::RightBrace, "Expected '}' after array elements")?;
            return Ok(Expr::Array(elements));
        }
        
        Err(format!("Unexpected token: {:?}", self.peek()))
    }
    
    fn match_binary_op(&mut self, types: &[TokenType]) -> Option<BinaryOp> {
        for token_type in types {
            if self.check(token_type) {
                let op = match token_type {
                    TokenType::Plus => BinaryOp::Add,
                    TokenType::Minus => BinaryOp::Subtract,
                    TokenType::Star => BinaryOp::Multiply,
                    TokenType::Slash => BinaryOp::Divide,
                    TokenType::Percent => BinaryOp::Modulo,
                    TokenType::Power => BinaryOp::Power,
                    TokenType::Equal => BinaryOp::Equal,
                    TokenType::NotEqual => BinaryOp::NotEqual,
                    TokenType::Less => BinaryOp::Less,
                    TokenType::Greater => BinaryOp::Greater,
                    TokenType::LessEqual => BinaryOp::LessEqual,
                    TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                    TokenType::And => BinaryOp::And,
                    TokenType::Or => BinaryOp::Or,
                    _ => return None,
                };
                self.advance();
                return Some(op);
            }
        }
        None
    }
    
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(format!("{} at line {}", message, self.peek().line))
        }
    }
    
    fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        if self.check(&TokenType::Identifier) {
            Ok(self.advance().lexeme.clone())
        } else {
            Err(format!("{} at line {}", message, self.peek().line))
        }
    }
}
