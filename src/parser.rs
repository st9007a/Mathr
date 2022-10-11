use crate::token::Tokenizer;

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn from_text(text: &str) -> Self {
        Self {
            tokenizer: Tokenizer::new(text),
        }
    }
}
