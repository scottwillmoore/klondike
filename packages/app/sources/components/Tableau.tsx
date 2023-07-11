import { FunctionComponent } from "react";

import { Rank, Suit } from "../models";
import { ChildrenProps } from "../utilities/reactTypes";
import { Card } from "./Card";

import css from "./Tableau.module.css";

export type TableauPileProps = ChildrenProps;

export const TableauPile: FunctionComponent<TableauPileProps> = ({ children }) => {
	return <div className={css.tableauPile}>{children}</div>;
};

export type TableauProps = {};

export const Tableau: FunctionComponent<TableauProps> = () => {
	return (
		<div className={css.tableau}>
			<TableauPile>
				<Card card={{ rank: Rank.Ten, suit: Suit.Spade }} faceDown />
				<Card card={{ rank: Rank.Jack, suit: Suit.Diamond }} faceDown />
				<Card card={{ rank: Rank.King, suit: Suit.Spade }} />
				<Card card={{ rank: Rank.Queen, suit: Suit.Heart }} />
			</TableauPile>
			<TableauPile>{}</TableauPile>
			<TableauPile>{}</TableauPile>
			<TableauPile>{}</TableauPile>
			<TableauPile>{}</TableauPile>
			<TableauPile>{}</TableauPile>
			<TableauPile>{}</TableauPile>
		</div>
	);
};
