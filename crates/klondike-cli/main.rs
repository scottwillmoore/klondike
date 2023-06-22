use std::io::{stdin, stdout, Result, Write};

use card::*;
use klondike::Game;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

fn help() {}

fn show(game: &Game) {
    println!(
        "{}\n",
        game.foundation()
            .enumerate_piles()
            .map(|(suit, pile)| pile.top_rank().map(|rank| Card::new(rank, suit)))
            .map(|option| option.map_or("()".to_string(), |card| format!("({})", card)))
            .collect::<Vec<String>>()
            .join(" ")
    );

    println!(
        "{}{}\n",
        game.stock()
            .bottom_cards()
            .rev()
            .map(|card| format!("{} ", card))
            .collect::<Vec<String>>()
            .join(""),
        game.stock()
            .top_card()
            .map_or("()".to_string(), |card| format!("({})", card))
    );

    for pile in game.tableau().piles() {
        println!(
            "{}({})",
            pile.bottom_cards()
                .rev()
                .map(|card| format!("{} ", card))
                .collect::<Vec<String>>()
                .join(""),
            pile.top_cards()
                .rev()
                .map(|card| format!("{}", card))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

pub fn main() -> Result<()> {
    let seed = 0;
    let mut random = Xoshiro256PlusPlus::seed_from_u64(seed);

    let mut deck = Deck::new();
    deck.cards_mut().shuffle(&mut random);

    let game = Game::new(deck);

    let mut line = String::new();

    loop {
        println!();
        print!("> ");
        stdout().flush()?;
        line.clear();
        stdin().read_line(&mut line)?;
        println!();

        let line = line.trim();
        match line.to_lowercase().as_str() {
            "h" | "help" => help(),
            "q" | "quit" => break,
            "s" | "show" => show(&game),
            _ => {
                match line.parse::<Card>() {
                    Ok(card) => {
                        println!("{} of {}s", card.rank().to_str(), card.suit().to_str());

                        println!("{:?}", game.find_card(card));
                    }
                    Err(error) => println!("Error: {}", error),
                };
            }
        }
    }

    Ok(())
}
