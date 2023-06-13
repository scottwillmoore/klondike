use std::io::{stdin, stdout, Result, Write};

use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use klondike::*;

pub fn main() -> Result<()> {
    let mut deck = deck();

    let seed = 0;
    let mut random = Xoshiro256PlusPlus::seed_from_u64(seed);
    deck.shuffle(&mut random);

    let state = State::new(&deck);

    loop {
        println!();
        print!("> ");
        stdout().flush()?;
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        println!();

        match line.to_lowercase().trim() {
            "q" => {
                return Ok(());
            }
            _ => {
                println!("{}", state);
            }
        }
    }
}
