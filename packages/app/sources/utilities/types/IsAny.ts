// https://github.com/sindresorhus/type-fest/blob/main/source/is-any.d.ts
// https://stackoverflow.com/a/49928360/7349225
export type IsAny<T> = 0 extends 1 & T ? true : false;
