pub mod ast;
pub mod error;
pub mod parser;
pub mod token;

use std::io;

use parser::Parser;
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

    let mut parser = Parser::from_text("1 * 2");
    let val = parser.parse().unwrap().eval();

    println!("{}", val);

    Ok(())
}
