use crate::*;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

impl std::convert::From<(Suit, Rank)> for Card {
    fn from(from: (Suit, Rank)) -> Self {
        Card::new(from.1, from.0)
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
                let rank = Rank::try_from(rank_char).ok();
                let suit = Suit::try_from(suit_char).ok();
                rank.zip(suit).map(Into::into).ok_or(ParseError::Invalid)
            }
            _ => Err(ParseError::TooLong),
        }
    }
}
