use crate::ast::*;
use crate::token::TokenType;
use super::Parser;

impl Parser {
    pub(crate) fn declaration(&mut self) -> Result<Stmt, String> {
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
    
    pub(crate) fn statement(&mut self) -> Result<Stmt, String> {
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
        } else if self.match_token(&[TokenType::Case]) {
            self.case_statement()
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
    
    fn case_statement(&mut self) -> Result<Stmt, String> {
        // CASE expr
        let expr = self.expression()?;
        self.skip_newlines();
        
        let mut cases = Vec::new();
        let mut otherwise = None;
        
        // Parse CASE value clauses
        while self.match_token(&[TokenType::Case]) {
            let value = self.expression()?;
            self.skip_newlines();
            
            let mut statements = Vec::new();
            while !self.is_at_end() && 
                  !self.check(&TokenType::Case) && 
                  !self.check(&TokenType::Otherwise) &&
                  !self.check(&TokenType::EndCase) {
                statements.push(self.declaration()?);
            }
            
            cases.push((value, statements));
        }
        
        // Parse optional OTHERWISE clause
        if self.match_token(&[TokenType::Otherwise]) {
            self.skip_newlines();
            let mut statements = Vec::new();
            while !self.is_at_end() && !self.check(&TokenType::EndCase) {
                statements.push(self.declaration()?);
            }
            otherwise = Some(statements);
        }
        
        self.consume(&TokenType::EndCase, "Expected ENDCASE")?;
        
        Ok(Stmt::Case { expr, cases, otherwise })
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
}
