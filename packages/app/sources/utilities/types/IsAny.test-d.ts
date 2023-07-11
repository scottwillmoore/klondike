import { assertType } from "vitest";

import { IsAny } from "./IsAny";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsAny<any>>(true);
assertType<IsAny<never>>(false);
assertType<IsAny<unknown>>(false);

assertType<IsAny<bigint>>(false);
assertType<IsAny<boolean>>(false);
assertType<IsAny<null>>(false);
assertType<IsAny<number>>(false);
assertType<IsAny<string>>(false);
assertType<IsAny<symbol>>(false);
assertType<IsAny<undefined>>(false);
