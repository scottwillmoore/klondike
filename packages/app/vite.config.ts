import { UserConfig } from "vite";

import createReactPlugin from "@vitejs/plugin-react";

export default {
	cacheDir: "./.vite",
	plugins: [createReactPlugin()],
} satisfies UserConfig;
