use std::result::Result;

use crate::error::InterpreterError;

#[derive(Debug, PartialEq)]
pub enum Token {
    NUMBER(f64),
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

    pub fn step(&mut self) -> Result<Token, InterpreterError> {
        self.skip_char();

        self.peek_char()
            .ok_or(InterpreterError::InvalidSyntax("eof".to_string()))
            .and_then(|ch| {
                if ch.is_ascii_digit() {
                    self.next_number()
                } else if ch.is_ascii_alphabetic() || ch == '_' {
                    self.next_identity()
                } else {
                    self.next_char();

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
                        _ => Err(InterpreterError::InvalidSyntax(ch.to_string())),
                    }
                }
            })
    }

    pub fn into_iter(self) -> TokenIterator {
        TokenIterator(self)
    }

    fn next_number(&mut self) -> Result<Token, InterpreterError> {
        let mut cur = String::new();

        while let Some(ch) = self.peek_char() {
            if !ch.is_ascii_digit() {
                break;
            }

            cur.push(self.next_char().unwrap());
        }

        if let Some(ch) = self.peek_char() {
            if ch != '.' {
                return cur
                    .parse::<f64>()
                    .map(|value| Token::NUMBER(value))
                    .map_err(|_| InterpreterError::InvalidSyntax(cur));
            }

            cur.push(self.next_char().unwrap());

            while let Some(ch) = self.peek_char() {
                if !ch.is_ascii_digit() {
                    break;
                }

                cur.push(self.next_char().unwrap());
            }
        }

        cur.parse::<f64>()
            .map(|value| Token::NUMBER(value))
            .map_err(|_| InterpreterError::InvalidSyntax(cur))
    }

    fn next_identity(&mut self) -> Result<Token, InterpreterError> {
        let mut cur = String::new();

        while let Some(ch) = self.peek_char() {
            if !ch.is_ascii_alphanumeric() && ch != '_' {
                break;
            }

            if cur.eq("e") && ch.is_ascii_digit() {
                return Ok(Token::E);
            }

            cur.push(self.next_char().unwrap());
        }

        if cur.eq("e") {
            Ok(Token::E)
        } else if cur.eq("pi") {
            Ok(Token::PI)
        } else {
            Ok(Token::ID(cur))
        }
    }

    fn skip_char(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch != ' ' && ch != '\n' {
                break;
            }
            self.next_char();
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.ptr < self.charvec.len() {
            Some(self.charvec[self.ptr])
        } else {
            None
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.ptr < self.charvec.len() {
            let ch = self.charvec[self.ptr];
            self.ptr += 1;

            Some(ch)
        } else {
            None
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
        let mut tokenizer = Tokenizer::new("1 + 2*(510   - 33 )  / 7.5 ");

        assert_eq!(tokenizer.step().ok(), Some(Token::NUMBER(1f64)));
        assert_eq!(tokenizer.step().ok(), Some(Token::PLUS));
        assert_eq!(tokenizer.step().ok(), Some(Token::NUMBER(2f64)));
        assert_eq!(tokenizer.step().ok(), Some(Token::MUL));
        assert_eq!(tokenizer.step().ok(), Some(Token::LPAREN));
        assert_eq!(tokenizer.step().ok(), Some(Token::NUMBER(510f64)));
        assert_eq!(tokenizer.step().ok(), Some(Token::MINUS));
        assert_eq!(tokenizer.step().ok(), Some(Token::NUMBER(33f64)));
        assert_eq!(tokenizer.step().ok(), Some(Token::RPAREN));
        assert_eq!(tokenizer.step().ok(), Some(Token::DIV));
        assert_eq!(tokenizer.step().ok(), Some(Token::NUMBER(7.5f64)));
        assert_eq!(tokenizer.step().ok(), None);
    }

    #[test]
    fn test_into_iter() {
        let tokenizer = Tokenizer::new("1 + 2*(510   - 33 )  / 7 ");
        let tokens: Vec<Token> = tokenizer.into_iter().collect();

        assert_eq!(
            tokens,
            [
                Token::NUMBER(1f64),
                Token::PLUS,
                Token::NUMBER(2f64),
                Token::MUL,
                Token::LPAREN,
                Token::NUMBER(510f64),
                Token::MINUS,
                Token::NUMBER(33f64),
                Token::RPAREN,
                Token::DIV,
                Token::NUMBER(7f64),
            ]
        );
    }

    #[test]
    fn test_parse_indentity() {
        let mut tokenizer = Tokenizer::new("e + 12 * -1.2e10 * pi - my_var");

        assert_eq!(tokenizer.step().ok(), Some(Token::E));
        assert_eq!(tokenizer.step().ok(), Some(Token::PLUS));
        assert_eq!(tokenizer.step().ok(), Some(Token::NUMBER(12f64)));
        assert_eq!(tokenizer.step().ok(), Some(Token::MUL));
        assert_eq!(tokenizer.step().ok(), Some(Token::MINUS));
        assert_eq!(tokenizer.step().ok(), Some(Token::NUMBER(1.2f64)));
        assert_eq!(tokenizer.step().ok(), Some(Token::E));
        assert_eq!(tokenizer.step().ok(), Some(Token::NUMBER(10f64)));
        assert_eq!(tokenizer.step().ok(), Some(Token::MUL));
        assert_eq!(tokenizer.step().ok(), Some(Token::PI));
        assert_eq!(tokenizer.step().ok(), Some(Token::MINUS));
        assert_eq!(tokenizer.step().ok(), Some(Token::ID("my_var".to_string())));
        assert_eq!(tokenizer.step().ok(), None);
    }
}
