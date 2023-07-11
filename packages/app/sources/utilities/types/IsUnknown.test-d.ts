import { assertType } from "vitest";

import { IsUnknown } from "./IsUnknown";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsUnknown<any>>(false);
assertType<IsUnknown<never>>(false);
assertType<IsUnknown<unknown>>(true);

assertType<IsUnknown<bigint>>(false);
assertType<IsUnknown<boolean>>(false);
assertType<IsUnknown<null>>(false);
assertType<IsUnknown<number>>(false);
assertType<IsUnknown<string>>(false);
assertType<IsUnknown<symbol>>(false);
assertType<IsUnknown<undefined>>(false);
