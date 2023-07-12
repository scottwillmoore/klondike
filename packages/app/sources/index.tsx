import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { greet } from "@klondike/utilities";

import { App } from "./components/App";

import "./index.css";

console.log(greet("Scott"));

const rootElement = document.getElementById("root");
if (rootElement == null) {
	throw new Error("Could not get the root element.");
}

const root = createRoot(rootElement);
root.render(
	<StrictMode>
		<App />
	</StrictMode>,
);
