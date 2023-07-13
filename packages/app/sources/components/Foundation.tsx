import { FunctionComponent } from "react";

import { Null } from "@klondike/utilities";
import { Card } from ".";
import Model from "../models";
import { ChildrenProps } from "../utilities/reactTypes";

import styles from "./Foundation.module.css";

type FoundationPileProps = ChildrenProps;

const FoundationPile: FunctionComponent<FoundationPileProps> = ({ children }) => {
	return <div className={styles.foundationPile}>{children}</div>;
};

export type FoundationProps = {
	foundation: Model.Foundation;
};

export const Foundation: FunctionComponent<FoundationProps> = ({ foundation }) => {
	return (
		<div className={styles.foundation}>
			{Model.Foundation.map(foundation, (pile, suit) => (
				// rome-ignore lint/suspicious/noArrayIndexKey:
				<FoundationPile key={suit}>
					{Null.map(pile.topRank, (topRank) => (
						<Card card={Model.Card.of(topRank, suit)} />
					))}
				</FoundationPile>
			))}
		</div>
	);
};
