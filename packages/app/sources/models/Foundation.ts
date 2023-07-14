import { Option } from "@klondike/utilities/option";

import { Rank, Suit } from ".";
import { Tuple } from "../utilities/types";

export interface FoundationPile {
	topRank: Option<Rank>;
}

type FoundationLength = 4;

export interface Foundation {
	piles: Tuple<FoundationPile, FoundationLength>;
}

export namespace Foundation {
	export function map<T>(
		foundation: Foundation,
		map: (pile: FoundationPile, suit: Suit) => T,
	): Tuple<T, FoundationLength> {
		return foundation.piles.map(map) as Tuple<T, FoundationLength>;
	}
}
