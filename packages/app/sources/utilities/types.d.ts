// https://github.com/sindresorhus/type-fest

export type Primitive =
	| bigint
	| boolean
	| null
	| number
	| string
	| symbol
	| undefined;

// https://github.com/sindresorhus/type-fest/blob/main/source/is-any.d.ts
// https://stackoverflow.com/a/49928360/7349225
export type IsAny<T> = 0 extends 1 & T ? true : false;

// https://github.com/microsoft/TypeScript/issues/31751#issuecomment-498526919
// https://github.com/sindresorhus/type-fest/blob/main/source/is-never.d.ts
// https://stackoverflow.com/a/53984913/7349225
// https://www.zhenghao.io/posts/ts-never#how-to-check-for-never
export type IsNever<T> = [T] extends [never] ? true : false;

// https://github.com/sindresorhus/type-fest/blob/main/source/is-unknown.d.ts
export type IsUnknown<T> = unknown extends T
	? IsAny<T> extends false
		? true
		: false
	: false;

export type IsStringLiteral<T> = false;
export type IsNumberLiteral<T> = false;
export type IsBigIntLiteral<T> = false;
export type IsBooleanLiteral<T> = false;

export type IsBigInt = 0;
