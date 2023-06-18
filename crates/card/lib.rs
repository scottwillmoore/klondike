use std::mem::MaybeUninit;

use enum_trait::Enum;

#[derive(Clone, Copy, Debug, Enum, Eq, PartialEq)]
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

pub use Rank::*;

impl Rank {
    pub fn into_char(self) -> char {
        match self {
            Ace => 'A',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            Ten => 'T',
            Jack => 'J',
            Queen => 'Q',
            King => 'K',
        }
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.into_char())
    }
}

#[derive(Clone, Copy, Debug, Enum, Eq, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

pub use Suit::*;

impl Suit {
    pub fn into_char(self) -> char {
        match self {
            Club => 'C',
            Diamond => 'D',
            Heart => 'H',
            Spade => 'S',
        }
    }

    pub fn into_usize(self) -> usize {
        self as usize
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.into_char())
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

    pub fn rank(self) -> Rank {
        self.rank
    }

    pub fn suit(self) -> Suit {
        self.suit
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}{}", self.rank, self.suit)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Deck {
    cards: [Card; 52],
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = [MaybeUninit::<Card>::uninit(); 52];

        for (i, rank) in Rank::variants().enumerate() {
            for (j, suit) in Suit::variants().enumerate() {
                cards[i * Suit::LENGTH + j].write(Card::new(rank, suit));
            }
        }

        Deck {
            cards: unsafe { std::mem::transmute(cards) },
        }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn cards_mut(&mut self) -> &mut [Card] {
        &mut self.cards
    }
}

#[cfg(test)]
mod test;
