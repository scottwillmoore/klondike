# Klondike

> The best information about the winnability of Klondike concerns a modified version of the game called "Thoughtful Solitaire" or "Thoughtful Klondike", in which the location of all 52 cards is known.

## Notations

With chess, there is a [standard notation](<https://en.wikipedia.org/wiki/Algebraic_notation_(chess)>) also known as algebraic notation which is used to record the moves in a game of chess. There does not appear to be a standard notation for Klondike.

### Ideas

`!`, `$`, `%`, `&`, `+`, `,`, `,`, `/`, `:`, `;`, `=`, `?`, `@`, `^`, `~`.

#### Compact (human-readable)

A card is represented in uppercase with a rank and a suit. A rank is `A`, `2`, ... `9`, `T`, `J`, `Q`, or `K`. A suit is `C`, `D`, `H` or `S`. A suit could also be `♣`, `♦`, `♥`, or `♠` instead. A position is represented in lowercase which indicates the destination of the card. The tableau piles are `a`, `b`, `c`, `d`, `e`, `f`, or `g`. The foundation is `z`. A foundation could also just use no symbol? A complete move is represented as a card, followed by a position. For example, `4Sa` is move the four of spades to the first pile in the tableau and `ACz` is move the ace of clubs to the foundation. The only other move is to draw a card from the stock which is represented by a ``? A win is indicated by a `#`? A loss is indicated by a `!`.

#### Efficient (machine-readable)

A binary format? Provide a derived description of the move, which includes all information! For example, it would indicate where the card is from and where the card is going to. It might include what the card is, but this might not be required? In addition, tableau moves would include how many cards are being moved.

## References

### Solitaire

#### Overview

- https://en.wikipedia.org/wiki/Solitaire
- https://en.wikipedia.org/wiki/Klondike_(solitaire)
- https://en.wikipedia.org/wiki/Playing_cards_in_Unicode

#### Implementation

- https://github.com/ShootMe/MinimalKlondike
- https://github.com/chrisbouchard/klondike-rs
- https://github.com/nielssp/csol
- https://github.com/brianstrauch/solitaire-tui

### Random Number Generation

- https://docs.rs/rand/latest/rand/
- https://docs.rs/rand_xoshiro/latest/rand_xoshiro/
- https://prng.di.unimi.it/
- https://rust-random.github.io/book/guide-rngs.html

### Procedural Macros

- https://doc.rust-lang.org/reference/procedural-macros.html
- https://developerlife.com/2022/03/30/rust-proc-macro/#what-are-procedural-macros
- https://blog.jetbrains.com/rust/2022/03/18/procedural-macros-under-the-hood-part-i/
- https://blog.jetbrains.com/rust/2022/07/07/procedural-macros-under-the-hood-part-ii/
- https://gist.github.com/Kestrer/8c05ebd4e0e9347eb05f265dfb7252e1
- https://github.com/dtolnay/proc-macro2
- https://github.com/dtolnay/quote
- https://github.com/dtolnay/syn
- https://github.com/xfix/enum-map/tree/master/enum-map-derive
