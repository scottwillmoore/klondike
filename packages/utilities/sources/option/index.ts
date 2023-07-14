export type None = null;

export const none: None = null;

export function isNone<T>(option: Option<T>): option is None {
	return option === none;
}

export type Some<T> = T;

export function some<T>(value: T): Option<T> {
	return value;
}

export function isSome<T>(option: Option<T>): option is Some<T> {
	return !isNone(option);
}

export type Option<T> = Some<T> | None;

export * from "./operators";

export * as default from ".";
