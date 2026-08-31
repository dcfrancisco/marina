use super::Parser;
use crate::ast::*;
use crate::token::TokenType;

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
                    return Err(self.error_at_previous("Invalid assignment target"));
                }
            }
        } else if self.match_token(&[
            TokenType::PlusAssign,
            TokenType::MinusAssign,
            TokenType::MultiplyAssign,
            TokenType::DivideAssign,
        ]) {
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
                    return Err(self.error_at_previous("Invalid augmented assignment target"));
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
                    return Err(self.error_at_previous("Invalid increment/decrement target"));
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

        while let Some(op) =
            self.match_binary_op(&[TokenType::Star, TokenType::Slash, TokenType::Percent])
        {
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
            } else if self.match_token(&[TokenType::Dot]) {
                expr = self.finish_member_access(expr)?;
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
        let name = self.flatten_call_target(callee)?;

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

    fn finish_member_access(&mut self, object: Expr) -> Result<Expr, String> {
        let property = self.consume_identifier("Expected identifier after '.'")?;
        Ok(Expr::Member {
            object: Box::new(object),
            property,
        })
    }

    fn flatten_call_target(&self, callee: Expr) -> Result<String, String> {
        match callee {
            Expr::Variable(name) => Ok(name),
            Expr::Member { object, property } => {
                let base = self.flatten_call_target(*object)?;
                Ok(format!("{base}.{property}"))
            }
            _ => Err(self.error_at_previous("Invalid function call")),
        }
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
            let value = self
                .previous()
                .lexeme
                .parse::<f64>()
                .map_err(|_| self.error_at_previous("Invalid number"))?;
            return Ok(Expr::Number(value));
        }

        if self.match_token(&[TokenType::String]) {
            return Ok(Expr::String(self.previous().lexeme.clone()));
        }

        if self.match_token(&[TokenType::Identifier, TokenType::DbSkip]) {
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

        {
            let message = format!("Unexpected token: {:?}", self.peek());
            Err(self.error_at_current(&message))
        }
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
