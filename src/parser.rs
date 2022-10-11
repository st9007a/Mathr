use crate::ast::{ASTNode, AddNode, DivNode, IntegerNode, MulNode, SubNode};
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

    fn factor(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
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

    fn term(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        let left = self.factor()?;

        if let Some(token) = self.token_iter.next() {
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

        if let Some(token) = self.token_iter.next() {
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
