import { FunctionComponent } from "react";

import { Rank, Suit } from "../models";
import { Card } from "./Card";
import { TableauPile } from "./TableauPile";

import css from "./Tableau.module.css";

export const Tableau: FunctionComponent = () => {
	return (
		<div className={css.tableau}>
			<TableauPile>
				<Card card={{ rank: Rank.Ten, suit: Suit.Spade }} faceDown />
				<Card card={{ rank: Rank.Jack, suit: Suit.Diamond }} faceDown />
				<Card card={{ rank: Rank.King, suit: Suit.Spade }} />
				<Card card={{ rank: Rank.Queen, suit: Suit.Heart }} />
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
