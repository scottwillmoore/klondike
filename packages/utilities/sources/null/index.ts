export type None = null;

export const none: None = null;

export function isNone<T>(n: Null<T>): n is None {
	return n === none;
}

export type Some<T> = T;

export function some<T>(t: T): Null<T> {
	return t;
}

export function isSome<T>(n: Null<T>): n is Some<T> {
	return !isNone(n);
}

export type Null<T> = Some<T> | None;

export * from "./operators";
