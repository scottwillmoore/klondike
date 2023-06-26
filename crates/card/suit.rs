use enum_trait::Enum;

use crate::*;

#[derive(Clone, Copy, Debug, Enum, Eq, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

pub use Suit::*;

impl Suit {
    pub fn color(self) -> Color {
        match self {
            Club => Black,
            Diamond => Red,
            Heart => Red,
            Spade => Black,
        }
    }

    pub fn to_ascii_char(self) -> char {
        match self {
            Club => 'C',
            Diamond => 'D',
            Heart => 'H',
            Spade => 'S',
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Club => '♣',
            Diamond => '♦',
            Heart => '♥',
            Spade => '♠',
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Club => "Club",
            Diamond => "Diamond",
            Heart => "Heart",
            Spade => "Spade",
        }
    }

    pub fn with_rank(self, rank: Rank) -> Card {
        Card::new(rank, self)
    }
}

impl std::convert::From<Card> for Suit {
    fn from(card: Card) -> Self {
        card.suit()
    }
}

impl std::convert::TryFrom<char> for Suit {
    type Error = ParseError;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            'C' => Ok(Club),
            'D' => Ok(Diamond),
            'H' => Ok(Heart),
            'S' => Ok(Spade),
            _ => Err(ParseError::Invalid),
        }
    }
}

impl std::convert::From<Suit> for char {
    fn from(suit: Suit) -> Self {
        suit.to_ascii_char()
    }
}

impl std::convert::From<Suit> for &'static str {
    fn from(suit: Suit) -> Self {
        suit.to_str()
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_str())
    }
}

impl std::str::FromStr for Suit {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        parse_char(str).and_then(Suit::try_from)
    }
}
