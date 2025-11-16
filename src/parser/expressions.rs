use crate::ast::*;
use crate::token::TokenType;
use super::Parser;

impl Parser {
    pub(crate) fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }
    
    pub(crate) fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.logical_or()?;
        
        if self.match_token(&[TokenType::Assign]) {
            match expr {
                Expr::Variable(name) => {
                    let value = self.assignment()?;
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                Expr::Index { object, index } => {
                    // Indexed assignment: arr[idx] := value
                    let value = self.assignment()?;
                    // Use special __SET_INDEX__ call
                    return Ok(Expr::Call {
                        name: "__SET_INDEX__".to_string(),
                        args: vec![*object, *index, value],
                    });
                }
                _ => {
                    return Err("Invalid assignment target".to_string());
                }
            }
        } else if self.match_token(&[TokenType::PlusAssign, TokenType::MinusAssign, 
                                      TokenType::MultiplyAssign, TokenType::DivideAssign]) {
            let op_type = self.previous().token_type.clone();
            match expr {
                Expr::Variable(name) => {
                    let value = self.assignment()?;
                    // Transform x += y into x := x + y
                    let op = match op_type {
                        TokenType::PlusAssign => BinaryOp::Add,
                        TokenType::MinusAssign => BinaryOp::Subtract,
                        TokenType::MultiplyAssign => BinaryOp::Multiply,
                        TokenType::DivideAssign => BinaryOp::Divide,
                        _ => unreachable!(),
                    };
                    let combined = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: op,
                        right: Box::new(value),
                    };
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(combined),
                    });
                }
                _ => {
                    return Err("Invalid augmented assignment target".to_string());
                }
            }
        } else if self.match_token(&[TokenType::Increment, TokenType::Decrement]) {
            let op_type = self.previous().token_type.clone();
            match expr {
                Expr::Variable(name) => {
                    // Transform x++ into x := x + 1
                    let one = Expr::Number(1.0);
                    let op = if op_type == TokenType::Increment {
                        BinaryOp::Add
                    } else {
                        BinaryOp::Subtract
                    };
                    let combined = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: op,
                        right: Box::new(one),
                    };
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(combined),
                    });
                }
                _ => {
                    return Err("Invalid increment/decrement target".to_string());
                }
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
    
    pub(crate) fn match_binary_op(&mut self, types: &[TokenType]) -> Option<BinaryOp> {
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
}
