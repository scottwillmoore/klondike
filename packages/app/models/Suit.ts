import { Color } from "./";

export enum Suit {
	Club = 0,
	Diamond = 1,
	Heart = 2,
	Spade = 3,
}

export namespace Suit {
	export function toASCIICharacter(suit: Suit): string {
		switch (suit) {
			case Suit.Club:
				return "C";
			case Suit.Diamond:
				return "D";
			case Suit.Heart:
				return "H";
			case Suit.Spade:
				return "S";
		}
	}

	export function toCharacter(suit: Suit): string {
		switch (suit) {
			case Suit.Club:
				return "♣";
			case Suit.Diamond:
				return "♦";
			case Suit.Heart:
				return "♥";
			case Suit.Spade:
				return "♠";
		}
	}

	export function toColor(suit: Suit): Color {
		switch (suit) {
			case Suit.Club:
				return Color.Black;
			case Suit.Diamond:
				return Color.Red;
			case Suit.Heart:
				return Color.Red;
			case Suit.Spade:
				return Color.Black;
		}
	}

	// rome-ignore lint/suspicious/noShadowRestrictedNames: Not a problem in a namespace.
	export function toString(suit: Suit): string {
		switch (suit) {
			case Suit.Club:
				return "Club";
			case Suit.Diamond:
				return "Diamond";
			case Suit.Heart:
				return "Heart";
			case Suit.Spade:
				return "Spade";
		}
	}
}
