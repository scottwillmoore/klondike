import { cx as classNames } from "classix";
import { FunctionComponent } from "react";

import * as models from "../models";

import styles from "./Card.module.css";

export type CardProps = {
	card: models.Card;
	faceDown?: boolean;
};

export const Card: FunctionComponent<CardProps> = ({
	card,
	faceDown = false,
}) => {
	const color = models.Card.toColor(card);
	const identifier = models.Card.toIdentifier(card);
	const suit = models.Suit.toCharacter(card.suit);

	return (
		<div
			className={classNames(
				styles.card,
				color === models.Color.Black ? styles.cardBlack : styles.cardRed,
				faceDown ? styles.cardFaceDown : styles.cardFaceUp
			)}
		>
			<span className={classNames(styles.identifier, styles.identifierTopLeft)}>
				{identifier}
			</span>
			<span className={styles.suit}>{suit}</span>
			<span
				className={classNames(styles.identifier, styles.identifierBottomRight)}
			>
				{identifier}
			</span>
		</div>
	);
};
