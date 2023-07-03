import { FunctionComponent } from "react";

import { ChildrenProps } from "../utilities/ChildrenProps";

import css from "./FoundationPile.module.css";

export type FoundationProps = ChildrenProps;

export const FoundationPile: FunctionComponent<FoundationProps> = ({
	children,
}) => {
	return <div className={css.foundationPile}>{children}</div>;
};
