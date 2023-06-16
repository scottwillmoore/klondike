use crate::card::*;

struct FoundationIndex;

struct TableauIndex;

enum Destination {
    Foundation,
    Tableau(TableauIndex),
}

enum HumanMove {
    Draw,
    Move { card: Card, to: Destination },
}

// The destination could be omitted, but then the move could be ambiguous.
// In addition, an omitted destination is used to move to the foundation.
// It could still work though, whereby a destination is only appended to remove ambiguity.

enum HumanMove {
    Draw,
    AmbiguousMove { card: Card },
    Move { card: Card, to: Destination },
}

enum ComputerMove {
    Draw,
    StockToFoundation,
    StockToTableau {
        to: TableauIndex,
    },
    TableauToTableau {
        from: TableauIndex,
        length: usize,
        to: TableauIndex,
    },
    TableauToFoundation {
        from: TableauIndex,
        to: Suit,
    },
}
