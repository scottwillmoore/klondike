use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Color {
    Black,
    Red,
}

pub use Color::*;

#[wasm_bindgen]
pub fn color_to_char(color: Color) -> char {
    match color {
        Black => 'B',
        Red => 'R',
    }
}

// pub fn color_to_str(self) -> &'static str {
//     match self {
//         Black => "Black",
//         Red => "Red",
//     }
// }

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

pub use Rank::*;

// NOTE: `impl` for `enum` is not supported by `wasm_bindgen`...
// #[wasm_bindgen]
// impl Rank {
//     pub fn to_char(self) -> char {
//         match self {
//             Ace => 'A',
//             Two => '2',
//             Three => '3',
//             Four => '4',
//             Five => '5',
//             Six => '6',
//             Seven => '7',
//             Eight => '8',
//             Nine => '9',
//             Ten => 'T',
//             Jack => 'J',
//             Queen => 'Q',
//             King => 'K',
//         }
//     }
// }

#[wasm_bindgen]
pub fn rank_to_char(rank: Rank) -> char {
    match rank {
        Ace => 'A',
        Two => '2',
        Three => '3',
        Four => '4',
        Five => '5',
        Six => '6',
        Seven => '7',
        Eight => '8',
        Nine => '9',
        Ten => 'T',
        Jack => 'J',
        Queen => 'Q',
        King => 'K',
    }
}

// NOTE: Cannot return `&'static str`, therefore must allocate an entire `String`...
// #[wasm_bindgen]
// pub fn to_string(rank: Rank) -> String {
//     match rank {
//         Ace => "Ace",
//         Two => "Two",
//         Three => "Three",
//         Four => "Four",
//         Five => "Five",
//         Six => "Six",
//         Seven => "Seven",
//         Eight => "Eight",
//         Nine => "Nine",
//         Ten => "Ten",
//         Jack => "Jack",
//         Queen => "Queen",
//         King => "King",
//     }
//     .to_string()
// }

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

pub use Suit::*;

#[wasm_bindgen]
pub fn suit_color(suit: Suit) -> Color {
    match suit {
        Club => Black,
        Diamond => Red,
        Heart => Red,
        Spade => Black,
    }
}

#[wasm_bindgen]
pub fn suit_to_ascii_char(suit: Suit) -> char {
    match suit {
        Club => 'C',
        Diamond => 'D',
        Heart => 'H',
        Spade => 'S',
    }
}

#[wasm_bindgen]
pub fn suit_to_char(suit: Suit) -> char {
    match suit {
        Club => '♣',
        Diamond => '♦',
        Heart => '♥',
        Spade => '♠',
    }
}

// pub fn suit_to_str(suit: Suit) -> &'static str {
//     match suit {
//         Club => "Club",
//         Diamond => "Diamond",
//         Heart => "Heart",
//         Spade => "Spade",
//     }
// }

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

#[wasm_bindgen]
impl Card {
    #[wasm_bindgen(constructor)]
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn color(self) -> Color {
        // self.suit.into()
        suit_color(self.suit)
    }

    pub fn rank(self) -> Rank {
        self.rank
    }

    pub fn suit(self) -> Suit {
        self.suit
    }
}
