use std::error;
use std::fmt;
use std::result::Result;

#[derive(Debug)]
pub struct InvalidTokenError;

impl InvalidTokenError {
    pub fn new() -> Self {
        Self {}
    }
}

impl error::Error for InvalidTokenError {}

impl fmt::Display for InvalidTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid token")
    }
}

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
    EOF,
}

pub struct Tokenizer {
    charvec: Vec<char>,
    ptr: usize,
}

impl Tokenizer {
    pub fn new(text: &str) -> Self {
        Self {
            charvec: text.chars().collect(),
            ptr: 0,
        }
    }

    fn make_integer(&mut self) -> Result<Token, InvalidTokenError> {
        let mut cur = String::new();
        while self.ptr < self.charvec.len() && self.charvec[self.ptr].is_numeric() {
            cur.push(self.charvec[self.ptr]);
            self.ptr += 1;
        }

        cur.parse::<u32>()
            .map(|num| Token::Integer(num))
            .map_err(|_| InvalidTokenError::new())
    }

    fn make_binary_op(&mut self) -> Result<Token, InvalidTokenError> {
        let ch = self.charvec[self.ptr];

        self.ptr += 1;

        match ch {
            '+' => Ok(Token::BinaryOp(BinaryOpType::Add)),
            '-' => Ok(Token::BinaryOp(BinaryOpType::Sub)),
            '*' => Ok(Token::BinaryOp(BinaryOpType::Mul)),
            '/' => Ok(Token::BinaryOp(BinaryOpType::Div)),
            _ => Err(InvalidTokenError::new()),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ptr < self.charvec.len() && self.charvec[self.ptr] == ' ' {
            self.ptr += 1
        }
    }

    pub fn step(&mut self) -> Result<Token, InvalidTokenError> {
        self.skip_whitespace();

        if self.ptr == self.charvec.len() {
            return Ok(Token::EOF);
        }

        let ch = self.charvec[self.ptr];

        if ch.is_numeric() {
            self.make_integer()
        } else if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
            self.make_binary_op()
        } else {
            Err(InvalidTokenError::new())
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.step().ok()
    }
}
