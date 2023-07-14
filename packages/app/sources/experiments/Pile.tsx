import { useDraggable } from "@dnd-kit/core";
import { Transform } from "@dnd-kit/utilities";
import { clsx as classNames } from "clsx";
import { FunctionComponent } from "react";

import { ChildrenProps } from "../utilities/reactTypes";
import * as N from "./Nullable";
import { pipe } from "./pipe";

import styles from "./Pile.module.css";

export type PileProps = ChildrenProps & {
	id: string;

	disabled?: boolean;
};

const toCSSTransform = (transform: Transform): string => {
	return `translate(${transform.x}px, ${transform.y}px)`;
};

export const Pile: FunctionComponent<PileProps> = ({ children, id, disabled = false }) => {
	const {
		attributes,
		isDragging: grabbed,
		listeners,
		setNodeRef,
		transform,
	} = useDraggable({ id, disabled });

	const classes = classNames({
		[styles.pile!]: true,
		[styles.pileDisabled!]: disabled,
		[styles.pileGrabbed!]: grabbed,
	});

	const inlineStyles = {
		transform: pipe(transform, N.map(toCSSTransform), N.toUndefinable),
	};

	return (
		<div className={classes} ref={setNodeRef} style={inlineStyles} {...attributes} {...listeners}>
			{children}
		</div>
	);
};
