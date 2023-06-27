use card::*;
use enum_trait::Enum;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableauPile {
    cards: Vec<Card>,
    top_bottom_index: usize,
}

impl TableauPile {
    fn new(cards: &[Card]) -> TableauPile {
        TableauPile {
            cards: cards.iter().copied().collect(),
            top_bottom_index: cards.len() - 1,
        }
    }

    pub fn cards(&self) -> impl DoubleEndedIterator<Item = Card> + '_ {
        self.cards.iter().rev().copied()
    }

    pub fn top_cards(&self) -> impl DoubleEndedIterator<Item = Card> + '_ {
        self.cards[self.top_bottom_index..].iter().rev().copied()
    }

    pub fn top_card(&self) -> Option<Card> {
        self.top_cards().next()
    }

    pub fn bottom_cards(&self) -> impl DoubleEndedIterator<Item = Card> + '_ {
        self.cards[..self.top_bottom_index].iter().rev().copied()
    }

    pub fn can_add(&self, card: Card) -> bool {
        self.top_card().map_or_else(
            || card.rank() == King,
            |top_card| {
                let has_rank = top_card
                    .rank()
                    .previous()
                    .is_some_and(|previous_rank| previous_rank == card.rank());

                let has_suit = top_card.color() != card.color();

                has_rank && has_suit
            },
        )
    }

    pub(crate) fn pop_card(&mut self) -> Option<Card> {
        let new_len = self.cards.len() - 1;
        if self.top_bottom_index == new_len {
            self.top_bottom_index = self.top_bottom_index.saturating_sub(1);
        }

        self.cards.pop()
    }

    pub(crate) fn pop_cards(&mut self, depth: usize) -> Vec<Card> {
        let top_cards_len = self.cards.len() - self.top_bottom_index;
        assert!(depth < top_cards_len);
        let new_len = self.cards.len() - depth - 1;

        if self.top_bottom_index == new_len {
            // Overflow! Check this...
            self.top_bottom_index -= 1;
        }

        self.cards.drain(new_len..).collect()
    }

    pub(crate) fn push_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub(crate) fn push_cards(&mut self, cards: &[Card]) {
        self.cards.extend(cards);
    }
}

pub type TableauIndex = usize;

pub const TABLEAU_PILE_COUNT: usize = 7;

pub type TableauPiles = [TableauPile; TABLEAU_PILE_COUNT];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tableau {
    piles: TableauPiles,
}

impl Tableau {
    pub(crate) fn new(cards: &[Card]) -> Tableau {
        assert_eq!(cards.len(), 28);

        let (a, cards) = cards.split_at(1);
        let (b, cards) = cards.split_at(2);
        let (c, cards) = cards.split_at(3);
        let (d, cards) = cards.split_at(4);
        let (e, cards) = cards.split_at(5);
        let (f, cards) = cards.split_at(6);
        let (g, _) = cards.split_at(7);

        Tableau {
            piles: [
                TableauPile::new(a),
                TableauPile::new(b),
                TableauPile::new(c),
                TableauPile::new(d),
                TableauPile::new(e),
                TableauPile::new(f),
                TableauPile::new(g),
            ],
        }
    }

    pub fn get(&self, index: TableauIndex) -> &TableauPile {
        &self.piles[index]
    }

    pub(crate) fn get_mut(&mut self, index: TableauIndex) -> &mut TableauPile {
        &mut self.piles[index]
    }

    pub fn piles(&self) -> &TableauPiles {
        &self.piles
    }

    pub fn enumerate_piles(&self) -> impl DoubleEndedIterator<Item = (TableauIndex, &TableauPile)> {
        self.piles.iter().enumerate()
    }
}

/*
impl std::ops::Index<TableauIndex> for Tableau {
    type Output = TableauPile;

    fn index(&self, index: TableauIndex) -> &Self::Output {
        &self.piles[index]
    }
}

impl std::ops::IndexMut<TableauIndex> for Tableau {
    fn index_mut(&mut self, index: TableauIndex) -> &mut Self::Output {
        &mut self.piles[index]
    }
}
*/
