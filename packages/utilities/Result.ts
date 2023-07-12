export const noneType = "none";
export type NoneType = typeof noneType;
export type None = { type: NoneType };

export const someType = "some";
export type SomeType = typeof someType;
export type Some<T> = { type: SomeType; some: T };

export type OptionType = NoneType | SomeType;
export type Option<T> = None | Some<T>;

// Constructors

export const none = <T>(): Option<T> => {
	return { type: noneType } satisfies None;
};

export const some = <T>(some: T): Option<T> => {
	return { type: someType, some } satisfies Some<T>;
};

// Queries

export const isNone = <T>(option: Option<T>): option is None => {
	return option.type === noneType;
};

export const isSome = <T>(option: Option<T>): option is Some<T> => {
	return option.type === someType;
};

// Boolean operations

export const isSomeAnd = <T>(option: Option<T>, f: (t: T) => boolean): boolean => {
	return option.type === someType && f(option.some);
};

export const and = <T>(a: Option<T>, b: Option<T>): Option<T> => {
	return a.type === noneType ? a : b;
};

export const andThen = <T>(a: Option<T>, f: () => Option<T>): Option<T> => {
	return a.type === noneType ? a : f();
};

export const or = <T>(a: Option<T>, b: Option<T>): Option<T> => {
	return a.type === someType ? a : b;
};

export const orElse = <T>(a: Option<T>, f: () => Option<T>): Option<T> => {
	return a.type === someType ? a : f();
};

// Transform operations

export const successType = "success";
export type SuccessType = typeof successType;
export type Success<T> = { type: SuccessType; success: T };

export const failureType = "failure";
export type FailureType = typeof failureType;
export type Failure<T> = { type: FailureType; failure: T };

export type ResultType = SuccessType | FailureType;
export type Result<T, U> = Success<T> | Failure<U>;

export const success = <T, U>(success: T): Result<T, U> => {
	return { type: successType, success } satisfies Success<T>;
};

export const failure = <T, U>(failure: U): Result<T, U> => {
	return { type: failureType, failure } satisfies Failure<U>;
};
