use rand::seq::SliceRandom;
use rand::Rng;

use crate::card::*;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let ranks = [
            //
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ];
        let suits = [
            //
            Club, Diamond, Heart, Spade,
        ];

        let mut cards = Vec::<Card>::new();
        for rank in ranks {
            for suit in suits {
                cards.push(Card::new(rank, suit));
            }
        }

        Deck { cards }
    }

    pub fn shuffle<R>(&mut self, random: &mut R)
    where
        R: Rng,
    {
        self.cards.shuffle(random)
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(", ");

        write!(formatter, "{}", cards)
    }
}
