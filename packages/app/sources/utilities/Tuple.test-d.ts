import { expectTypeOf } from "vitest";

import { Tuple } from "./Tuple";

type T = unknown;

expectTypeOf<Tuple<T, number>>().toEqualTypeOf<never>();

expectTypeOf<Tuple<T, 0>>().toEqualTypeOf<[]>();
expectTypeOf<Tuple<T, 1>>().toEqualTypeOf<[T]>();
expectTypeOf<Tuple<T, 2>>().toEqualTypeOf<[T, T]>();
expectTypeOf<Tuple<T, 4>>().toEqualTypeOf<[T, T, T, T]>();

expectTypeOf<Tuple<T, 0 | 1>>().toEqualTypeOf<[] | [T]>();
expectTypeOf<Tuple<T, 0 | 1 | 2>>().toEqualTypeOf<[] | [T] | [T, T]>();
expectTypeOf<Tuple<T, 0 | 1 | 2 | 4>>().toEqualTypeOf<[] | [T] | [T, T] | [T, T, T, T]>();
