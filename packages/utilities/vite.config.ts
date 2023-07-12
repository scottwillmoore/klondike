import { UserConfig } from "vite";

import dts from "vite-plugin-dts";
import tsconfigPaths from "vite-plugin-tsconfig-paths";

export default {
	cacheDir: "./.vite",
	build: {
		lib: {
			entry: "./sources/index.ts",
			fileName: "index",
			name: "utilities",
			formats: ["cjs", "es"],
		},
		sourcemap: true,
	},
	plugins: [
		dts({
			exclude: ["./sources/**/*.test.ts", "./sources/**/*.test-d.ts"],
		}),
		tsconfigPaths(),
	],
	test: {
		cache: {
			dir: "./.vitest",
		},
	},
} satisfies UserConfig;
