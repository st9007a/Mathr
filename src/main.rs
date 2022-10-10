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

    let tokenizer = Tokenizer::new("1 +    2");
    let mut iter = tokenizer.into_iter();

    for _ in 0..3 {
        match iter.next() {
            Some(token) => println!("{:?}", token),
            None => println!("None"),
        }
    }

    Ok(())
}
