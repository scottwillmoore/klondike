import { filter, map } from ".";

export class Stream<T> {
	protected iterable: Iterable<T>;

	public static of<T>(iterable: Iterable<T>): Stream<T> {
		return new Stream(iterable);
	}

	protected constructor(iterable: Iterable<T>) {
		this.iterable = iterable;
	}

	public filter(f: (t: T) => boolean): Stream<T> {
		this.iterable = filter(this.iterable, f);
		return this;
	}

	public map<U>(f: (t: T) => U): Stream<U> {
		const that = this as Stream<unknown> as Stream<U>;
		that.iterable = map(this.iterable, f);
		return that;
	}

	public toArray(): T[] {
		return [...this.iterable];
	}
}
