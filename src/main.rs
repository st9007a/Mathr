pub mod ast;
pub mod error;
pub mod parser;
pub mod token;
pub mod interpreter;

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
        interpreter.interpret(&buffer);
    }
}
