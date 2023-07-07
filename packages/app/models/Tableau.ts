import { Card } from "./";

export interface TableauPile {
	cards: Card[];
	top_bottom_index: number;
}

export interface Tableau {
	piles: TableauPile[];
	// piles: Tuple<TableauPile, 7>;
}
