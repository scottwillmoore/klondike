import { expectTypeOf } from "vitest";

import { None, Option, Some } from ".";

expectTypeOf<Option<null>>().toEqualTypeOf<null>();
expectTypeOf<Option<null>>().toEqualTypeOf<None>();

expectTypeOf<Option<0>>().toEqualTypeOf<0 | null>();
expectTypeOf<Option<0>>().toEqualTypeOf<Some<0> | None>();

expectTypeOf<Option<{}>>().toEqualTypeOf<{} | null>();
expectTypeOf<Option<{}>>().toEqualTypeOf<Some<{}> | None>();
