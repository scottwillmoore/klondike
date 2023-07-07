import { Color, Rank, Suit } from ".";

export interface Card {
	rank: Rank;
	suit: Suit;
}

export namespace Card {
	export function of(rank: Rank, suit: Suit): Card {
		return { rank, suit };
	}

	export function toColor(card: Card): Color {
		return Suit.toColor(card.suit);
	}

	export function toIdentifier(card: Card): string {
		return `${Rank.toCharacter(card.rank)}${Suit.toCharacter(card.suit)}`;
	}

	// rome-ignore lint/suspicious/noShadowRestrictedNames: Not a problem in a namespace.
	export function toString(card: Card): string {
		return `${Rank.toString(card.rank)} of ${Suit.toString(card.suit)}s`;
	}
}
