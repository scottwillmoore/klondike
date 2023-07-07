import { assertType } from "vitest";

import { IsAny } from "./IsType";

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

import { IsNever } from "./IsType";

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

import { IsUnknown } from "./IsType";

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

import { IsBigInt } from "./IsType";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsBigInt<any>>(false);
assertType<IsBigInt<never>>(false);
assertType<IsBigInt<unknown>>(false);

assertType<IsBigInt<bigint>>(true);
assertType<IsBigInt<boolean>>(false);
assertType<IsBigInt<null>>(false);
assertType<IsBigInt<number>>(false);
assertType<IsBigInt<string>>(false);
assertType<IsBigInt<symbol>>(false);
assertType<IsBigInt<undefined>>(false);

import { IsBoolean } from "./IsType";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsBoolean<any>>(false);
assertType<IsBoolean<never>>(false);
assertType<IsBoolean<unknown>>(false);

assertType<IsBoolean<bigint>>(false);
assertType<IsBoolean<boolean>>(true);
assertType<IsBoolean<null>>(false);
assertType<IsBoolean<number>>(false);
assertType<IsBoolean<string>>(false);
assertType<IsBoolean<symbol>>(false);
assertType<IsBoolean<undefined>>(false);

import { IsNull } from "./IsType";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsNull<any>>(false);
assertType<IsNull<never>>(false);
assertType<IsNull<unknown>>(false);

assertType<IsNull<bigint>>(false);
assertType<IsNull<boolean>>(true);
assertType<IsNull<null>>(false);
assertType<IsNull<number>>(false);
assertType<IsNull<string>>(false);
assertType<IsNull<symbol>>(false);
assertType<IsNull<undefined>>(false);

import { IsNumber } from "./IsType";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsNumber<any>>(false);
assertType<IsNumber<never>>(false);
assertType<IsNumber<unknown>>(false);

assertType<IsNumber<bigint>>(false);
assertType<IsNumber<boolean>>(true);
assertType<IsNumber<null>>(false);
assertType<IsNumber<number>>(false);
assertType<IsNumber<string>>(false);
assertType<IsNumber<symbol>>(false);
assertType<IsNumber<undefined>>(false);

import { IsString } from "./IsType";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsString<any>>(false);
assertType<IsString<never>>(false);
assertType<IsString<unknown>>(false);

assertType<IsString<bigint>>(false);
assertType<IsString<boolean>>(true);
assertType<IsString<null>>(false);
assertType<IsString<number>>(false);
assertType<IsString<string>>(false);
assertType<IsString<symbol>>(false);
assertType<IsString<undefined>>(false);

import { IsSymbol } from "./IsType";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsSymbol<any>>(false);
assertType<IsSymbol<never>>(false);
assertType<IsSymbol<unknown>>(false);

assertType<IsSymbol<bigint>>(false);
assertType<IsSymbol<boolean>>(true);
assertType<IsSymbol<null>>(false);
assertType<IsSymbol<number>>(false);
assertType<IsSymbol<string>>(false);
assertType<IsSymbol<symbol>>(false);
assertType<IsSymbol<undefined>>(false);

import { IsUndefined } from "./IsType";

// rome-ignore lint/suspicious/noExplicitAny:
assertType<IsUndefined<any>>(false);
assertType<IsUndefined<never>>(false);
assertType<IsUndefined<unknown>>(false);

assertType<IsUndefined<bigint>>(false);
assertType<IsUndefined<boolean>>(true);
assertType<IsUndefined<null>>(false);
assertType<IsUndefined<number>>(false);
assertType<IsUndefined<string>>(false);
assertType<IsUndefined<symbol>>(false);
assertType<IsUndefined<undefined>>(false);
