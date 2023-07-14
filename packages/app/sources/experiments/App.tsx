import { DndContext as DraggableContext } from "@dnd-kit/core";
import { FunctionComponent } from "react";

import { Card } from "./Card";
import { Pile } from "./Pile";
import { TableauPile } from "./Tableau";

import css from "./App.module.css";

export const App: FunctionComponent = () => {
	return (
		<DraggableContext>
			<div className={css.app}>
				<TableauPile>
					<Card rank="four" suit="heart" faceDown />
					<Card rank="three" suit="spade" faceDown />
					<Pile id="2D">
						<Card rank="two" suit="diamond" />
						<Pile id="AC">
							<Card rank="ace" suit="club" />
						</Pile>
					</Pile>
				</TableauPile>
			</div>
		</DraggableContext>
	);
};
