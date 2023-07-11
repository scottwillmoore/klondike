import { assertType } from "vitest";

import { IsNever } from "./IsNever";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsNever<any>>(false);
assertType<IsNever<never>>(true);
assertType<IsNever<unknown>>(false);

assertType<IsNever<bigint>>(false);
assertType<IsNever<boolean>>(false);
assertType<IsNever<null>>(false);
assertType<IsNever<number>>(false);
assertType<IsNever<string>>(false);
assertType<IsNever<symbol>>(false);
assertType<IsNever<undefined>>(false);
