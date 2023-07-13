import { expect, test } from "vitest";

import * as Null from ".";

test("none", () => {
	const none = Null.none;

	expect(none).toBeNull();

	expect(none).toBe(Null.none);

	expect(Null.isNone(none)).toBe(true);
	expect(Null.isSome(none)).toBe(false);
});

test("some(0)", () => {
	const some = Null.some(0);

	expect(some).not.toBeNull();

	expect(some).toBe(Null.some(0));

	expect(Null.isNone(some)).toBe(false);
	expect(Null.isSome(some)).toBe(true);
});

test("some({})", () => {
	const some = Null.some({});

	expect(some).not.toBeNull();

	expect(some).not.toBe(Null.some({}));
	expect(some).toStrictEqual(Null.some({}));

	expect(Null.isNone(some)).toBe(false);
	expect(Null.isSome(some)).toBe(true);
});
