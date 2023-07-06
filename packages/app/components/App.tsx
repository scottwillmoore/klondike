import { FunctionComponent } from "react";

import { Foundation } from "./Foundation";
import { Stock } from "./Stock";
import { Tableau } from "./Tableau";

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
