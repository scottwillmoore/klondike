import { cx as classNames } from "classix";
import { FunctionComponent } from "react";

import styles from "./Card.module.css";

export type Color = "black" | "red";

export type Rank =
	| "ace"
	| "two"
	| "three"
	| "four"
	| "five"
	| "six"
	| "seven"
	| "eight"
	| "nine"
	| "ten"
	| "jack"
	| "queen"
	| "king";

const rankToCharacter: Record<Rank, string> = {
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
};

export type Suit = "club" | "diamond" | "heart" | "spade";

const suitToColor: Record<Suit, Color> = {
	club: "black",
	diamond: "red",
	heart: "red",
	spade: "black",
};

const suitToCharacter: Record<Suit, string> = {
	club: "♣",
	diamond: "♦",
	heart: "♥",
	spade: "♠",
};

export type CardProps = {
	rank: Rank;
	suit: Suit;

	faceDown?: boolean;
};

export const Card: FunctionComponent<CardProps> = ({ rank, suit, faceDown = false }) => {
	const color = suitToColor[suit];

	const rankCharacter = rankToCharacter[rank];
	const suitCharacter = suitToCharacter[suit];

	const id = rankCharacter + suitCharacter;

	return (
		<div
			className={classNames(
				styles.card,
				color === "black" ? styles.cardBlack : styles.cardRed,
				faceDown ? styles.cardFaceDown : styles.cardFaceUp,
			)}
		>
			<span className={classNames(styles.id, styles.idTopLeft)}>{id}</span>
			<span className={styles.suit}>{suitCharacter}</span>
			<span className={classNames(styles.id, styles.idBottomRight)}>{id}</span>
		</div>
	);
};
