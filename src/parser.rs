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

    pub fn from_tokenizer(tokenizer: Tokenizer) -> Self {
        Self {
            token_iter: tokenizer.into_iter().peekable(),
        }
    }

    pub fn from_iter(token_iter: TokenIterator) -> Self {
        Self {
            token_iter: token_iter.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        self.expr()
    }

    pub fn factor(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        if let Some(token) = self.next_token() {
            match token {
                Token::INTEGER(value) => Ok(Box::new(IntegerNode::new(value))),
                Token::PLUS => Ok(Box::new(PosNode::new(self.factor()?))),
                Token::MINUS => Ok(Box::new(NegNode::new(self.factor()?))),
                Token::LPAREN => {
                    let node = self.expr()?;

                    self.next_token()
                        .ok_or(UnexpectedTokenError::new(Token::EOF))
                        .and_then(move |next_token| match next_token {
                            Token::RPAREN => Ok(node),
                            _ => Err(UnexpectedTokenError::new(next_token)),
                        })
                }
                _ => Err(UnexpectedTokenError::new(token)),
            }
        } else {
            Err(UnexpectedTokenError::new(Token::EOF))
        }
    }

    pub fn term(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        let mut left = self.factor()?;

        while let Some(token) = self.peek_token() {
            match token {
                Token::MUL => {
                    self.next_token();
                    left = Box::new(MulNode::new(left, self.factor()?));
                }
                Token::DIV => {
                    self.next_token();
                    left = Box::new(DivNode::new(left, self.factor()?));
                }
                _ => {
                    break;
                }
            }
        }

        Ok(left)
    }

    pub fn expr(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        let mut left = self.term()?;

        while let Some(token) = self.peek_token() {
            match token {
                Token::PLUS => {
                    self.next_token();
                    left = Box::new(AddNode::new(left, self.term()?));
                }
                Token::MINUS => {
                    self.next_token();
                    left = Box::new(SubNode::new(left, self.term()?));
                }
                _ => {
                    break;
                }
            }
        }

        Ok(left)
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.token_iter.peek()
    }

    fn next_token(&mut self) -> Option<Token> {
        self.token_iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_factor_integer() {
        let mut parser = Parser::from_text(" 123   ");
        let node = parser.factor();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 123);
    }

    #[test]
    fn test_term() {
        let mut parser = Parser::from_text("4 * 12");
        let node = parser.term();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 48);
    }

    #[test]
    fn test_expr() {
        let mut parser = Parser::from_text("4311 + 111");
        let node = parser.expr();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 4422);
    }

    #[test]
    fn test_factor_parenthesis() {
        let mut parser = Parser::from_text(" ( 12 + 21)");
        let node = parser.factor();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(), 33);
    }

    #[test]
    fn test_factor_unary_op() {
        let mut parser = Parser::from_text("- -   12");
        let node = parser.factor();

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
