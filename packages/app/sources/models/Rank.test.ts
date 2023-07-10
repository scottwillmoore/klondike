import { test, expect } from "vitest";

import { Rank } from "./Rank";

test("discriminant", () => {
	expect(Rank.Ace).toBe(0);
	expect(Rank.Two).toBe(1);
	// ...
});

test("toCharacter", () => {
	expect(Rank.toCharacter(Rank.Ace)).toBe("A");
	expect(Rank.toCharacter(Rank.Two)).toBe("2");
	// ...
});

test("toString", () => {
	expect(Rank.toString(Rank.Ace)).toBe("Ace");
	expect(Rank.toString(Rank.Two)).toBe("Two");
	// ...
});
