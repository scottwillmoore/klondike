import { expectTypeOf } from "vitest";

import { Rank } from "./Rank";

// @ts-expect-error
expectTypeOf<null>().toMatchTypeOf<Rank>();

// @ts-expect-error
expectTypeOf<undefined>().toMatchTypeOf<Rank>();

expectTypeOf<0>().toMatchTypeOf<Rank>();
expectTypeOf<12>().toMatchTypeOf<Rank>();

// @ts-expect-error
expectTypeOf<13>().toMatchTypeOf<Rank>();

// TODO: @ts-expect-error
expectTypeOf<number>().toMatchTypeOf<Rank>();

// @ts-expect-error
expectTypeOf<"">().toMatchTypeOf<Rank>();

// @ts-expect-error
expectTypeOf<string>().toMatchTypeOf<Rank>();

expectTypeOf<Rank>().toMatchTypeOf<Rank>();
