import { FunctionComponent } from "react";

import { Card } from "./Card";

import css from "./App.module.css";

const ranks = {
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

const suits = {
	club: "♣",
	diamond: "♦",
	heart: "♥",
	spade: "♠",
};

export const App: FunctionComponent = () => {
	return (
		<div className={css.app}>
			<Card rank={ranks.two} suit={suits.spade} />
		</div>
	);
};
