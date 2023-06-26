use enum_trait::Enum;

use crate::*;

#[derive(Clone, Copy, Debug, Enum, Eq, Ord, PartialEq, PartialOrd)]
pub enum Color {
    Black,
    Red,
}

pub use Color::*;

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Black => Red,
            Red => Black,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Black => 'B',
            Red => 'R',
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Black => "Black",
            Red => "Red",
        }
    }
}

impl std::convert::From<Suit> for Color {
    fn from(suit: Suit) -> Self {
        suit.color()
    }
}

impl std::convert::From<Card> for Color {
    fn from(card: Card) -> Self {
        card.suit().into()
    }
}

impl std::convert::TryFrom<char> for Color {
    type Error = ParseError;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            'B' => Ok(Black),
            'R' => Ok(Red),
            _ => Err(ParseError::Invalid),
        }
    }
}

impl std::convert::From<Color> for char {
    fn from(color: Color) -> Self {
        color.to_char()
    }
}

impl std::convert::From<Color> for &'static str {
    fn from(color: Color) -> Self {
        color.to_str()
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_str())
    }
}

impl std::str::FromStr for Color {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        parse_char(str).and_then(Color::try_from)
    }
}
