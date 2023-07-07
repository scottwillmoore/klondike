export type Tuple<
	T,
	TLength extends number,
	TArray extends unknown[] = []
> = TArray extends { length: TLength }
	? TArray
	: Tuple<T, TLength, [...TArray, T]>;

export enum Color {
	Black = 0,
	Red = 1,
}

export enum Rank {
	Ace = 0,
	Two = 1,
	Three = 2,
	Four = 3,
	Five = 4,
	Six = 5,
	Seven = 6,
	Eight = 7,
	Nine = 8,
	Ten = 9,
	Jack = 10,
	Queen = 11,
	King = 12,
}

export enum Suit {
	Club = 0,
	Diamond = 1,
	Heart = 2,
	Spade = 3,
}

export interface Card {
	rank: Rank;
	suit: Suit;
}

export interface FoundationPile {
	topRank: Rank | null;
}

export interface Foundation {
	piles: Tuple<FoundationPile, 4>;
}

export interface Stock {
	cards: Card[];
}

export interface TableauPile {
	cards: Card[];
	top_bottom_index: number;
}

export interface Tableau {
	piles: Tuple<TableauPile, 7>;
}

export interface State {
	foundation: Foundation;
	stock: Stock;
	tableau: Tableau;
}

// Should these be classes, or stay as interfaces?
// Interfaces are pure data, therefore can be serialized easily.
// Most of the logic should be implemented on the Rust-side of the project.
// We only really want a data-exchange format. Not data-logic on the JavaScript-side of the project.
// React will accept the pure data. A transformation from state to view.
// What logic would even be needed on the JavaScript-side of the project.
// Any query is just a WASM function call?
// Performance at the WASM boundary may be improved.
// It doesn't have to be a serialization of JS or JSON data.
// There could be a diffing algorithm? Only exchange the minimum amount.
