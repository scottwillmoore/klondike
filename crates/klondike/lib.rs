use std::collections::VecDeque;

use card::*;

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

    // pub fn iter(&self) -> impl Iterator<Item = Option<Card>> {
    //     self.piles
    //         .iter()
    //         .enumerate()
    //         .map(|(i, pile)| pile.top_rank().map(|rank| Card::new(rank, i as Suit)))
    // }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stock {
    cards: VecDeque<Card>,
}

impl Stock {
    fn new(cards: &[Card]) -> Stock {
        assert_eq!(cards.len(), 24);

        Stock {
            cards: cards.iter().copied().collect(),
        }
    }

    pub fn cards(&self) -> (&[Card], &[Card]) {
        self.cards.as_slices()
    }

    pub fn top_card(&self) -> Option<&Card> {
        self.cards.front()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableauPile {
    cards: Vec<Card>,
    face_up_bottom_index: usize,
}

impl TableauPile {
    fn new(cards: &[Card]) -> TableauPile {
        TableauPile {
            cards: cards.iter().copied().collect(),
            face_up_bottom_index: cards.len() - 1,
        }
    }

    pub fn face_down_cards(&self) -> &[Card] {
        &self.cards[..self.face_up_bottom_index]
    }

    pub fn face_up_cards(&self) -> &[Card] {
        &self.cards[self.face_up_bottom_index..]
    }
}

const TABLEAU_PILE_COUNT: usize = 7;

type TableauPiles = [TableauPile; TABLEAU_PILE_COUNT];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tableau {
    piles: TableauPiles,
}

impl Tableau {
    fn new(cards: &[Card]) -> Tableau {
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    foundation: Foundation,
    stock: Stock,
    tableau: Tableau,
}

impl Game {
    pub fn new(deck: Deck) -> Self {
        let (stock_cards, tableau_cards) = deck.cards().split_at(24);
        Game {
            foundation: Foundation::default(),
            stock: Stock::new(stock_cards),
            tableau: Tableau::new(tableau_cards),
        }
    }

    pub fn foundation(&self) -> &Foundation {
        &self.foundation
    }

    pub fn stock(&self) -> &Stock {
        &self.stock
    }

    pub fn tableau(&self) -> &Tableau {
        &self.tableau
    }
}
