export function* enumerate<T>(iterable: Iterable<T>): IterableIterator<[T, number]> {
	let index = 0;
	for (const value of iterable) {
		yield [value, index++];
	}
}
