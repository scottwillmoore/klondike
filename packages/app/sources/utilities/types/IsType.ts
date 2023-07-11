import { IsNever } from ".";

type IsType<T, Type> = IsNever<T> extends false ? (T extends Type ? true : false) : false;

export type IsBigInt<T> = IsType<T, bigint>;

export type IsBoolean<T> = IsType<T, boolean>;

export type IsNull<T> = IsType<T, null>;

export type IsNumber<T> = IsType<T, number>;

export type IsString<T> = IsType<T, string>;

export type IsSymbol<T> = IsType<T, symbol>;

export type IsUndefined<T> = IsType<T, undefined>;
