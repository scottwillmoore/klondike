import { test, assert } from "vitest";
import { range } from "./range";

function next(): IteratorReturnResult<undefined>;
function next<T>(value: T): IteratorReturnResult<T>;
function next<T>(value?: T) {
	return { done: false, value };
}

function done(): IteratorReturnResult<undefined>;
function done<T>(value: T): IteratorReturnResult<T>;
function done<T>(value?: T) {
	return { done: true, value };
}

test("range(0, 0)", () => {
	const iterable = range(0, 0);
	assert.deepStrictEqual(iterable.next(), done());
});

test("range(0, 1)", () => {
	const iterable = range(0, 1);
	assert.deepStrictEqual(iterable.next(), next(0));
	assert.deepStrictEqual(iterable.next(), done());
});

test("range(0, 2)", () => {
	const iterable = range(0, 2);
	assert.deepStrictEqual(iterable.next(), next(0));
	assert.deepStrictEqual(iterable.next(), next(1));
	assert.deepStrictEqual(iterable.next(), done());
});

test("range(1, 0)", () => {
	const iterable = range(1, 0);
	assert.deepStrictEqual(iterable.next(), done());
});

test("range(2, 0)", () => {
	const iterable = range(2, 0);
	assert.deepStrictEqual(iterable.next(), done());
});

// What if the numbers are `NaN`?
// What if the numbers are not an integer?
// What if the numbers are not finite?
// What if the numbers are the maximum integer?
// What if the numbers are the minimum integer?
