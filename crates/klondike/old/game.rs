use crate::card::*;

// Consider using a EnumMap for the Foundation.
// Consider using a LinkedList for the Stock.
// Consider using an ArrayVec for the Deck and Stock.

// https://docs.rs/arrayvec/latest/arrayvec/index.html
// https://docs.rs/enum-map/latest/enum_map/index.html

// FoundationIndex = Suit = {Clubs, Diamonds, Hearts, Spades}
// Foundation (piles: [FoundationPile; 4])
// FoundationPile (rank: Rank)
// Stock (cards: Vec<Card>)
// TableauIndex = 7 = {A, B, C, D, E, F, G}
// Tableau (piles: [TableauPile; 7])
// TableauPile (...)

trait Enum {
    const LENGTH: usize;

    fn from_usize(from: usize) -> Self;
    fn into_usize(self) -> usize;
}

impl Enum for Suit {
    const LENGTH: usize = 4;

    fn from_usize(from: usize) -> Self {
        match from {
            0 => Club,
            1 => Diamond,
            2 => Heart,
            3 => Spade,
            _ => unreachable!(),
        }
    }

    fn into_usize(self) -> usize {
        match self {
            Club => 0,
            Diamond => 1,
            Heart => 2,
            Spade => 3,
        }
    }
}

struct FoundationIndex(Suit);

struct FoundationPile(Rank);

#[derive(Debug)]
struct Foundation {
    piles: [Suit; 4],
}

impl Foundation {
    pub fn new() -> Foundation {
        Foundation { piles: SUITS }
    }
}

impl std::fmt::Display for Foundation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self.piles.map(|pile| format!("{}", pile)).join(" ");

        write!(formatter, "{}", cards)
    }
}

#[derive(Debug)]
struct Stock {
    cards: Vec<Card>, // Max length of 24.
    bottom_index: usize,
}

impl Stock {
    pub fn new(cards: &[Card]) -> Stock {
        Stock {
            cards: cards.to_owned(),
            bottom_index: 0,
        }
    }
}

impl std::fmt::Display for Stock {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (top_cards, bottom_cards) = self.cards.split_at(self.bottom_index);

        let (bottom_cards, top_cards, top_card) =
            if let Some((top_card, top_cards)) = top_cards.split_last() {
                (bottom_cards, top_cards, top_card)
            } else {
                let (top_card, bottom_cards) = bottom_cards.split_last().unwrap();
                (bottom_cards, top_cards, top_card)
            };

        let cards = bottom_cards
            .iter()
            .chain(top_cards.iter())
            .map(|card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(" ");

        write!(formatter, "{} ({})", cards, top_card)
    }
}

#[derive(Debug)]
struct Pile {
    cards: Vec<Card>, // Max length of n + 12. 7 + 12 = 19.
    face_up_bottom_index: usize,
}

impl Pile {
    pub fn new(cards: &[Card]) -> Pile {
        Pile {
            cards: cards.to_owned(),
            face_up_bottom_index: cards.len() - 1,
        }
    }
}

impl std::fmt::Display for Pile {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (face_down_cards, face_up_cards) = self.cards.split_at(self.face_up_bottom_index);

        let face_down = face_down_cards
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(" ");

        let face_up = face_up_cards
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(" ");

        write!(formatter, "{:<17} ({})", face_down, face_up)
    }
}

#[derive(Debug)]
struct Tableau {
    piles: [Pile; 7],
}

impl Tableau {
    pub fn new(cards: &[Card]) -> Tableau {
        let (a, cards) = cards.split_at(1);
        let (b, cards) = cards.split_at(2);
        let (c, cards) = cards.split_at(3);
        let (d, cards) = cards.split_at(4);
        let (e, cards) = cards.split_at(5);
        let (f, cards) = cards.split_at(6);
        let (g, cards) = cards.split_at(7);
        assert_eq!(cards.len(), 0);

        Tableau {
            piles: [
                Pile::new(a),
                Pile::new(b),
                Pile::new(c),
                Pile::new(d),
                Pile::new(e),
                Pile::new(f),
                Pile::new(g),
            ],
        }
    }
}

impl std::fmt::Display for Tableau {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tableau = self
            .piles
            .as_slice()
            .iter()
            .map(|pile| format!("{}", pile))
            .collect::<Vec<String>>()
            .join("\n");

        write!(formatter, "{}", tableau)
    }
}

#[derive(Debug)]
pub struct State {
    foundation: Foundation,
    stock: Stock,
    tableau: Tableau,
}

impl State {
    pub fn new(deck: &[Card]) -> State {
        assert_eq!(deck.len(), 52);

        let (stock_cards, tableau_cards) = deck.split_at(24);

        State {
            foundation: Foundation::new(),
            stock: Stock::new(stock_cards),
            tableau: Tableau::new(tableau_cards),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}\n\n{}\n\n{}",
            self.foundation, self.stock, self.tableau
        )
    }
}
