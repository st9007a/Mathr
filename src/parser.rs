use std::iter::Peekable;

use crate::ast::{ASTNode, AddNode, DivNode, IntegerNode, MulNode, SubNode};
use crate::error::UnexpectedTokenError;
use crate::token::{BinaryOpType, Token, TokenIterator, Tokenizer};

pub struct Parser {
    token_iter: Peekable<TokenIterator>,
}

impl Parser {
    pub fn from_text(text: &str) -> Self {
        Self {
            token_iter: Tokenizer::new(text).into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        self.expr()
    }

    fn get_next_token(&mut self) -> Option<Token> {
        self.token_iter.next()
    }

    fn factor(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        if let Some(token) = self.get_next_token() {
            match token {
                Token::Integer(value) => Ok(Box::new(IntegerNode::new(value))),
                Token::ParentheseStart => {
                    let node = self.expr()?;

                    self.get_next_token()
                        .ok_or(UnexpectedTokenError::new(Token::EOF))
                        .and_then(move |next_token| match next_token {
                            Token::ParentheseStart => Ok(node),
                            _ => Err(UnexpectedTokenError::new(next_token)),
                        })
                }
                _ => Err(UnexpectedTokenError::new(token)),
            }
        } else {
            Err(UnexpectedTokenError::new(Token::EOF))
        }
    }

    fn term(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        let left = self.factor()?;

        if let Some(token) = self.get_next_token() {
            match token {
                Token::BinaryOp(ref op_type) => match op_type {
                    BinaryOpType::Mul => Ok(Box::new(MulNode::new(left, self.factor()?))),
                    BinaryOpType::Div => Ok(Box::new(DivNode::new(left, self.factor()?))),
                    _ => Err(UnexpectedTokenError::new(token)),
                },
                _ => Err(UnexpectedTokenError::new(token)),
            }
        } else {
            Ok(left)
        }
    }

    fn expr(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        let left = self.term()?;

        if let Some(token) = self.get_next_token() {
            match token {
                Token::BinaryOp(ref op_type) => match op_type {
                    BinaryOpType::Add => Ok(Box::new(AddNode::new(left, self.term()?))),
                    BinaryOpType::Sub => Ok(Box::new(SubNode::new(left, self.term()?))),
                    _ => Err(UnexpectedTokenError::new(token)),
                },
                _ => Err(UnexpectedTokenError::new(token)),
            }
        } else {
            Ok(left)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_factor_integer() {
        let mut parser = Parser::from_text(" 123   ");
        let node = parser.parse();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 123);
    }

    #[test]
    fn test_term() {
        let mut parser = Parser::from_text("4 * 12");
        let node = parser.parse();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 48);
    }

    #[test]
    fn test_expr() {
        let mut parser = Parser::from_text("4311 + 111");
        let node = parser.parse();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 4422);
    }

    #[test]
    fn test_factor_parenthesis() {
        let mut parser = Parser::from_text(" ( 12 + 21)");
        let node = parser.parse();

        println!("{:?}", node.err());
        assert!(false);

        // assert!(node.is_ok());
        // assert_eq!(node.unwrap().eval(), 33);
    }
}
