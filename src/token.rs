use std::result::Result;

use crate::error::InvalidSyntaxError;

#[derive(Debug, PartialEq)]
pub enum Token {
    INTEGER(i32),
    ID(String),
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    COMMA,
    DOT,
    ASSIGN,
    PI,
    E,
    SEMI,
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
                '+' => Ok(Token::PLUS),
                '-' => Ok(Token::MINUS),
                '*' => Ok(Token::MUL),
                '/' => Ok(Token::DIV),
                '(' => Ok(Token::LPAREN),
                ')' => Ok(Token::RPAREN),
                ',' => Ok(Token::COMMA),
                '.' => Ok(Token::DOT),
                '=' => Ok(Token::ASSIGN),
                ';' => Ok(Token::SEMI),
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
            .map(|num| Token::INTEGER(num))
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
            Ok(Token::PI)
        } else {
            Ok(Token::ID(cur))
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

        assert_eq!(tokenizer.step().ok(), Some(Token::INTEGER(1)));
        assert_eq!(tokenizer.step().ok(), Some(Token::PLUS));
        assert_eq!(tokenizer.step().ok(), Some(Token::INTEGER(2)));
        assert_eq!(tokenizer.step().ok(), Some(Token::MUL));
        assert_eq!(tokenizer.step().ok(), Some(Token::LPAREN));
        assert_eq!(tokenizer.step().ok(), Some(Token::INTEGER(510)));
        assert_eq!(tokenizer.step().ok(), Some(Token::MINUS));
        assert_eq!(tokenizer.step().ok(), Some(Token::INTEGER(33)));
        assert_eq!(tokenizer.step().ok(), Some(Token::RPAREN));
        assert_eq!(tokenizer.step().ok(), Some(Token::DIV));
        assert_eq!(tokenizer.step().ok(), Some(Token::INTEGER(7)));
        assert_eq!(tokenizer.step().ok(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut tokenizer = Tokenizer::new("1 + 2*(510   - 33 )  / 7 ").into_iter();

        assert_eq!(tokenizer.next(), Some(Token::INTEGER(1)));
        assert_eq!(tokenizer.next(), Some(Token::PLUS));
        assert_eq!(tokenizer.next(), Some(Token::INTEGER(2)));
        assert_eq!(tokenizer.next(), Some(Token::MUL));
        assert_eq!(tokenizer.next(), Some(Token::LPAREN));
        assert_eq!(tokenizer.next(), Some(Token::INTEGER(510)));
        assert_eq!(tokenizer.next(), Some(Token::MINUS));
        assert_eq!(tokenizer.next(), Some(Token::INTEGER(33)));
        assert_eq!(tokenizer.next(), Some(Token::RPAREN));
        assert_eq!(tokenizer.next(), Some(Token::DIV));
        assert_eq!(tokenizer.next(), Some(Token::INTEGER(7)));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_parse_indentity() {
        let mut tokenizer = Tokenizer::new("e + 12 * -1.2e10 * pi - my_var");

        assert_eq!(tokenizer.step().ok(), Some(Token::E));
        assert_eq!(tokenizer.step().ok(), Some(Token::PLUS));
        assert_eq!(tokenizer.step().ok(), Some(Token::INTEGER(12)));
        assert_eq!(tokenizer.step().ok(), Some(Token::MUL));
        assert_eq!(tokenizer.step().ok(), Some(Token::MINUS));
        assert_eq!(tokenizer.step().ok(), Some(Token::INTEGER(1)));
        assert_eq!(tokenizer.step().ok(), Some(Token::DOT));
        assert_eq!(tokenizer.step().ok(), Some(Token::INTEGER(2)));
        assert_eq!(tokenizer.step().ok(), Some(Token::E));
        assert_eq!(tokenizer.step().ok(), Some(Token::INTEGER(10)));
        assert_eq!(tokenizer.step().ok(), Some(Token::MUL));
        assert_eq!(tokenizer.step().ok(), Some(Token::PI));
        assert_eq!(tokenizer.step().ok(), Some(Token::MINUS));
        assert_eq!(
            tokenizer.step().ok(),
            Some(Token::ID("my_var".to_string()))
        );
        assert_eq!(tokenizer.step().ok(), None);
    }
}
