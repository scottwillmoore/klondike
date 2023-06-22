mod rank;
pub use crate::rank::*;

mod suit;
pub use crate::suit::*;

mod card;
pub use crate::card::*;

mod deck;
pub use crate::deck::*;

#[cfg(test)]
mod test;

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

impl std::str::FromStr for Suit {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        parse_char(str).and_then(|char| Suit::from_char(char).ok_or(ParseError::Invalid))
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
