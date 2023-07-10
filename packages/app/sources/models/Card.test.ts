import { expect, test } from "vitest";

import { Card, Color, Rank, Suit } from ".";

const card = Card.of(Rank.Ace, Suit.Club);

test("toColor", () => {
	expect(Card.toColor(card)).toBe(Color.Black);
});

test("toCharacter", () => {
	expect(Card.toIdentifier(card)).toBe("A♣");
});

test("toString", () => {
	expect(Card.toString(card)).toBe("Ace of Clubs");
});
