export class Suit {
	private static discriminant = 0;

	public static readonly Club = new Suit(this.discriminant++);
	public static readonly Diamond = new Suit(this.discriminant++);
	public static readonly Heart = new Suit(this.discriminant++);
	public static readonly Spade = new Suit(this.discriminant++);

	private readonly discriminant: number;

	private constructor(discriminant: number) {
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
