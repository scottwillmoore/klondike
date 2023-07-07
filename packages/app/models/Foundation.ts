import { Rank } from "./";

export interface FoundationPile {
	topRank: Rank | null;
}

export interface Foundation {
	piles: FoundationPile[];
	// piles: Tuple<FoundationPile, 4>;
}

export namespace Foundation {
	// export function map(foundation: Foundation) {}
}
