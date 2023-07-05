declare const brand: unique symbol;
type Brand<T, U> = T & { [brand]?: U };

type Range<
	End extends number,
	Array extends unknown[] = []
> = End extends Array["length"]
	? Array[number]
	: Range<End, [...Array, Array["length"]]>;

type Rank2 = Brand<Range<12>, "Rank">;

const r2: Rank2 = 10;

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
}

export enum Suit {
	Club = 0,
	Diamond = 1,
	Heart = 2,
	Spade = 3,
}

export namespace Suit {
	export function toCharacter(suit: Suit): string {
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
}

export interface Card {
	readonly rank: Rank;
	readonly suit: Suit;
}

export namespace Card {
	export function toIdentifier(card: Card): string {
		return `${Rank.toCharacter(card.rank)}${Suit.toCharacter(card.suit)}`;
	}
}

const card: Card = {
	rank: Rank.Ace,
	suit: Suit.Spade,
};

console.log(card);
console.log(Card.toIdentifier(card));
