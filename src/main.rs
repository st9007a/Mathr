pub mod ast;
pub mod error;
pub mod interpreter;
pub mod parser;
pub mod symbol_table;
pub mod token;
pub mod tokenizer;

use std::io;
use std::io::Write;

use interpreter::Interpreter;

fn main() -> io::Result<()> {
    let mut interpreter = Interpreter::new();

    loop {
        let mut buffer = String::new();

        print!(">>> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;

        buffer.pop();

        match interpreter.interpret(&buffer) {
            Ok(value) => println!("{}", value),
            Err(err) => println!("{}", err),
        }
    }
}
