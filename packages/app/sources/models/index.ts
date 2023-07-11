export * from "./Card";
export * from "./Color";
export * from "./Foundation";
export * from "./Rank";
export * from "./Stock";
export * from "./Suit";
export * from "./Tableau";

export * as default from ".";

import { Foundation, Stock, Tableau } from "./";

export interface State {
	foundation: Foundation;
	stock: Stock;
	tableau: Tableau;
}
