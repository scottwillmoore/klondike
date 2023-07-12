// https://doc.rust-lang.org/std/option/
// https://docs.oracle.com/javase/8/docs/api/java/util/Optional.html

export type Option<T> = T | null;

export function none<T>(): Option<T> {
	return null;
}

export function some<T>(t: T): Option<T> {
	return t;
}

export function get<T>(a: Option<T>): T {
	if (a !== null) {
		return a;
	} else {
		throw new Error();
	}
}

export function getOr<T>(a: Option<T>, b: T): T {
	if (a !== null) {
		return a;
	} else {
		return b;
	}
}

export function getOrElse<T>(a: Option<T>, f: () => T): T {
	if (a !== null) {
		return a;
	} else {
		return f();
	}
}

export function getUnchecked<T>(a: Option<T>): T {
	return a as T;
}

export function or<T>(a: Option<T>, b: Option<T>): Option<T> {
	if (a !== null) {
		return a;
	} else {
		return b;
	}
}

export function orElse<T>(t: Option<T>, f: () => Option<T>): Option<T> {
	if (t !== null) {
		return t;
	} else {
		return f();
	}
}

export function map<T, U>(t: Option<T>, f: (t: T) => U): Option<U> {
	if (t !== null) {
		return f(t);
	} else {
		return null;
	}
}

export function mapOr<T, U>(t: Option<T>, u: U, f: (t: T) => U): Option<U> {
	if (t !== null) {
		return f(t);
	} else {
		return u;
	}
}

export function mapOrElse<T, U>(t: Option<T>, u: () => U, f: (t: T) => U): Option<U> {
	if (t !== null) {
		return f(t);
	} else {
		return u();
	}
}
