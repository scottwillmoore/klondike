export enum Color {
	Black = 0,
	Red = 1,
}

export namespace Color {
	export function toCharacter(color: Color): string {
		switch (color) {
			case Color.Black:
				return "B";
			case Color.Red:
				return "R";
		}
	}

	// rome-ignore lint/suspicious/noShadowRestrictedNames: Not a problem in a namespace.
	export function toString(color: Color): string {
		switch (color) {
			case Color.Black:
				return "Black";
			case Color.Red:
				return "Red";
		}
	}
}

export enum Rank {
	Ace = 0,
	Two = 1,
	Three = 2,
	Four = 3,
	Five = 4,
	Six = 5,
	Seven = 6,
	Eight = 7,
	Nine = 8,
	Ten = 9,
	Jack = 10,
	Queen = 11,
	King = 12,
}

export namespace Rank {
	export function toCharacter(rank: Rank): string {
		switch (rank) {
			case Rank.Ace:
				return "A";
			case Rank.Two:
				return "2";
			case Rank.Three:
				return "3";
			case Rank.Four:
				return "4";
			case Rank.Five:
				return "5";
			case Rank.Six:
				return "6";
			case Rank.Seven:
				return "7";
			case Rank.Eight:
				return "8";
			case Rank.Nine:
				return "9";
			case Rank.Ten:
				return "T";
			case Rank.Jack:
				return "J";
			case Rank.Queen:
				return "Q";
			case Rank.King:
				return "K";
		}
	}

	// rome-ignore lint/suspicious/noShadowRestrictedNames: Not a problem in a namespace.
	export function toString(rank: Rank): string {
		switch (rank) {
			case Rank.Ace:
				return "Ace";
			case Rank.Two:
				return "Two";
			case Rank.Three:
				return "Three";
			case Rank.Four:
				return "Four";
			case Rank.Five:
				return "Five";
			case Rank.Six:
				return "Six";
			case Rank.Seven:
				return "Seven";
			case Rank.Eight:
				return "Eight";
			case Rank.Nine:
				return "Nine";
			case Rank.Ten:
				return "Ten";
			case Rank.Jack:
				return "Jack";
			case Rank.Queen:
				return "Queen";
			case Rank.King:
				return "King";
		}
	}
}

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
