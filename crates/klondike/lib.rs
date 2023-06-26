mod foundation;
pub use crate::foundation::*;

mod stock;
pub use crate::stock::*;

mod tableau;
pub use crate::tableau::*;

#[cfg(test)]
mod test;

use card::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Position {
    Foundation { index: FoundationIndex },
    Stock,
    Tableau { index: TableauIndex, depth: usize },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IndirectMove {
    card: Card,
    tableau_index: Option<TableauIndex>,
}

fn parse_tableau_char(char: char) -> Result<TableauIndex, ParseError> {
    Into::<u32>::into(char)
        .checked_sub('a'.into())
        .ok_or(ParseError::Invalid)?
        .try_into()
        .map_err(|_| ParseError::Invalid)
        .and_then(|index| {
            if index < TABLEAU_PILE_COUNT {
                Ok(index)
            } else {
                Err(ParseError::Invalid)
            }
        })
}

impl std::str::FromStr for IndirectMove {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut chars = str.chars();
        match (chars.next(), chars.next(), chars.next(), chars.next()) {
            (None, _, _, _) => Err(ParseError::TooShort),
            (_, None, _, _) => Err(ParseError::TooShort),
            (Some(rank_char), Some(suit_char), tableau_char, None) => {
                let rank = Rank::try_from(rank_char)?;
                let suit = Suit::try_from(suit_char)?;
                let card = Card::new(rank, suit);

                let tableau_index = tableau_char.map(parse_tableau_char).transpose()?;

                Ok(IndirectMove {
                    card,
                    tableau_index,
                })
            }
            _ => Err(ParseError::TooLong),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DirectMove {
    FoundationToTableau {
        from: FoundationIndex,
        to: TableauIndex,
    },
    StockToFoundation,
    StockToTableau {
        to: TableauIndex,
    },
    TableauToFoundation {
        from: TableauIndex,
    },
    TableauToTableau {
        from: TableauIndex,
        depth: usize,
        to: TableauIndex,
    },
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
            foundation: Foundation::new(),
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
        if self
            .foundation
            .get(card.suit())
            .top_rank()
            .is_some_and(|pile_rank| pile_rank == card.rank())
        {
            return Some(Position::Foundation { index: card.suit() });
        }

        if self
            .stock
            .top_card()
            .is_some_and(|stock_card| stock_card == card)
        {
            return Some(Position::Stock);
        }

        for (index, pile) in self.tableau.enumerate_piles() {
            for (depth, pile_card) in pile.top_cards().enumerate() {
                if pile_card == card {
                    return Some(Position::Tableau { index, depth });
                }
            }
        }

        None
    }

    pub fn can_indirect_move(&self, indirect_move: IndirectMove) -> bool {
        self.find_card(indirect_move.card)
            .and_then(|position| match position {
                Position::Foundation { index: from } => indirect_move.tableau_index.map(|to| {
                    assert!(to < TABLEAU_PILE_COUNT);
                    DirectMove::FoundationToTableau { from, to }
                }),

                Position::Stock => Some(indirect_move.tableau_index.map_or(
                    DirectMove::StockToFoundation,
                    |to| {
                        assert!(to < TABLEAU_PILE_COUNT);
                        DirectMove::StockToTableau { to }
                    },
                )),

                Position::Tableau { index: from, depth } => {
                    assert!(from < TABLEAU_PILE_COUNT);
                    Some(indirect_move.tableau_index.map_or(
                        DirectMove::TableauToFoundation { from },
                        |to| {
                            assert!(to < TABLEAU_PILE_COUNT);
                            assert_ne!(from, to);
                            DirectMove::TableauToTableau { from, depth, to }
                        },
                    ))
                }
            })
            .is_some_and(|direct_move| self.can_move(direct_move))
    }

    pub fn can_move(&self, direct_move: DirectMove) -> bool {
        match direct_move {
            DirectMove::FoundationToTableau { from, to } => {
                assert!(to < TABLEAU_PILE_COUNT);
                self.foundation
                    .top_card(from)
                    .is_some_and(|foundation_card| self.tableau.get(to).can_add(foundation_card))
            }

            DirectMove::StockToFoundation => self
                .stock
                .top_card()
                .is_some_and(|stock_card| self.foundation.can_add(stock_card)),

            DirectMove::StockToTableau { to } => {
                assert!(to < TABLEAU_PILE_COUNT);
                self.stock
                    .top_card()
                    .is_some_and(|stock_card| self.tableau.get(to).can_add(stock_card))
            }

            DirectMove::TableauToFoundation { from } => {
                assert!(from < TABLEAU_PILE_COUNT);
                self.tableau
                    .get(from)
                    .top_card()
                    .is_some_and(|tableau_card| self.foundation.can_add(tableau_card))
            }

            DirectMove::TableauToTableau { from, depth, to } => {
                assert!(from < TABLEAU_PILE_COUNT);
                assert!(to < TABLEAU_PILE_COUNT);
                assert_ne!(from, to);
                self.tableau
                    .get(from)
                    .top_cards()
                    .nth(depth)
                    .is_some_and(|tableau_card| self.tableau.get(to).can_add(tableau_card))
            }
        }
    }

    pub fn play_indirect_move(&mut self, indirect_move: IndirectMove) {
        todo!()
    }

    pub fn play_move(&mut self, direct_move: DirectMove) -> bool {
        match direct_move {
            DirectMove::FoundationToTableau { from, to } => {
                assert!(to < TABLEAU_PILE_COUNT);
                todo!()
            }
            DirectMove::StockToFoundation => {
                todo!()
            }
            DirectMove::StockToTableau { to } => {
                assert!(to < TABLEAU_PILE_COUNT);
                todo!()
            }
            DirectMove::TableauToFoundation { from } => {
                assert!(from < TABLEAU_PILE_COUNT);
                todo!()
            }
            DirectMove::TableauToTableau { from, depth, to } => {
                assert!(from < TABLEAU_PILE_COUNT);
                assert!(to < TABLEAU_PILE_COUNT);
                assert_ne!(from, to);
                todo!()
            }
        }
    }
}
