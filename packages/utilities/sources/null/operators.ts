import { Null, isNone, isSome } from ".";

export function and<T>(n: Null<T>, m: Null<T>): Null<T> {
	if (isNone(n)) {
		return n;
	} else {
		return m;
	}
}

export function andThen<T>(n: Null<T>, f: () => Null<T>): Null<T> {
	if (isNone(n)) {
		return n;
	} else {
		return f();
	}
}

export function get<T>(n: Null<T>): T {
	if (isSome(n)) {
		return n;
	} else {
		throw new TypeError();
	}
}

export function getOr<T>(n: Null<T>, t: T): T {
	if (isSome(n)) {
		return n;
	} else {
		return t;
	}
}

export function getOrElse<T>(n: Null<T>, f: () => T): T {
	if (isSome(n)) {
		return n;
	} else {
		return f();
	}
}

export function getUnchecked<T>(n: Null<T>): T {
	return n as T;
}

export function inspect<T>(n: Null<T>, f: (t: T) => void): void {
	if (isSome(n)) {
		f(n);
	}
}

export function map<T, U>(n: Null<T>, f: (t: T) => U): Null<U> {
	if (isSome(n)) {
		return f(n);
	} else {
		return n;
	}
}

export function mapOr<T, U>(n: Null<T>, f: (t: T) => U, u: U): U {
	if (isSome(n)) {
		return f(n);
	} else {
		return u;
	}
}

export function mapOrElse<T, U>(n: Null<T>, f: (t: T) => U, g: () => U): U {
	if (isSome(n)) {
		return f(n);
	} else {
		return g();
	}
}

export function or<T>(n: Null<T>, m: Null<T>): Null<T> {
	if (isSome(n)) {
		return n;
	} else {
		return m;
	}
}

export function orElse<T>(n: Null<T>, f: () => Null<T>): Null<T> {
	if (isSome(n)) {
		return n;
	} else {
		return f();
	}
}
