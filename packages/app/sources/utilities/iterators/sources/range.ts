export function* range(from: number, to: number): IterableIterator<number> {
	while (from < to) {
		// rome-ignore lint/style/noParameterAssign:
		yield from++;
	}
}
