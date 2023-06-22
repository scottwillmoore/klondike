use std::collections::VecDeque;

use card::*;
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

    pub fn cards(&self) -> impl DoubleEndedIterator<Item = Card> + '_ {
        self.cards.iter().copied()
    }

    pub fn bottom_cards(&self) -> impl DoubleEndedIterator<Item = Card> + '_ {
        self.cards.iter().skip(1).copied()
    }

    pub fn top_card(&self) -> Option<Card> {
        self.cards.front().copied()
    }
}

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

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub enum Transition {
//     TableauToTableau {
//         from: usize,
//         amount: usize,
//         to: usize,
//     },
// }

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Position {
    Stock,
    Tableau { index: usize, depth: usize },
    Foundation,
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

    pub fn find_card(&self, card: Card) -> Option<Position> {
        if self.foundation[card.suit()]
            .top_rank()
            .is_some_and(|pile_rank| pile_rank == card.rank())
        {
            return Some(Position::Foundation);
        }

        if self
            .stock
            .top_card()
            .is_some_and(|stock_card| stock_card == card)
        {
            return Some(Position::Stock);
        }

        for (index, pile) in self.tableau.piles().iter().enumerate() {
            for (depth, pile_card) in pile.top_cards().enumerate() {
                if pile_card == card {
                    return Some(Position::Tableau { index, depth });
                }
            }
        }

        None
    }

    // pub fn tableau_to_tableau(&mut self, from: usize, amount: usize, to: usize) -> Option<()> {
    //     let from = &mut self.tableau.piles[from];

    //     let new_length = from.cards.len().checked_sub(amount)?;
    //     // Can I copy the cards without an intermediate allocation?
    //     let cards = from.cards.drain(new_length..).collect::<Vec<Card>>();

    //     let to = &mut self.tableau.piles[to];
    //     to.cards.extend(cards);

    //     Some(())
    // }
}
