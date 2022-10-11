use crate::error::InvalidSyntaxError;
use std::result::Result;

#[derive(Debug)]
pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Token {
    Integer(u32),
    BinaryOp(BinaryOpType),
    ParentheseStart,
    ParentheseEnd,
    Comma,
    BuiltinCall(String),
    BuiltinSymbol(String),
    EOF,
}

pub struct Tokenizer {
    charvec: Vec<char>,
    ptr: usize,
}

pub struct TokenIterator(Tokenizer);

impl Tokenizer {
    pub fn new(text: &str) -> Self {
        Self {
            charvec: text.chars().collect(),
            ptr: 0,
        }
    }

    pub fn step(&mut self) -> Result<Token, InvalidSyntaxError> {
        self.skip_whitespace();

        if self.ptr == self.charvec.len() {
            self.ptr += 1;
            return Ok(Token::EOF);
        }

        if self.ptr == self.charvec.len() + 1 {
            return Err(InvalidSyntaxError::new("eof".to_string()));
        }

        let ch = self.charvec[self.ptr];

        if ch.is_numeric() {
            self.consume_integer()
        } else {
            self.ptr += 1;

            match ch {
                '+' => Ok(Token::BinaryOp(BinaryOpType::Add)),
                '-' => Ok(Token::BinaryOp(BinaryOpType::Sub)),
                '*' => Ok(Token::BinaryOp(BinaryOpType::Mul)),
                '/' => Ok(Token::BinaryOp(BinaryOpType::Div)),
                '(' => Ok(Token::ParentheseStart),
                ')' => Ok(Token::ParentheseEnd),
                ',' => Ok(Token::Comma),
                _ => Err(InvalidSyntaxError::new(ch.to_string())),
            }
        }
    }

    pub fn into_iter(self) -> TokenIterator {
        TokenIterator(self)
    }

    fn consume_integer(&mut self) -> Result<Token, InvalidSyntaxError> {
        let mut cur = String::new();
        while self.ptr < self.charvec.len() && self.charvec[self.ptr].is_numeric() {
            cur.push(self.charvec[self.ptr]);
            self.ptr += 1;
        }

        cur.parse::<u32>()
            .map(|num| Token::Integer(num))
            .map_err(|_| InvalidSyntaxError::new(cur))
    }

    fn skip_whitespace(&mut self) {
        while self.ptr < self.charvec.len() && self.charvec[self.ptr] == ' ' {
            self.ptr += 1
        }
    }
}

impl Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.step().ok()
    }
}

impl IntoIterator for Tokenizer {
    type Item = Token;
    type IntoIter = TokenIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}
