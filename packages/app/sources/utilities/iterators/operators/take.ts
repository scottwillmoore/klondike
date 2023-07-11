import { enumerate } from "..";

export function* take<T>(iterable: Iterable<T>, amount: number): IterableIterator<T> {
	for (const [value, index] of enumerate(iterable)) {
		if (index < amount) {
			yield value;
		} else {
			return;
		}
	}
}
