pub mod ast;
pub mod error;
pub mod parser;
pub mod token;

use std::io;

use parser::Parser;
use token::{Token, Tokenizer};
// use std::io::Write;

fn main() -> io::Result<()> {
    // loop {
    //     let mut buffer = String::new();
    //
    //     print!(">>> ");
    //     io::stdout().flush()?;
    //     io::stdin().read_line(&mut buffer)?;
    //
    //     buffer.pop();
    //
    //     println!("{}", buffer);
    // }

    let _tokens: Vec<Token> = Tokenizer::new("1 + 2").into_iter().collect();

    let mut parser = Parser::from_text("11 * (2 + 3)");
    let val = parser.parse().unwrap().eval();

    println!("{}", val);

    Ok(())
}
