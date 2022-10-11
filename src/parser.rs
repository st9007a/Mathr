use crate::ast::{ASTNode, AddNode, IntegerNode, SubNode};
use crate::error::UnexpectedTokenError;
use crate::token::{BinaryOpType, Token, TokenIterator, Tokenizer};

pub struct Parser {
    token_iter: TokenIterator,
}

impl Parser {
    pub fn from_text(text: &str) -> Self {
        Self {
            token_iter: Tokenizer::new(text).into_iter(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        self.expr()
    }

    fn term(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        if let Some(token) = self.token_iter.next() {
            match token {
                Token::Integer(value) => Ok(Box::new(IntegerNode::new(value))),
                Token::ParentheseStart => {
                    let node = self.expr()?;

                    if let Some(next_token) = self.token_iter.next() {
                        match next_token {
                            Token::ParentheseEnd => Ok(node),
                            _ => Err(UnexpectedTokenError::new(next_token)),
                        }
                    } else {
                        Err(UnexpectedTokenError::new(token))
                    }
                }
                _ => Err(UnexpectedTokenError::new(token)),
            }
        } else {
            Err(UnexpectedTokenError::new(Token::EOF))
        }
    }

    fn expr(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        let left = self.term()?;

        if let Some(token) = self.token_iter.next() {
            match token {
                Token::BinaryOp(ref op_type) => match op_type {
                    BinaryOpType::Add => {
                        let right = self.term()?;
                        Ok(Box::new(AddNode::new(left, right)))
                    }
                    BinaryOpType::Sub => {
                        let right = self.term()?;
                        Ok(Box::new(SubNode::new(left, right)))
                    }
                    _ => Err(UnexpectedTokenError::new(token)),
                },
                _ => Err(UnexpectedTokenError::new(token)),
            }
        } else {
            Err(UnexpectedTokenError::new(Token::EOF))
        }
    }
}
