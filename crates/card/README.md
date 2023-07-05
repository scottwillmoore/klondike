# `card`

## Features

- Standard 52 card deck.
- Enumerations for colors, ranks and suits.
- Iterators for colors, ranks and suits.
- Parse and serialize cards.
- ASCII and Unicode representations.
- Support `serde`?
- ...

## Goals

- Efficient.
- Fast.
- Feature-complete.
- Rust library.
- WASM library, with JavaScript/TypeScript bindings.

## References

- https://en.wikipedia.org/wiki/Standard_52-card_deck

### Libraries

My research yielded the following card libraries. Most of the libraries are not great in both implementation and API design. Any marked with a `!` are not bad. I have only looked for standalone card libraries, but I expect there are many more projects that include cards as a component of their larger purpose, such as a poker library.

- https://github.com/topics/playing-cards

#### JavaScript

- https://www.npmjs.com/package/cards
- https://www.npmjs.com/package/deck-of-cards
- https://www.npmjs.com/package/deckofcards
- https://www.npmjs.com/package/plain-cards
- ! https://www.npmjs.com/package/typedeck

#### Python

- https://pypi.org/project/deck-of-cards/
- https://pypi.org/project/pydealer/

#### Rust

- https://crates.io/crates/card_deck
- https://crates.io/crates/cardpack
- https://crates.io/crates/cards
- ! https://crates.io/crates/french-suited-playing-cards
- ! https://crates.io/crates/playin-cards
- https://crates.io/crates/playing-cards
- https://crates.io/crates/rust_cards
- https://crates.io/crates/simple-cards
