import { useDroppable } from "@dnd-kit/core";
import { FunctionComponent } from "react";

import { cx } from "classix";
import { ChildrenProps } from "../utilities/ChildrenProps";
import css from "./_Pile.module.css";

export type PileProps = ChildrenProps & {
	id: string;
};

export const Pile: FunctionComponent<PileProps> = ({ children, id }) => {
	const { isOver, setNodeRef } = useDroppable({ id });
	return (
		<div className={cx(css.stack, isOver && css.isOver)} ref={setNodeRef}>
			{children}
		</div>
	);
};
