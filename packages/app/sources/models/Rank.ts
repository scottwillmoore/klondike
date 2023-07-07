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
