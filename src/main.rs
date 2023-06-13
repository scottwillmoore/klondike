use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use klondike::*;

pub fn main() {
    let seed = 1;
    let mut random = Xoshiro256PlusPlus::seed_from_u64(seed);

    let mut deck = Deck::new();
    deck.shuffle(&mut random);

    println!("{}", deck);
}
