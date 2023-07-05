import { cx } from "classix";
import { FunctionComponent } from "react";

import css from "./Card.module.css";

export const ranks = {
	ace: "A",
	two: "2",
	three: "3",
	four: "4",
	five: "5",
	six: "6",
	seven: "7",
	eight: "8",
	nine: "9",
	ten: "T",
	jack: "J",
	queen: "Q",
	king: "K",
} as const;

export type Rank = (typeof ranks)[keyof typeof ranks];

export const suits = {
	club: "♣",
	diamond: "♦",
	heart: "♥",
	spade: "♠",
} as const;

export type Suit = (typeof suits)[keyof typeof suits];

export const colors = {
	red: "red",
	black: "black",
} as const;

export type Color = (typeof colors)[keyof typeof colors];

export const suitToColor: Record<Suit, Color> = {
	[suits.club]: colors.black,
	[suits.diamond]: colors.red,
	[suits.heart]: colors.red,
	[suits.spade]: colors.black,
};

export type CardProps = {
	rank: Rank;
	suit: Suit;
};

export const Card: FunctionComponent<CardProps> = ({ rank, suit }) => {
	const id = rank + suit;

	const color = suitToColor[suit];

	const classNames = cx(
		css.card,
		color === colors.black && css.card_black,
		color === colors.red && css.card_red
	);

	return (
		<div className={classNames}>
			<span className={cx(css.id, css.id_topLeft)}>{id}</span>
			<span className={css.suit}>{suit}</span>
			<span className={cx(css.id, css.id_bottomRight)}>{id}</span>
		</div>
	);
};