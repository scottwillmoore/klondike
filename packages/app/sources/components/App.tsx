import { FunctionComponent } from "react";

import { Foundation, Stock, Tableau } from ".";
import { unimplemented } from "../utilities";

import css from "./App.module.css";

export const App: FunctionComponent = () => {
	return (
		<div className={css.app}>
			<div className={css.stockFoundation}>
				<Stock />
				<Foundation foundation={unimplemented()} />
			</div>
			<div className={css.tableau}>
				<Tableau />
			</div>
		</div>
	);
};
