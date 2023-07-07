import { FunctionComponent } from "react";

import { Foundation, Stock, Tableau } from ".";

import css from "./App.module.css";

export const App: FunctionComponent = () => {
	return (
		<div className={css.app}>
			<div className={css.stockFoundation}>
				<Stock />
				<Foundation />
			</div>
			<div className={css.tableau}>
				<Tableau />
			</div>
		</div>
	);
};
