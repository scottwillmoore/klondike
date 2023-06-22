use card::{Rank, Suit};
use enum_trait::Enum;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FoundationPile {
    top_rank: Option<Rank>,
}

impl FoundationPile {
    pub fn top_rank(&self) -> Option<Rank> {
        self.top_rank
    }
}

const FOUNDATION_PILE_COUNT: usize = 4;

type FoundationPiles = [FoundationPile; FOUNDATION_PILE_COUNT];

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Foundation {
    piles: FoundationPiles,
}

impl Foundation {
    pub fn piles(&self) -> &FoundationPiles {
        &self.piles
    }

    pub fn enumerate_piles(&self) -> impl DoubleEndedIterator<Item = (Suit, &FoundationPile)> {
        self.piles
            .iter()
            .enumerate()
            .map(|(i, pile)| (Suit::from_index(i).unwrap(), pile))
    }
}

impl std::ops::Index<Suit> for Foundation {
    type Output = FoundationPile;

    fn index(&self, index: Suit) -> &Self::Output {
        &self.piles[index.into_index()]
    }
}

impl std::ops::IndexMut<Suit> for Foundation {
    fn index_mut(&mut self, index: Suit) -> &mut Self::Output {
        &mut self.piles[index.into_index()]
    }
}
