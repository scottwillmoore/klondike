// https://github.com/sindresorhus/type-fest/blob/main/source/is-literal.d.ts
// https://stackoverflow.com/a/52806744/10292952

import { IsNever, Primitive } from ".";

// prettier-ignore
type IsLiteral<T, Type extends Primitive> =
	IsNever<T> extends false
		? [T] extends [Type]
			? [Type] extends [T]
				? false
				: true
			: false
		: false;

export type IsBigIntLiteral<T> = IsLiteral<T, bigint>;

export type IsBooleanLiteral<T> = IsLiteral<T, boolean>;

export type IsNumberLiteral<T> = IsLiteral<T, number>;

export type IsStringLiteral<T> = IsLiteral<T, string>;

export type IsSymbolLiteral<T> = IsLiteral<T, symbol>;
