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

    pub fn can_deal(&self) -> bool {
        self.cards.len() > 0
    }

    pub(crate) fn deal(&mut self) {
        self.cards.rotate_left(1);
    }

    pub(crate) fn pop_card(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }
}
