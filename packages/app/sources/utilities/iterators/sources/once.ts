export function* once<T>(value: T): IterableIterator<T> {
	yield value;
}
