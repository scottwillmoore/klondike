use crate::{Rank, Suit};

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
