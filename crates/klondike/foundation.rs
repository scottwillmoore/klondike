use std::array::from_fn;

use card::{Card, Rank, Suit};
use enum_trait::Enum;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoundationPile {
    top_rank: Option<Rank>,
}

impl FoundationPile {
    fn new() -> FoundationPile {
        FoundationPile { top_rank: None }
    }

    pub fn top_rank(&self) -> &Option<Rank> {
        &self.top_rank
    }

    pub fn can_add(&self, rank: Rank) -> bool {
        self.top_rank
            .map_or_else(Rank::first, Rank::next)
            .is_some_and(|next_rank| next_rank == rank)
    }

    pub(crate) fn pop_rank(&mut self) -> Option<Rank> {
        // This is a bad name for the function.
        let rank = self.top_rank;
        self.top_rank = rank.and_then(Rank::previous);
        rank
    }

    pub(crate) fn push_rank(&mut self) {
        // This is a bad name for the function.
        // There is no check...
        self.top_rank = self.top_rank.map_or_else(Rank::first, Rank::next);
    }
}

pub type FoundationIndex = Suit;

pub const FOUNDATION_PILE_COUNT: usize = FoundationIndex::LENGTH;

pub type FoundationPiles = [FoundationPile; FOUNDATION_PILE_COUNT];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Foundation {
    piles: FoundationPiles,
}

impl Foundation {
    pub(crate) fn new() -> Foundation {
        Foundation {
            piles: from_fn(|_| FoundationPile::new()),
        }
    }

    pub fn get(&self, index: FoundationIndex) -> &FoundationPile {
        &self.piles[index.to_index()]
    }

    pub fn get_mut(&mut self, index: FoundationIndex) -> &mut FoundationPile {
        &mut self.piles[index.to_index()]
    }

    pub fn top_card(&self, index: FoundationIndex) -> Option<Card> {
        self.get(index)
            .top_rank()
            .map(|rank| Card::new(rank, index))
    }

    pub fn can_add(&self, card: Card) -> bool {
        self.get(card.suit()).can_add(card.rank())
    }

    pub fn piles(&self) -> &FoundationPiles {
        &self.piles
    }

    pub fn enumerate_piles(
        &self,
    ) -> impl DoubleEndedIterator<Item = (FoundationIndex, &FoundationPile)> {
        self.piles
            .iter()
            .enumerate()
            .map(|(index, pile)| (FoundationIndex::from_index(index).unwrap(), pile))
    }

    pub(crate) fn pop_card(&mut self, suit: Suit) -> Option<Card> {
        // This is a bad name for the function.
        let rank = self.get_mut(suit).pop_rank();
        rank.map(|rank| rank.with_suit(suit))
    }

    pub(crate) fn push_card(&mut self, card: Card) {
        // This is a bad name for the function.
        self.get_mut(card.suit()).push_rank();
    }
}
