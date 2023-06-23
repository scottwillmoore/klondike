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
    pub fn from_char(char: char) -> Option<Self> {
        match char {
            'C' => Some(Club),
            'D' => Some(Diamond),
            'H' => Some(Heart),
            'S' => Some(Spade),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Club => 'C',
            Diamond => 'D',
            Heart => 'H',
            Spade => 'S',
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

impl std::convert::TryFrom<char> for Suit {
    type Error = ParseError;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        Self::from_char(char).ok_or(ParseError::Invalid)
    }
}

impl std::convert::From<Suit> for char {
    fn from(suit: Suit) -> Self {
        suit.to_char()
    }
}

impl std::convert::From<Suit> for &'static str {
    fn from(suit: Suit) -> Self {
        suit.to_str()
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_char())
    }
}

impl std::str::FromStr for Suit {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        parse_char(str).and_then(Suit::try_from)
    }
}
