import { FunctionComponent } from "react";

import { Rank, Suit } from "../models";
import { Card } from "./Card";
import { FoundationPile } from "./FoundationPile";

import css from "./Foundation.module.css";

export const Foundation: FunctionComponent = () => {
	return (
		<div className={css.foundation}>
			<FoundationPile>
				<Card card={{ rank: Rank.Ace, suit: Suit.Club }} />
			</FoundationPile>
			<FoundationPile>
				<Card card={{ rank: Rank.Ace, suit: Suit.Diamond }} faceDown />
				<Card card={{ rank: Rank.Two, suit: Suit.Diamond }} />
			</FoundationPile>
			<FoundationPile></FoundationPile>
			<FoundationPile></FoundationPile>
		</div>
	);
};
