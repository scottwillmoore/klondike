export type SuitDiscriminant = 0 | 1 | 2 | 3;

export class Suit {
	public static readonly Club = new Suit(0);
	public static readonly Diamond = new Suit(1);
	public static readonly Heart = new Suit(2);
	public static readonly Spade = new Suit(3);

	private readonly discriminant: SuitDiscriminant;

	private constructor(discriminant: SuitDiscriminant) {
		this.discriminant = discriminant;
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
			// Error: Bad!
			// default:
			// 	throw new Error();
		}
	}

	public toCharacter2(): string {
		switch (this.discriminant) {
			case 0:
				return "C";
			case 1:
				return "D";
			case 2:
				return "H";
			case 3:
				return "S";
			// No error: Good!
		}
	}
}

const a = Suit.Club;
const b = Suit.Diamond;

let c: number;
c = 0;
c = 4;
c = a; // Error: Good!
c = b; // Error: Good!

let d: Suit;
d = 0; // Error: Good!
d = 4; // Error: Good!
d = a;
d = b;

const y = a.toCharacter();
const z = b.toCharacter();
