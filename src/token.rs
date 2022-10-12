use std::result::Result;

use crate::error::InvalidSyntaxError;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Identity(String),
    Add,
    Sub,
    Mul,
    Div,
    ParentheseStart,
    ParentheseEnd,
    Comma,
    Dot,
    Assign,
    Pi,
    E,
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
            return Err(InvalidSyntaxError::new("eof".to_string()));
        }

        let ch = self.charvec[self.ptr];

        if ch.is_ascii_digit() {
            self.consume_integer()
        } else if ch.is_ascii_alphabetic() || ch == '_' {
            self.consume_identity()
        } else {
            self.ptr += 1;

            match ch {
                '+' => Ok(Token::Add),
                '-' => Ok(Token::Sub),
                '*' => Ok(Token::Mul),
                '/' => Ok(Token::Div),
                '(' => Ok(Token::ParentheseStart),
                ')' => Ok(Token::ParentheseEnd),
                ',' => Ok(Token::Comma),
                '.' => Ok(Token::Dot),
                '=' => Ok(Token::Assign),
                _ => Err(InvalidSyntaxError::new(ch.to_string())),
            }
        }
    }

    pub fn into_iter(self) -> TokenIterator {
        TokenIterator(self)
    }

    fn consume_integer(&mut self) -> Result<Token, InvalidSyntaxError> {
        let mut cur = String::new();
        while self.ptr < self.charvec.len() && self.charvec[self.ptr].is_ascii_digit() {
            cur.push(self.charvec[self.ptr]);
            self.ptr += 1;
        }

        cur.parse::<i32>()
            .map(|num| Token::Number(num))
            .map_err(|_| InvalidSyntaxError::new(cur))
    }

    fn consume_identity(&mut self) -> Result<Token, InvalidSyntaxError> {
        let mut cur = String::new();

        while self.ptr < self.charvec.len()
            && (self.charvec[self.ptr].is_ascii_alphanumeric() || self.charvec[self.ptr] == '_')
        {
            // FIXME: This will limit variable and function naming which cannot start from e and
            // follow a digit.
            if cur.eq("e") && self.charvec[self.ptr].is_ascii_digit() {
                return Ok(Token::E);
            }

            cur.push(self.charvec[self.ptr]);
            self.ptr += 1;
        }

        if cur.eq("e") {
            Ok(Token::E)
        } else if cur.eq("pi") {
            Ok(Token::Pi)
        } else {
            Ok(Token::Identity(cur))
        }
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

#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer};

    #[test]
    fn test_step() {
        let mut tokenizer = Tokenizer::new("1 + 2*(510   - 33 )  / 7 ");

        assert_eq!(tokenizer.step().ok(), Some(Token::Number(1)));
        assert_eq!(tokenizer.step().ok(), Some(Token::Add));
        assert_eq!(tokenizer.step().ok(), Some(Token::Number(2)));
        assert_eq!(tokenizer.step().ok(), Some(Token::Mul));
        assert_eq!(tokenizer.step().ok(), Some(Token::ParentheseStart));
        assert_eq!(tokenizer.step().ok(), Some(Token::Number(510)));
        assert_eq!(tokenizer.step().ok(), Some(Token::Sub));
        assert_eq!(tokenizer.step().ok(), Some(Token::Number(33)));
        assert_eq!(tokenizer.step().ok(), Some(Token::ParentheseEnd));
        assert_eq!(tokenizer.step().ok(), Some(Token::Div));
        assert_eq!(tokenizer.step().ok(), Some(Token::Number(7)));
        assert_eq!(tokenizer.step().ok(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut tokenizer = Tokenizer::new("1 + 2*(510   - 33 )  / 7 ").into_iter();

        assert_eq!(tokenizer.next(), Some(Token::Number(1)));
        assert_eq!(tokenizer.next(), Some(Token::Add));
        assert_eq!(tokenizer.next(), Some(Token::Number(2)));
        assert_eq!(tokenizer.next(), Some(Token::Mul));
        assert_eq!(tokenizer.next(), Some(Token::ParentheseStart));
        assert_eq!(tokenizer.next(), Some(Token::Number(510)));
        assert_eq!(tokenizer.next(), Some(Token::Sub));
        assert_eq!(tokenizer.next(), Some(Token::Number(33)));
        assert_eq!(tokenizer.next(), Some(Token::ParentheseEnd));
        assert_eq!(tokenizer.next(), Some(Token::Div));
        assert_eq!(tokenizer.next(), Some(Token::Number(7)));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_parse_indentity() {
        let mut tokenizer = Tokenizer::new("e + 12 * -1.2e10 * pi - my_var");

        assert_eq!(tokenizer.step().ok(), Some(Token::E));
        assert_eq!(tokenizer.step().ok(), Some(Token::Add));
        assert_eq!(tokenizer.step().ok(), Some(Token::Number(12)));
        assert_eq!(tokenizer.step().ok(), Some(Token::Mul));
        assert_eq!(tokenizer.step().ok(), Some(Token::Sub));
        assert_eq!(tokenizer.step().ok(), Some(Token::Number(1)));
        assert_eq!(tokenizer.step().ok(), Some(Token::Dot));
        assert_eq!(tokenizer.step().ok(), Some(Token::Number(2)));
        assert_eq!(tokenizer.step().ok(), Some(Token::E));
        assert_eq!(tokenizer.step().ok(), Some(Token::Number(10)));
        assert_eq!(tokenizer.step().ok(), Some(Token::Mul));
        assert_eq!(tokenizer.step().ok(), Some(Token::Pi));
        assert_eq!(tokenizer.step().ok(), Some(Token::Sub));
        assert_eq!(
            tokenizer.step().ok(),
            Some(Token::Identity("my_var".to_string()))
        );
        assert_eq!(tokenizer.step().ok(), None);
    }
}
