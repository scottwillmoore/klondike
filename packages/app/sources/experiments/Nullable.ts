export type Nullable<A> = A | null;

export type Undefinable<A> = A | undefined;

export function isNull<A>(nullable: Nullable<A>): nullable is null {
	return nullable === null;
}

export function isNotNull<A>(nullable: Nullable<A>): nullable is A {
	return !isNull(nullable);
}

function _map<A, B>(nullable: Nullable<A>, operation: (value: A) => B): Nullable<B> {
	if (isNotNull(nullable)) {
		return operation(nullable);
	} else {
		return nullable;
	}
}

export function map<A, B>(operation: (value: A) => B): (nullable: Nullable<A>) => Nullable<B> {
	return (nullable) => _map(nullable, operation);
}

export function toUndefinable<A>(nullable: Nullable<A>): Undefinable<A> {
	if (isNotNull(nullable)) {
		return nullable;
	} else {
		return undefined;
	}
}

export * as default from "./Nullable";
