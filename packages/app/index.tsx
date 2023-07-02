import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import "./index.css";

const rootElement = document.getElementById("root");
if (rootElement == null) {
	throw new Error("Could not get the root element.");
}

const root = createRoot(rootElement);
root.render(
	<StrictMode>
		<h1>Hello, world!</h1>
	</StrictMode>
);
