import { Color } from ".";

export const club = 0;

export type Club = typeof club;

export const diamond = 1;

export type Diamond = typeof diamond;

export const heart = 2;

export type Heart = typeof heart;

export const spade = 3;

export type Spade = typeof spade;

export type Suit = Club | Diamond | Heart | Spade;

export function toASCIICharacter(suit: Suit): string {
	switch (suit) {
		case club:
			return "C";
		case diamond:
			return "D";
		case heart:
			return "H";
		case spade:
			return "S";
	}
}

export function toCharacter(suit: Suit): string {
	switch (suit) {
		case club:
			return "♣";
		case diamond:
			return "♦";
		case heart:
			return "♥";
		case spade:
			return "♠";
	}
}

export function toColor(suit: Suit): Color {
	switch (suit) {
		case club:
			return Color.Black;
		case diamond:
			return Color.Red;
		case heart:
			return Color.Red;
		case spade:
			return Color.Black;
	}
}

// rome-ignore lint/suspicious/noShadowRestrictedNames:
export function toString(suit: Suit): string {
	switch (suit) {
		case club:
			return "Club";
		case diamond:
			return "Diamond";
		case heart:
			return "Heart";
		case spade:
			return "Spade";
	}
}
