use enum_trait::Enum;

use crate::*;

#[derive(Clone, Copy, Debug, Enum, Eq, Ord, PartialEq, PartialOrd)]
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
    pub fn to_char(self) -> char {
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

    pub fn to_str(self) -> &'static str {
        match self {
            Ace => "Ace",
            Two => "Two",
            Three => "Three",
            Four => "Four",
            Five => "Five",
            Six => "Six",
            Seven => "Seven",
            Eight => "Eight",
            Nine => "Nine",
            Ten => "Ten",
            Jack => "Jack",
            Queen => "Queen",
            King => "King",
        }
    }

    pub fn with_suit(self, suit: Suit) -> Card {
        Card::new(self, suit)
    }
}

impl std::convert::From<Card> for Rank {
    fn from(card: Card) -> Self {
        card.rank()
    }
}

impl std::convert::TryFrom<char> for Rank {
    type Error = ParseError;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            'A' => Ok(Ace),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            'T' => Ok(Ten),
            'J' => Ok(Jack),
            'Q' => Ok(Queen),
            'K' => Ok(King),
            _ => Err(ParseError::Invalid),
        }
    }
}

impl std::convert::From<Rank> for char {
    fn from(rank: Rank) -> Self {
        rank.to_char()
    }
}

impl std::convert::From<Rank> for &'static str {
    fn from(rank: Rank) -> Self {
        rank.to_str()
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_str())
    }
}

impl std::str::FromStr for Rank {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        parse_char(str).and_then(Rank::try_from)
    }
}
