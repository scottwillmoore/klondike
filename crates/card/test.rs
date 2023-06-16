use crate::*;

#[test]
fn test_size_of() {
    assert_eq!(std::mem::size_of::<Rank>(), 1);
    assert_eq!(std::mem::size_of::<Suit>(), 1);
    assert_eq!(std::mem::size_of::<Card>(), 2);
}

#[test]
fn test_rank() {
    assert_eq!(Ace as u8, 0);
    assert_eq!(King as u8, 12);

    for (i, rank) in RANKS.into_iter().enumerate() {
        assert_eq!(i, rank as usize);
    }
}

#[test]
fn test_suit() {
    assert_eq!(Club as u8, 0);
    assert_eq!(Spade as u8, 3);

    for (i, suit) in SUITS.into_iter().enumerate() {
        assert_eq!(i, suit as usize);
    }
}

#[test]
fn test_card() {
    let card = Card::new(Ace, Club);
    assert_eq!(card.rank(), Ace);
    assert_eq!(card.suit(), Club);

    let card = Card::new(King, Spade);
    assert_eq!(card.rank(), King);
    assert_eq!(card.suit(), Spade);
}

#[test]
fn test_deck() {
    let deck = deck();
    assert_eq!(deck.len(), 52)
}
