use std::io::{stdin, stdout, Result, Write};

use card::{Card, Deck};
use klondike::Game;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

fn show(game: &Game) {
    println!(
        "{}\n",
        game.foundation()
            .cards()
            .map(|option| option.map_or("()".to_string(), |card| format!("({})", card)))
            .collect::<Vec<String>>()
            .join(" ")
    );

    println!(
        "{}{}\n",
        game.stock()
            .bottom_cards()
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
            pile.face_down_cards()
                .iter()
                .map(|card| format!("{} ", card))
                .collect::<Vec<String>>()
                .join(""),
            pile.face_up_cards()
                .iter()
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
            "q" | "quit" => {
                break;
            }
            "s" | "show" => {
                show(&game);
            }
            _ => match line.parse::<Card>() {
                Ok(card) => println!(
                    "Card: {} of {}s",
                    card.rank().to_str(),
                    card.suit().to_str()
                ),
                Err(error) => println!("Error: {}", error),
            },
        }
    }

    Ok(())
}
