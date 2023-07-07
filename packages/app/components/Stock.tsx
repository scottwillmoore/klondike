import { FunctionComponent } from "react";

import { Rank, Suit } from "../models";
import { Card } from "./Card";

import css from "./Stock.module.css";

export type StockProps = {};

export const Stock: FunctionComponent<StockProps> = () => {
	return (
		<div className={css.stock}>
			<Card card={{ rank: Rank.Seven, suit: Suit.Spade }} faceDown />
			<Card card={{ rank: Rank.Five, suit: Suit.Heart }} faceDown />
			<Card card={{ rank: Rank.Eight, suit: Suit.Club }} />
		</div>
	);
};
