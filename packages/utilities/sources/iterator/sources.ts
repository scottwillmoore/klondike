export function* none(): IterableIterator<never> {}

export function* once<T>(value: T): IterableIterator<T> {
	yield value;
}

export function* range(from: number, to: number): IterableIterator<number> {
	while (from < to) {
		// rome-ignore lint/style/noParameterAssign:
		yield from++;
	}
}

export function* repeat<T>(value: T): IterableIterator<T> {
	while (true) {
		yield value;
	}
}
