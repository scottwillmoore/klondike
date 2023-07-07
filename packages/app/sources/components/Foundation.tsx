import { FunctionComponent } from "react";

// import * as models from "../models";
import { Rank, Suit } from "../models";
import { ChildrenProps } from "../utilities";
import { Card } from "./Card";

import css from "./Foundation.module.css";

type FoundationPileProps = ChildrenProps;

const FoundationPile: FunctionComponent<FoundationPileProps> = ({
	children,
}) => {
	return <div className={css.foundationPile}>{children}</div>;
};

export type FoundationProps = {
	// foundation: models.Foundation;
};

export const Foundation: FunctionComponent<FoundationProps> = () => {
	return (
		<div className={css.foundation}>
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
