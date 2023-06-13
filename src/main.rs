#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

use Rank::*;

impl std::fmt::Display for Rank {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Ace => "A",
                Two => "2",
                Three => "3",
                Four => "4",
                Five => "5",
                Six => "6",
                Seven => "7",
                Eight => "8",
                Nine => "9",
                Ten => "10",
                Jack => "J",
                Queen => "Q",
                King => "K",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

use Suit::*;

impl std::fmt::Display for Suit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Club => "♣",
                Diamond => "♦",
                Heart => "♥",
                Spade => "♠",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}{}", self.rank, self.suit)
    }
}

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let ranks = [
            //
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ];
        let suits = [
            //
            Club, Diamond, Heart, Spade,
        ];

        let mut cards = Vec::<Card>::new();
        for rank in ranks {
            for suit in suits {
                cards.push(Card::new(rank, suit));
            }
        }

        Deck { cards }
    }

    pub fn shuffle<R>(&mut self, random: &mut R)
    where
        R: Rng,
    {
        self.cards.shuffle(random)
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(", ");

        write!(formatter, "{}", cards)
    }
}

pub fn main() {
    let mut random = thread_rng();

    let mut deck = Deck::new();
    deck.shuffle(&mut random);

    println!("{}", deck);
}
