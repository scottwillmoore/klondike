import { FunctionComponent } from "react";

import { Card, ranks, suits } from "./Card";
import css from "./Stock.module.css";

export const Stock: FunctionComponent = () => {
	return (
		<div className={css.stock}>
			<Card rank={ranks.seven} suit={suits.spade} />
			<Card rank={ranks.five} suit={suits.heart} />
			<Card rank={ranks.eight} suit={suits.club} />
		</div>
	);
};
