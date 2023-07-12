import { UserConfig } from "vite";

import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-plugin-tsconfig-paths";

export default {
	cacheDir: "./.vite",
	plugins: [react(), tsconfigPaths()],
	test: {
		cache: {
			dir: "./.vitest",
		},
	},
} satisfies UserConfig;
