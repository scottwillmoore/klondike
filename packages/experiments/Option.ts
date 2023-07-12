// Type

export type OptionT<T> = T | null;

// Constructors

export function none<T>(): OptionT<T> {
	return null;
}

export function some<T>(t: T): OptionT<T> {
	return t;
}

// Methods

export function and<T>(a: OptionT<T>, b: OptionT<T>): OptionT<T> {
	return a === null ? a : b;
}

export function andThen<T>(a: OptionT<T>, f: () => OptionT<T>): OptionT<T> {
	return a === null ? a : f();
}

export function or<T>(a: OptionT<T>, b: OptionT<T>): OptionT<T> {
	return a !== null ? a : b;
}

export function orElse<T>(a: OptionT<T>, f: () => OptionT<T>): OptionT<T> {
	return a !== null ? a : f();
}

// Class

export class OptionC<T> {
	// Constructors

	public static some<T>(t: T): OptionC<T> {
		return new OptionC(some(t));
	}

	public static none<T>(): OptionC<T> {
		return new OptionC(none());
	}

	protected option: OptionT<T>;

	protected constructor(option: OptionT<T>) {
		this.option = option;
	}

	// Methods

	public and(b: OptionC<T>): OptionC<T> {
		return new OptionC(and(this.option, b.option));
	}

	public or(b: OptionC<T>): OptionC<T> {
		return new OptionC(or(this.option, b.option));
	}
}
