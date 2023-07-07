import { expect, test } from "vitest";

import { Card, Rank, Suit } from ".";

test("Card.toString", () => {
	const card = Card.of(Rank.Ace, Suit.Club);
	expect(Card.toString(card)).toBe("Ace of Clubs");
});
