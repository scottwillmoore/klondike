// https://vitest.dev/guide/testing-types.html
// https://github.com/sindresorhus/type-fest/blob/main/test-d/is-any.ts
// https://github.com/SamVerschueren/tsd

// https://dev.to/ecyrbe/how-to-unit-test-your-typescript-utility-types-3cnm

// https://stackoverflow.com/questions/53807517/how-to-test-if-two-types-are-exactly-the-same
// https://github.com/aleclarson/spec.ts/blob/master/index.d.ts
// https://github.com/garronej/tsafe

export type Primitive =
	| bigint
	| boolean
	| null
	| number
	| string
	| symbol
	| undefined;

// https://stackoverflow.com/a/49928360/7349225
export type IsAny<T> = false;

// https://github.com/microsoft/TypeScript/issues/31751#issuecomment-498526919
// https://stackoverflow.com/a/53984913/7349225
// https://www.zhenghao.io/posts/ts-never#how-to-check-for-never
export type IsNever<T> = T extends never ? true : false;

type A = IsNever<0>; // =>

type B = IsNever<never>; // =>

export type IsUnknown<T> = false;

export type IsStringLiteral<T> = false;
export type IsNumberLiteral<T> = false;
export type IsBigIntLiteral<T> = false;
export type IsBooleanLiteral<T> = false;

export type IsBigInt = 0;
