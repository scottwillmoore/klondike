use card::Card;
use std::collections::VecDeque;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stock {
    cards: VecDeque<Card>,
}

impl Stock {
    pub(crate) fn new(cards: &[Card]) -> Stock {
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

    pub(crate) fn try_remove(&mut self, card: Card) -> Result<(), ()> {
        if self.top_card().is_some_and(|top_card| top_card == card) {
            self.cards.pop_front();
            Ok(())
        } else {
            Err(())
        }
    }
}
