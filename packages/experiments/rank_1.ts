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

export class Suit {
	private static nextDiscriminant = 0;

	public static readonly Club = new Suit(Suit.nextDiscriminant++);
	public static readonly Diamond = new Suit(Suit.nextDiscriminant++);
	public static readonly Heart = new Suit(Suit.nextDiscriminant++);
	public static readonly Spade = new Suit(Suit.nextDiscriminant++);

	private readonly discriminant: number;

	private constructor(discriminant: number) {
		if (discriminant < Suit.nextDiscriminant) {
			this.discriminant = discriminant;
		} else {
			throw new Error();
		}
	}

	public toCharacter(): string {
		switch (this.discriminant) {
			case Suit.Club.discriminant:
				return "C";
			case Suit.Diamond.discriminant:
				return "D";
			case Suit.Heart.discriminant:
				return "H";
			case Suit.Spade.discriminant:
				return "S";
			default:
				throw new Error();
		}
	}

	public toString(): string {
		switch (this.discriminant) {
			case Suit.Club.discriminant:
				return "Club";
			case Suit.Diamond.discriminant:
				return "Diamond";
			case Suit.Heart.discriminant:
				return "Heart";
			case Suit.Spade.discriminant:
				return "Spade";
			default:
				throw new Error();
		}
	}
}

const suit = Suit.Club;
console.log(suit);
console.log(suit.toCharacter());
console.log(suit.toString());

export class Card {
	rank: Rank;
	suit: Suit;

	constructor(rank: Rank, suit: Suit) {
		this.rank = rank;
		this.suit = suit;
	}
}
