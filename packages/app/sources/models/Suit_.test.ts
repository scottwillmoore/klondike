import { test, expect } from "vitest";

import { suits } from "./Suit_";
import * as suits from "./Suit_";

test("discriminant", () => {
	expect(suits.club).toBe(0);
	expect(suits.diamond).toBe(1);
	// ...
});

test("toCharacter", () => {
	expect(suits.toCharacter(suits.club)).toBe("♣");
	expect(suits.toCharacter(suits.diamond)).toBe("♦");
	// ...
});

test("toString", () => {
	expect(suits.toString(suits.club)).toBe("Club");
	expect(suits.toString(suits.diamond)).toBe("Diamond");
	// ...
});
