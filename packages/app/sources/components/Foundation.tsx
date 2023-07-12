import { FunctionComponent } from "react";

import { Card } from ".";
import models from "../models";
import { ChildrenProps } from "../utilities/reactTypes";

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
				// rome-ignore lint/suspicious/noArrayIndexKey:
				<FoundationPile key={suit}>
					{pile.topRank === null ? null : <Card card={models.Card.of(pile.topRank, suit)} />}
				</FoundationPile>
			))}
		</div>
	);
};
