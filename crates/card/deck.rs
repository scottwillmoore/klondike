use std::mem::MaybeUninit;

use enum_trait::Enum;

use crate::{Card, Rank, Suit};

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
