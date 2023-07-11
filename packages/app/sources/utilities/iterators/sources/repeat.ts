export function* repeat<T>(value: T): IterableIterator<T> {
	while (true) {
		yield value;
	}
}
