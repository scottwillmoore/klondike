import { enumerate } from ".";

export function* filter<T>(
	iterable: Iterable<T>,
	filter: (value: T) => boolean,
): IterableIterator<T> {
	for (const value of iterable) {
		if (filter(value)) {
			yield value;
		}
	}
}

export function* enumerateFilter<T>(
	iterable: Iterable<T>,
	filter: (value: T, index: number) => boolean,
): IterableIterator<T> {
	for (const [value, index] of enumerate(iterable)) {
		if (filter(value, index)) {
			yield value;
		}
	}
}
