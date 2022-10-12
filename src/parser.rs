use std::iter::Peekable;

use crate::ast::{ASTNode, AddNode, DivNode, IntegerNode, MulNode, NegNode, PosNode, SubNode};
use crate::error::UnexpectedTokenError;
use crate::token::{Token, TokenIterator, Tokenizer};

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

    fn peek_next_token(&mut self) -> Option<&Token> {
        self.token_iter.peek()
    }

    fn get_next_token(&mut self) -> Option<Token> {
        self.token_iter.next()
    }

    fn factor(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        if let Some(token) = self.get_next_token() {
            match token {
                Token::Integer(value) => Ok(Box::new(IntegerNode::new(value))),
                Token::Add => Ok(Box::new(PosNode::new(self.factor()?))),
                Token::Sub => Ok(Box::new(NegNode::new(self.factor()?))),
                Token::ParentheseStart => {
                    let node = self.expr()?;

                    self.get_next_token()
                        .ok_or(UnexpectedTokenError::new(Token::EOF))
                        .and_then(move |next_token| match next_token {
                            Token::ParentheseEnd => Ok(node),
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
        let mut left = self.factor()?;

        while let Some(token) = self.peek_next_token() {
            match token {
                Token::Mul => {
                    self.get_next_token();
                    left = Box::new(MulNode::new(left, self.factor()?));
                }
                Token::Div => {
                    self.get_next_token();
                    left = Box::new(DivNode::new(left, self.factor()?));
                }
                _ => {
                    break;
                }
            }
        }

        Ok(left)
    }

    fn expr(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        let mut left = self.term()?;

        while let Some(token) = self.peek_next_token() {
            match token {
                Token::Add => {
                    self.get_next_token();
                    left = Box::new(AddNode::new(left, self.term()?));
                }
                Token::Sub => {
                    self.get_next_token();
                    left = Box::new(SubNode::new(left, self.term()?));
                }
                _ => {
                    break;
                }
            }
        }

        Ok(left)
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

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 33);
    }

    #[test]
    fn test_factor_unary_op() {
        let mut parser = Parser::from_text("- -   12");
        let node = parser.parse();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 12);
    }

    #[test]
    fn test_parse() {
        let mut parser = Parser::from_text("1 + 2 * (-3 - 4 / 2) + 10");
        let node = parser.parse();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 1);
    }
}
