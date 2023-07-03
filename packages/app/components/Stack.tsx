import { useDroppable } from "@dnd-kit/core";
import { FunctionComponent } from "react";

import { cx } from "classix";
import { ChildrenProps } from "../utilities/ChildrenProps";
import css from "./Stack.module.css";

export type StackProps = ChildrenProps & {
	id: string;
};

export const Stack: FunctionComponent<StackProps> = ({ children, id }) => {
	const { isOver, setNodeRef } = useDroppable({ id });
	return (
		<div className={cx(css.stack, isOver && css.isOver)} ref={setNodeRef}>
			{children}
		</div>
	);
};
