# Klondike

An implementation of [Klondike](<https://en.wikipedia.org/wiki/Klondike_(solitaire)>), the most popular version of [Solitaire](https://en.wikipedia.org/wiki/Solitaire), in [Rust](https://www.rust-lang.org/)! While this project should support traditional Klondike, the aim is to support the less popular, but more skillful, ["Thoughtful Klondike"](<https://en.wikipedia.org/wiki/Klondike_(solitaire)#Probability_of_winning>).

> The best information about the winnability of Klondike concerns a modified version of the game called "Thoughtful Solitaire" or "Thoughtful Klondike", in which the location of all 52 cards is known.

## Features

- A Rust library which implements a [standard 52-card deck](https://en.wikipedia.org/wiki/Standard_52-card_deck).
- A Rust library which implements the game of Klondike.
- A ["notation"](https://en.wikipedia.org/wiki/Chess_notation) for the game of Klondike.
- A Rust library which implements a ["Klondike engine"](https://en.wikipedia.org/wiki/Chess_engine).
- A CLI application.
- A TUI application?
- A [WASM](https://webassembly.org/) binary that can be embedded in a web application.
- A TypeScript library which interfaces with the WASM binary.
- A GUI web application written in TypeScript which uses [React](https://react.dev/).

## Rules

...

### Definitions

**Klondike**: ...

**Solitaire**: ...

**Foundation**: ...

**Foundation pile**: ...

**Stock**: ...

**Tableau**: ...

**Tableau pile**: ...

## Notations

In chess, there is a [standard notation](<https://en.wikipedia.org/wiki/Algebraic_notation_(chess)>) also known as algebraic notation which is used to record the moves in a game of chess. However, in Klondike there does not appear to be a standard notation. A notation is important as it allows games to be represented in literature, and act as a format that can be transferred and processed by applications.

I have proposed a notation for Klondike that is very similar to the notation used in chess. The notation is still subject to change and hence not yet rigorously specified. It is also (at the moment) only implemented by this application. Two different, but similar notations have been described, with one suited for human-use and the other suited to computer-use.

### Compact (human-readable)

A card is represented in uppercase with a rank and a suit. A rank is `A`, `2`, ... `9`, `T`, `J`, `Q`, or `K`. A suit is `C`, `D`, `H` or `S`. A suit could also be `♣`, `♦`, `♥`, or `♠` instead. A position is represented in lowercase which indicates the destination of the card. The tableau piles are `a`, `b`, `c`, `d`, `e`, `f`, or `g`. The foundation is `z`. A foundation could also just use no symbol? A complete move is represented as a card, followed by a position. For example, `4Sa` is move the four of spades to the first pile in the tableau and `ACz` is move the ace of clubs to the foundation. The only other move is to draw a card from the stock which is represented by a ``? A win is indicated by a `#`? A loss is indicated by a `!`.

### Efficient (machine-readable)

A binary format? Provide a derived description of the move, which includes all information! For example, it would indicate where the card is from and where the card is going to. It might include what the card is, but this might not be required? In addition, tableau moves would include how many cards are being moved.

## References

### Rust

- https://rust-lang.github.io/api-guidelines/naming.html

#### WASM

- https://rustwasm.github.io/
- https://rustwasm.github.io/docs/wasm-pack/
- https://rustwasm.github.io/docs/wasm-bindgen/
- https://surma.dev/things/rust-to-webassembly/
- https://github.com/rustwasm/awesome-rust-and-webassembly
- https://github.com/rustwasm/wasm-bindgen/issues/2073
- https://wasmtime.dev/
- https://github.com/bytecodealliance/wit-bindgen
- https://wasmer.io/
- https://wasmer.io/posts/WAI-is-the-answer
- https://github.com/wasmerio/wai
- https://wapm.io/
- https://www.fermyon.com/blog/webassembly-component-model
- https://github.com/bytecodealliance/wasm-tools
- https://github.com/bytecodealliance/jco
- https://github.com/Menci/vite-plugin-wasm
- https://github.com/WebAssembly/wabt

### Random Number Generation

- https://docs.rs/rand/latest/rand/
- https://docs.rs/rand_xoshiro/latest/rand_xoshiro/
- https://prng.di.unimi.it/
- https://rust-random.github.io/book/guide-rngs.html

### Solitaire

- https://en.wikipedia.org/wiki/Solitaire
- https://en.wikipedia.org/wiki/Klondike_(solitaire)
- https://en.wikipedia.org/wiki/Playing_cards_in_Unicode

#### Implementation

- https://github.com/ShootMe/MinimalKlondike
- https://github.com/chrisbouchard/klondike-rs
- https://github.com/nielssp/csol
- https://github.com/brianstrauch/solitaire-tui
