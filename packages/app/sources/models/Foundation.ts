import { Rank, Suit } from ".";
import { Tuple } from "../utilities";

export interface FoundationPile {
	topRank: Rank | null;
}

type FoundationLength = 4;

export interface Foundation {
	piles: Tuple<FoundationPile, FoundationLength>;
}

export namespace Foundation {
	export function map<T>(
		foundation: Foundation,
		callback: (pile: FoundationPile, suit: Suit) => T,
	): Tuple<T, FoundationLength> {
		return foundation.piles.map(callback) as Tuple<T, FoundationLength>;
	}
}
