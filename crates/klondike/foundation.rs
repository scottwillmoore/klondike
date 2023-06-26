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
}
