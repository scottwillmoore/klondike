import { expectTypeOf } from "vitest";

import { suits, Suit } from "./Suit_";

// @ts-expect-error
expectTypeOf<undefined>().toMatchTypeOf<Suit>();

// @ts-expect-error
expectTypeOf<null>().toMatchTypeOf<Suit>();

expectTypeOf<0>().toMatchTypeOf<Suit>();
expectTypeOf<1>().toMatchTypeOf<Suit>();
expectTypeOf<2>().toMatchTypeOf<Suit>();
expectTypeOf<3>().toMatchTypeOf<Suit>();

// @ts-expect-error
expectTypeOf<4>().toMatchTypeOf<Suit>();

// @ts-expect-error
expectTypeOf<number>().toMatchTypeOf<Suit>();

// @ts-expect-error
expectTypeOf<"">().toMatchTypeOf<Suit>();

// @ts-expect-error
expectTypeOf<string>().toMatchTypeOf<Suit>();

expectTypeOf<Suit>().toMatchTypeOf<Suit>();

expectTypeOf(suits.club).toMatchTypeOf<Suit>();
expectTypeOf(suits.diamond).toMatchTypeOf<Suit>();
expectTypeOf(suits.heart).toMatchTypeOf<Suit>();
expectTypeOf(suits.spade).toMatchTypeOf<Suit>();
