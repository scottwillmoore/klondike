import { Option, isNone, isSome } from ".";

export function and<T>(option: Option<T>, and: Option<T>): Option<T> {
	if (isNone(option)) {
		return option;
	} else {
		return and;
	}
}

export function andThen<T>(option: Option<T>, andThen: () => Option<T>): Option<T> {
	if (isNone(option)) {
		return option;
	} else {
		return andThen();
	}
}

export function get<T>(option: Option<T>): T {
	if (isSome(option)) {
		return option;
	} else {
		throw new TypeError();
	}
}

export function getOr<T>(option: Option<T>, or: T): T {
	if (isSome(option)) {
		return option;
	} else {
		return or;
	}
}

export function getOrElse<T>(option: Option<T>, orElse: () => T): T {
	if (isSome(option)) {
		return option;
	} else {
		return orElse();
	}
}

export function getUnchecked<T>(option: Option<T>): T {
	return option as T;
}

export function inspect<T>(option: Option<T>, inspect: (value: T) => void): void {
	if (isSome(option)) {
		inspect(option);
	}
}

export function map<T, U>(option: Option<T>, map: (value: T) => U): Option<U> {
	if (isSome(option)) {
		return map(option);
	} else {
		return option;
	}
}

export function mapOr<T, U>(option: Option<T>, map: (value: T) => U, or: U): U {
	if (isSome(option)) {
		return map(option);
	} else {
		return or;
	}
}

export function mapOrElse<T, U>(option: Option<T>, map: (value: T) => U, orElse: () => U): U {
	if (isSome(option)) {
		return map(option);
	} else {
		return orElse();
	}
}

export function or<T>(option: Option<T>, or: Option<T>): Option<T> {
	if (isSome(option)) {
		return option;
	} else {
		return or;
	}
}

export function orElse<T>(option: Option<T>, orElse: () => Option<T>): Option<T> {
	if (isSome(option)) {
		return option;
	} else {
		return orElse();
	}
}
