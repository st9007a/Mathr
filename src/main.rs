pub mod ast;
pub mod error;
pub mod parser;
pub mod token;

use std::io;
use std::io::Write;

use parser::Parser;

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();

        print!(">>> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;

        buffer.pop();

        let mut parser = Parser::from_text(&buffer);

        match parser.parse() {
            Ok(node) => println!("{}", node.eval()),
            Err(err) => println!("{}", err),
        }
    }
}
