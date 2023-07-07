import { assertType } from "vitest";

import { IsAny } from "./types";

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

import { IsNever } from "./types";

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

import { IsUnknown } from "./types";

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
