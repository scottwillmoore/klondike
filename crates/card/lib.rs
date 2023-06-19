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
    pub fn from_char(char: char) -> Option<Self> {
        match char {
            'A' => Some(Ace),
            '2' => Some(Two),
            '3' => Some(Three),
            '4' => Some(Four),
            '5' => Some(Five),
            '6' => Some(Six),
            '7' => Some(Seven),
            '8' => Some(Eight),
            '9' => Some(Nine),
            'T' => Some(Ten),
            'J' => Some(Jack),
            'Q' => Some(Queen),
            'K' => Some(King),
            _ => None,
        }
    }

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
}

impl std::fmt::Display for Rank {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_char())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    TooShort,
    Invalid,
    TooLong,
}

fn parse_char(str: &str) -> Result<char, ParseError> {
    let mut chars = str.chars();
    match (chars.next(), chars.next()) {
        (None, _) => Err(ParseError::TooShort),
        (Some(char), None) => Ok(char),
        _ => Err(ParseError::TooLong),
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                ParseError::TooShort => "too few characters",
                ParseError::Invalid => "invalid characters",
                ParseError::TooLong => "too many characters",
            }
        )
    }
}

impl std::str::FromStr for Rank {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        parse_char(str).and_then(|char| Rank::from_char(char).ok_or(ParseError::Invalid))
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
}

impl std::fmt::Display for Suit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_char())
    }
}

impl std::str::FromStr for Suit {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        parse_char(str).and_then(|char| Suit::from_char(char).ok_or(ParseError::Invalid))
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

impl std::convert::From<(Rank, Suit)> for Card {
    fn from(from: (Rank, Suit)) -> Self {
        Card::new(from.0, from.1)
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}{}", self.rank, self.suit)
    }
}

impl std::str::FromStr for Card {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut chars = str.chars();
        match (chars.next(), chars.next(), chars.next()) {
            (None, _, _) => Err(ParseError::TooShort),
            (Some(rank_char), Some(suit_char), None) => {
                let rank = Rank::from_char(rank_char);
                let suit = Suit::from_char(suit_char);
                rank.zip(suit).map(Into::into).ok_or(ParseError::Invalid)
            }
            _ => Err(ParseError::TooLong),
        }
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

impl std::fmt::Display for Deck {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            self.cards
                .iter()
                .map(|card| format!("{}", card))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod test;
