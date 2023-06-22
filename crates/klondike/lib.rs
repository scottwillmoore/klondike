mod foundation;
pub use crate::foundation::*;

mod stock;
pub use crate::stock::*;

mod tableau;
pub use crate::tableau::*;

use card::{Card, Deck};

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
