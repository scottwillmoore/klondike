import { expectTypeOf } from "vitest";

import { None, Null, Some } from ".";

expectTypeOf<Null<null>>().toEqualTypeOf<null>();
expectTypeOf<Null<null>>().toEqualTypeOf<None>();

expectTypeOf<Null<0>>().toEqualTypeOf<0 | null>();
expectTypeOf<Null<0>>().toEqualTypeOf<Some<0> | None>();

expectTypeOf<Null<{}>>().toEqualTypeOf<{} | null>();
expectTypeOf<Null<{}>>().toEqualTypeOf<Some<{}> | None>();
