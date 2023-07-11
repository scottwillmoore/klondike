export function all<T>(iterable: Iterable<T>, predicate: (value: T) => boolean): boolean {
	for (const value of iterable) {
		if (!predicate(value)) {
			return false;
		}
	}
	return true;
}

export function any<T>(iterable: Iterable<T>, predicate: (value: T) => boolean): boolean {
	for (const value of iterable) {
		if (predicate(value)) {
			return true;
		}
	}
	return false;
}

export function max(iterable: Iterable<number>): number {
	return Math.max(...iterable);
}

export function min(iterable: Iterable<number>): number {
	return Math.min(...iterable);
}

export function product(iterable: Iterable<number>): number {
	let product = 1;
	for (const value of iterable) {
		product *= value;
	}
	return product;
}

export function sum(iterable: Iterable<number>): number {
	let sum = 0;
	for (const value of iterable) {
		sum += value;
	}
	return sum;
}

export function toArray<T>(iterable: Iterable<T>): T[] {
	return Array.from(iterable);
}

export function toSet<T>(iterable: Iterable<T>): Set<T> {
	return new Set(iterable);
}

export function toMap<K, V>(iterable: Iterable<[K, V]>): Map<K, V> {
	return new Map(iterable);
}

export function toObject<K extends PropertyKey, V>(iterable: Iterable<[K, V]>): Record<K, V> {
	return Object.fromEntries(iterable) as Record<K, V>;
}
