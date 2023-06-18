use std::io::{stdin, stdout, Result, Write};

use card::Deck;
use klondike::Game;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

fn print_game(game: &Game) {
    println!("{:?}", game.foundation().piles());
    println!("{:?}", game.stock().cards());
    println!("{:?}", game.tableau().piles());
}

pub fn main() -> Result<()> {
    let seed = 0;
    let mut random = Xoshiro256PlusPlus::seed_from_u64(seed);

    let mut deck = Deck::new();
    deck.cards_mut().shuffle(&mut random);

    let mut game = Game::new(deck);

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
                print_game(&game);
            }
        }
    }

    Ok(())
}
