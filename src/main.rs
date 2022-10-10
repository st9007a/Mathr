use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();

        print!(">>> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;

        buffer.pop();

        println!("{}", buffer);
    }
}
