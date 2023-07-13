import { expect, test } from "vitest";

import { range } from "./sources";
import { done, next } from "./utilities";

test("range(0, 0)", () => {
	const iterable = range(0, 0);
	expect(iterable.next()).toStrictEqual(done());
});

test("range(0, 1)", () => {
	const iterable = range(0, 1);
	expect(iterable.next()).toStrictEqual(next(0));
	expect(iterable.next()).toStrictEqual(done());
});

test("range(0, 2)", () => {
	const iterable = range(0, 2);
	expect(iterable.next()).toStrictEqual(next(0));
	expect(iterable.next()).toStrictEqual(next(1));
	expect(iterable.next()).toStrictEqual(done());
});

test("range(1, 0)", () => {
	const iterable = range(1, 0);
	expect(iterable.next()).toStrictEqual(done());
});

test("range(2, 0)", () => {
	const iterable = range(2, 0);
	expect(iterable.next()).toStrictEqual(done());
});

test("range(0.0, 1.0)", () => {
	const iterable = range(0.0, 1.0);
	expect(iterable.next()).toStrictEqual(next(0.0));
	expect(iterable.next()).toStrictEqual(done());
});

test("range(0.0, 1.0)", () => {
	const iterable = range(0.0, 1.0);
	expect(iterable.next()).toStrictEqual(next(0.0));
	expect(iterable.next()).toStrictEqual(done());
});

// What if the numbers are `NaN`?
// What if the numbers are not an integer?
// What if the numbers are not finite?
// What if the numbers are the maximum integer?
// What if the numbers are the minimum integer?
