export function* map<T, U>(iterable: Iterable<T>, map: (value: T) => U): IterableIterator<U> {
	for (const value of iterable) {
		yield map(value);
	}
}
