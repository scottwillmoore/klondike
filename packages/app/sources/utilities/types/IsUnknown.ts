import { IsAny } from ".";

// https://github.com/sindresorhus/type-fest/blob/main/source/is-unknown.d.ts
export type IsUnknown<T> = unknown extends T ? (IsAny<T> extends false ? true : false) : false;
