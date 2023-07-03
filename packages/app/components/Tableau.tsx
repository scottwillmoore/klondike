import { FunctionComponent } from "react";

import { Card, ranks, suits } from "./Card";
import { TableauPile } from "./TableauPile";

import css from "./Tableau.module.css";

export const Tableau: FunctionComponent = () => {
	return (
		<div className={css.tableau}>
			<TableauPile>
				<Card rank={ranks.ten} suit={suits.spade} />
				<Card rank={ranks.jack} suit={suits.diamond} />
				<Card rank={ranks.queen} suit={suits.heart} />
				<Card rank={ranks.king} suit={suits.spade} />
			</TableauPile>
			<TableauPile></TableauPile>
			<TableauPile></TableauPile>
			<TableauPile></TableauPile>
			<TableauPile></TableauPile>
			<TableauPile></TableauPile>
			<TableauPile></TableauPile>
		</div>
	);
};
