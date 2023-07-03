import { FunctionComponent } from "react";

import css from "./Card.module.css";

export type CardProps = {
	rank: string;
	suit: string;
};

export const Card: FunctionComponent<CardProps> = ({ rank, suit }) => {
	const mark = rank + suit;
	return (
		<div className={css.card}>
			<span className={`${css.mark} ${css.topLeft}`}>{mark}</span>
			<span className={css.suit}>{suit}</span>
			<span className={`${css.mark} ${css.bottomRight}`}>{mark}</span>
		</div>
	);
};
