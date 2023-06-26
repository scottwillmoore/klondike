use std::io::{stdin, stdout, Write};

use anyhow::Result;
use card::*;
use klondike::Game;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

fn read_line(prompt: &str, line: &mut String) -> Result<()> {
    println!();

    print!("{}", prompt);
    stdout().flush()?;

    line.clear();
    stdin().read_line(line)?;

    println!();

    Ok(())
}

fn print_game(game: &Game) {
    let to_mark = |card: Card| card.to_ascii_mark();
    // let to_mark = |card: Card| card.to_mark();

    println!(
        "{}\n",
        game.foundation()
            .enumerate_piles()
            .map(|(suit, pile)| pile.top_rank().map(|rank| Card::new(rank, suit)))
            .map(|option| option.map_or("()".to_string(), |card| format!("({})", to_mark(card))))
            .collect::<Vec<String>>()
            .join(" ")
    );

    println!(
        "{}{}\n",
        game.stock()
            .bottom_cards()
            .rev()
            .map(|card| format!("{} ", to_mark(card)))
            .collect::<Vec<String>>()
            .join(""),
        game.stock()
            .top_card()
            .map_or("()".to_string(), |card| format!("({})", to_mark(card)))
    );

    for (index, pile) in game.tableau().enumerate_piles() {
        println!(
            "{}: {}({})",
            TryInto::<char>::try_into(
                TryInto::<u32>::try_into(index)
                    .map(|index| Into::<u32>::into('a') + index)
                    .unwrap()
            )
            .unwrap(),
            pile.bottom_cards()
                .rev()
                .map(|card| format!("{} ", to_mark(card)))
                .collect::<Vec<String>>()
                .join(""),
            pile.top_cards()
                .rev()
                .map(|card| to_mark(card))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

pub fn main() -> Result<()> {
    let mut line = String::new();

    read_line("seed = ", &mut line)?;
    let seed = line.trim().parse::<u64>()?;
    let mut generator = Xoshiro256PlusPlus::seed_from_u64(seed);

    let mut deck = Deck::new();
    deck.cards_mut().shuffle(&mut generator);

    let game = Game::new(deck);
    print_game(&game);

    loop {
        read_line("> ", &mut line)?;

        match line.trim().to_lowercase().as_str() {
            "q" | "quit" => break,
            "s" | "show" => print_game(&game),
            _ => {
                match line.trim().parse::<Card>() {
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
