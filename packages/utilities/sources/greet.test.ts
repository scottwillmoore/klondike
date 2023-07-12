import { assert, test } from "vitest";

import { greet } from "./greet";

test("greet", () => {
	assert.strictEqual(greet("Scott"), "Hello, Scott!");
});
