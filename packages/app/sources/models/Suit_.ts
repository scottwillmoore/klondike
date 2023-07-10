import { Color } from ".";

export const suits = {
	club: 0,
	diamond: 1,
	heart: 2,
	spade: 3,
} as const;

export type Suit = (typeof suits)[keyof typeof suits];

export function toASCIICharacter(suit: Suit): string {
	switch (suit) {
		case suits.club:
			return "C";
		case suits.diamond:
			return "D";
		case suits.heart:
			return "H";
		case suits.spade:
			return "S";
	}
}

export function toCharacter(suit: Suit): string {
	switch (suit) {
		case suits.club:
			return "♣";
		case suits.diamond:
			return "♦";
		case suits.heart:
			return "♥";
		case suits.spade:
			return "♠";
	}
}

export function toColor(suit: Suit): Color {
	switch (suit) {
		case suits.club:
			return Color.Black;
		case suits.diamond:
			return Color.Red;
		case suits.heart:
			return Color.Red;
		case suits.spade:
			return Color.Black;
	}
}

// rome-ignore lint/suspicious/noShadowRestrictedNames: Not a problem in a namespace.
export function toString(suit: Suit): string {
	switch (suit) {
		case suits.club:
			return "Club";
		case suits.diamond:
			return "Diamond";
		case suits.heart:
			return "Heart";
		case suits.spade:
			return "Spade";
	}
}
