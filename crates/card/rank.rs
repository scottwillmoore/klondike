use enum_trait::Enum;

#[derive(Clone, Copy, Debug, Enum, Eq, PartialEq)]
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

impl Rank {
    pub fn from_char(char: char) -> Option<Self> {
        match char {
            'A' => Some(Ace),
            '2' => Some(Two),
            '3' => Some(Three),
            '4' => Some(Four),
            '5' => Some(Five),
            '6' => Some(Six),
            '7' => Some(Seven),
            '8' => Some(Eight),
            '9' => Some(Nine),
            'T' => Some(Ten),
            'J' => Some(Jack),
            'Q' => Some(Queen),
            'K' => Some(King),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
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

    pub fn to_str(self) -> &'static str {
        match self {
            Ace => "Ace",
            Two => "Two",
            Three => "Three",
            Four => "Four",
            Five => "Five",
            Six => "Six",
            Seven => "Seven",
            Eight => "Eight",
            Nine => "Nine",
            Ten => "Ten",
            Jack => "Jack",
            Queen => "Queen",
            King => "King",
        }
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_char())
    }
}
