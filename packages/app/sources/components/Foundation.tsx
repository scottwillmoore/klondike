import { FunctionComponent } from "react";

import * as models from "../models";
import { Rank, Suit } from "../models";
import { ChildrenProps } from "../utilities";
import { Card } from ".";

import styles from "./Foundation.module.css";

type FoundationPileProps = ChildrenProps;

const FoundationPile: FunctionComponent<FoundationPileProps> = ({ children }) => {
	return <div className={styles.foundationPile}>{children}</div>;
};

export type FoundationProps = {
	foundation: models.Foundation;
};

export const Foundation: FunctionComponent<FoundationProps> = ({ foundation }) => {
	return (
		<div className={styles.foundation}>
			{models.Foundation.map(foundation, (pile, suit) => (
				<FoundationPile>
					{/* An `enum` can be assigned `null`, `number` and `undefined`... */}
					{/* The `topRank` property is `null`, but can be passed to `Card.of`... */}
					<Card card={models.Card.of(pile.topRank, suit)} />
				</FoundationPile>
			))}
			<FoundationPile>
				<Card card={{ rank: Rank.Ace, suit: Suit.Club }} />
			</FoundationPile>
			<FoundationPile>
				<Card card={{ rank: Rank.Ace, suit: Suit.Diamond }} faceDown />
				<Card card={{ rank: Rank.Two, suit: Suit.Diamond }} />
			</FoundationPile>
			<FoundationPile>{}</FoundationPile>
			<FoundationPile>{}</FoundationPile>
		</div>
	);
};
