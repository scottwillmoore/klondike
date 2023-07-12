import { assertType } from "vitest";

import { greet } from "./greet";

assertType<(name: string) => string>(greet);
