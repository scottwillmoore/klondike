import { DndContext } from "@dnd-kit/core";
import { FunctionComponent } from "react";

import { Card, ranks, suits } from "./Card";
import { Stack } from "./Stack";

import css from "./App.module.css";

export const App: FunctionComponent = () => {
	const stacks = ["a", "b"];
	return (
		<DndContext>
			<div className={css.app}>
				<Stack id="a">
					<Card rank={ranks.ten} suit={suits.spade} />
					<Card rank={ranks.jack} suit={suits.diamond} />
					<Card rank={ranks.queen} suit={suits.heart} />
					<Card rank={ranks.king} suit={suits.spade} />
				</Stack>
				<Stack id="b" />
				<Card rank={ranks.two} suit={suits.spade} />
			</div>
		</DndContext>
	);
};
