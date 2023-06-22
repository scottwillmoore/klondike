use card::Card;

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

    pub fn bottom_cards(&self) -> impl DoubleEndedIterator<Item = Card> + '_ {
        self.cards[..self.top_bottom_index].iter().rev().copied()
    }

    pub fn top_cards(&self) -> impl DoubleEndedIterator<Item = Card> + '_ {
        self.cards[self.top_bottom_index..].iter().rev().copied()
    }
}

const TABLEAU_PILE_COUNT: usize = 7;

type TableauPiles = [TableauPile; TABLEAU_PILE_COUNT];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tableau {
    piles: TableauPiles,
}

impl Tableau {
    pub fn new(cards: &[Card]) -> Tableau {
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

    pub fn piles(&self) -> &TableauPiles {
        &self.piles
    }

    pub fn enumerate_piles(&self) -> impl DoubleEndedIterator<Item = (usize, &TableauPile)> {
        self.piles.iter().enumerate()
    }
}

impl std::ops::Index<usize> for Tableau {
    type Output = TableauPile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.piles[index]
    }
}

impl std::ops::IndexMut<usize> for Tableau {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.piles[index]
    }
}
