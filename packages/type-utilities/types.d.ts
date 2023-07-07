// https://github.com/sindresorhus/type-fest

// any.d.ts

// https://github.com/sindresorhus/type-fest/blob/main/source/is-any.d.ts
// https://stackoverflow.com/a/49928360/7349225
export type IsAny<T> = 0 extends 1 & T ? true : false;

// never.d.ts

// https://github.com/microsoft/TypeScript/issues/31751#issuecomment-498526919
// https://github.com/sindresorhus/type-fest/blob/main/source/is-never.d.ts
// https://stackoverflow.com/a/53984913/7349225
// https://www.zhenghao.io/posts/ts-never#how-to-check-for-never
export type IsNever<T> = [T] extends [never] ? true : false;

// unknown.d.ts

// https://github.com/sindresorhus/type-fest/blob/main/source/is-unknown.d.ts
export type IsUnknown<T> = unknown extends T ? (IsAny<T> extends false ? true : false) : false;

// bigint.d.ts

type IsType<T, Type> = IsNever<T> extends false ? (T extends Type ? true : false) : false;

export type IsBigInt<T> = IsNever<T> extends false ? (T extends bigint ? true : false) : false;

// type Boolean = boolean;

export type IsBoolean<T> = false;

// type Null = null;

export type IsNull<T> = false;

// type Number = number;

export type IsNumber<T> = false;

// type String = string;

export type IsString<T> = false;

// type Symbol = symbol;

export type IsSymbol<T> = false;

// type Undefined = undefined;

export type IsUndefined<T> = false;

//

// https://github.com/sindresorhus/type-fest/blob/main/source/numeric.d.ts
export type Numeric = bigint | number;

// https://github.com/sindresorhus/type-fest/blob/main/source/primitive.d.ts
export type Primitive = bigint | boolean | null | number | string | symbol | undefined;

// https://github.com/sindresorhus/type-fest/blob/main/source/is-literal.d.ts
// https://stackoverflow.com/a/52806744/10292952

// prettier-ignore
type IsLiteralOf<T, Type extends Primitive> =
	IsNever<T> extends false
		? [T] extends [Type]
			? [Type] extends [T]
				? false
				: true
			: false
		: false;

export type IsBigIntLiteral<T> = IsLiteralOf<T, bigint>;

export type IsBooleanLiteral<T> = IsLiteralOf<T, boolean>;

export type IsNumberLiteral<T> = IsLiteralOf<T, number>;

export type IsStringLiteral<T> = IsLiteralOf<T, string>;

export type IsSymbolLiteral<T> = IsLiteralOf<T, symbol>;

export type IsLiteral<T extends Primitive> =
	| IsBigIntLiteral<T>
	| IsBooleanLiteral<T>
	| IsNumberLiteral<T>
	| IsStringLiteral<T>
	| IsSymbolLiteral<T>;
