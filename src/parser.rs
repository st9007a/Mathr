use crate::ast::ast::ASTNode;
use crate::error::UnexpectedTokenError;
use crate::token::{Token, TokenIterator, Tokenizer};

pub struct Parser {
    toke_iter: TokenIterator,
}

impl Parser {
    pub fn from_text(text: &str) -> Self {
        Self {
            toke_iter: Tokenizer::new(text).into_iter(),
        }
    }

    fn term(&mut self) -> Result<Box<dyn ASTNode>, UnexpectedTokenError> {
        match self.toke_iter.next() {
            Some(token) => Ok(),
            None => Err(UnexpectedTokenError::new(Token::EOF)),
        }
    }
}
