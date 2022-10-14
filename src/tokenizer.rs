use crate::error::InterpreterError;
use crate::token::Token;

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

    pub fn try_collect(&mut self) -> Result<Vec<Token>, InterpreterError> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            match self.next() {
                Ok(token) => tokens.push(token),
                Err(err) => match err {
                    InterpreterError::EOF => {
                        break;
                    }
                    _ => {
                        return Err(err);
                    }
                },
            }
        }

        Ok(tokens)
    }

    pub fn next(&mut self) -> Result<Token, InterpreterError> {
        self.skip_char();
        self.skip_comment();

        self.peek_char()
            .ok_or(InterpreterError::EOF)
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

    fn skip_comment(&mut self) {
        if let Some(ch) = self.peek_char() {
            if ch == '#' {
                self.next_char();

                while let Some(ch) = self.next_char() {
                    if ch == '\n' {
                        break;
                    }
                }
            }
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

#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer};

    #[test]
    fn test_try_collect() {
        let mut tokenizer =
            Tokenizer::new("x = 1 + 2*(510   - 33 )  / 7.5 + (e * my_var) # Some comment");

        let token_result = tokenizer.try_collect();

        assert!(token_result.is_ok());

        let tokens = token_result.unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::ID("x".to_string()),
                Token::ASSIGN,
                Token::NUMBER(1.),
                Token::PLUS,
                Token::NUMBER(2.),
                Token::MUL,
                Token::LPAREN,
                Token::NUMBER(510.),
                Token::MINUS,
                Token::NUMBER(33.),
                Token::RPAREN,
                Token::DIV,
                Token::NUMBER(7.5),
                Token::PLUS,
                Token::LPAREN,
                Token::E,
                Token::MUL,
                Token::ID("my_var".to_string()),
                Token::RPAREN,
            ]
        );
    }
}
