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

pub use Rank::*;

#[rustfmt::skip]
pub const RANKS: [Rank; 13] = [
    Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
];

impl Rank {
    pub fn as_str(self) -> &'static str {
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
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

pub use Suit::*;

#[rustfmt::skip]
pub const SUITS: [Suit; 4] = [
    Club, Diamond, Heart, Spade,
];

impl Suit {
    pub fn as_str(self) -> &'static str {
        match self {
            Club => "♣",
            // Club => "C",
            Diamond => "♦",
            // Diamond => "D",
            Heart => "♥",
            // Heart => "H",
            Spade => "♠",
            // Spade => "S",
        }
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
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

pub fn deck() -> Vec<Card> {
    let mut cards = Vec::with_capacity(52);

    for rank in RANKS {
        for suit in SUITS {
            cards.push(Card::new(rank, suit));
        }
    }

    cards
}
