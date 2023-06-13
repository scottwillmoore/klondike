use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use klondike::*;

pub fn main() {
    let mut deck = deck();

    let seed = 0;
    let mut random = Xoshiro256PlusPlus::seed_from_u64(seed);
    deck.shuffle(&mut random);

    let state = State::new(&deck);

    println!("{}", state);
}
