import { FunctionComponent } from "react";

import { ChildrenProps } from "../utilities/ChildrenProps";

import css from "./TableauPile.module.css";

export type TableauPileProps = ChildrenProps;

export const TableauPile: FunctionComponent<TableauPileProps> = ({
	children,
}) => {
	return <div className={css.tableauPile}>{children}</div>;
};
