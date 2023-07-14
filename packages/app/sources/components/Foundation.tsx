import O from "@klondike/utilities/option";
import { FunctionComponent } from "react";

import { Card } from ".";
import M from "../models";
import { ChildrenProps } from "../utilities/reactTypes";

import styles from "./Foundation.module.css";

type FoundationPileProps = ChildrenProps;

const FoundationPile: FunctionComponent<FoundationPileProps> = ({ children }) => {
	return <div className={styles.foundationPile}>{children}</div>;
};

export type FoundationProps = {
	foundation: M.Foundation;
};

export const Foundation: FunctionComponent<FoundationProps> = ({ foundation }) => {
	return (
		<div className={styles.foundation}>
			{M.Foundation.map(foundation, (pile, suit) => (
				// rome-ignore lint/suspicious/noArrayIndexKey:
				<FoundationPile key={suit}>
					{O.map(pile.topRank, (topRank) => (
						<Card card={M.Card.of(topRank, suit)} />
					))}
				</FoundationPile>
			))}
		</div>
	);
};
