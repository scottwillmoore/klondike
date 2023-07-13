export function next(): IteratorReturnResult<undefined>;
export function next<T>(value: T): IteratorReturnResult<T>;
export function next<T>(value?: T) {
	return { done: false, value };
}

export function done(): IteratorReturnResult<undefined>;
export function done<T>(value: T): IteratorReturnResult<T>;
export function done<T>(value?: T) {
	return { done: true, value };
}
