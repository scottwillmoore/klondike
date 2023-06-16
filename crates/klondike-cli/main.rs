use std::io::{stdin, stdout, Result, Write};

pub fn main() -> Result<()> {
    let mut line = String::new();

    loop {
        println!();
        print!("> ");
        stdout().flush()?;
        line.clear();
        stdin().read_line(&mut line)?;
        println!();

        match line.to_lowercase().trim() {
            "q" | "quit" => {
                break;
            }
            _ => {
                println!("{}", line.trim());
            }
        }
    }

    Ok(())
}
