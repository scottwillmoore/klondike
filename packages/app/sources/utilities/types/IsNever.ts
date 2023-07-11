// https://github.com/microsoft/TypeScript/issues/31751#issuecomment-498526919
// https://github.com/sindresorhus/type-fest/blob/main/source/is-never.d.ts
// https://stackoverflow.com/a/53984913/7349225
// https://www.zhenghao.io/posts/ts-never#how-to-check-for-never
export type IsNever<T> = [T] extends [never] ? true : false;
