import { FunctionComponent } from "react";

import { Foundation, Stock, Tableau } from ".";
import models, { Rank } from "../models";

import css from "./App.module.css";

const foundation: models.Foundation = {
	piles: [{ topRank: Rank.Ace }, { topRank: Rank.Two }, { topRank: null }, { topRank: null }],
};

export const App: FunctionComponent = () => {
	return (
		<div className={css.app}>
			<div className={css.stockFoundation}>
				<Stock />
				<Foundation foundation={foundation} />
			</div>
			<div className={css.tableau}>
				<Tableau />
			</div>
		</div>
	);
};
