import { enumerate } from ".";

export function* map<T, U>(iterable: Iterable<T>, map: (value: T) => U): IterableIterator<U> {
	for (const value of iterable) {
		yield map(value);
	}
}

export function* enumerateMap<T, U>(
	iterable: Iterable<T>,
	map: (value: T, index: number) => U,
): IterableIterator<U> {
	for (const [value, index] of enumerate(iterable)) {
		yield map(value, index);
	}
}
