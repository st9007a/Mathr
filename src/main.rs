pub mod ast;
pub mod token;

use std::io;

use token::Tokenizer;
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

    let tokenizer = Tokenizer::new("1 +    2 / (53 + 1122)");

    for token in tokenizer {
        println!("{:?}", token);
    }

    Ok(())
}
