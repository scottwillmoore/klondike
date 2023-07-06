import { FunctionComponent } from "react";

import { Card, ranks, suits } from "./Card";
import { FoundationPile } from "./FoundationPile";

import css from "./Foundation.module.css";

export const Foundation: FunctionComponent = () => {
	return (
		<div className={css.foundation}>
			<FoundationPile>
				<Card rank={ranks.ace} suit={suits.club} />
			</FoundationPile>
			<FoundationPile>
				<Card rank={ranks.ace} suit={suits.diamond} faceDown={false} />
				<Card rank={ranks.two} suit={suits.diamond} />
			</FoundationPile>
			<FoundationPile></FoundationPile>
			<FoundationPile></FoundationPile>
		</div>
	);
};
