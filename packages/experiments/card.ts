export enum Rank {
	Ace = 0,
	Two = 1,
	Three = 2,
	Four = 3,
	// ...
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
	rank: Rank;
	suit: Suit;
}

export namespace Card {
	export function create(rank: Rank, suit: Suit): Card {
		return { rank, suit };
	}

	export function toIdentifier(card: Card): string {
		return `${Rank.toCharacter(card.rank)}${Suit.toCharacter(card.suit)}`;
	}
}

const card = Card.create(Rank.Ace, Suit.Club);
const identifier = Card.toIdentifier(card);
