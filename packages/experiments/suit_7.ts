export enum SuitType {
	Club = 0,
	Diamond = 1,
	Heart = 2,
	Spade = 3,
}

export class Suit {
	public static readonly Club = new Suit(0);
	public static readonly Diamond = new Suit(1);
	public static readonly Heart = new Suit(2);
	public static readonly Spade = new Suit(3);

	private readonly type: SuitType;

	private constructor(type: SuitType) {
		this.type = type;
	}

	public toCharacter(): string {
		switch (this.type) {
			case SuitType.Club:
				return "C";
			case SuitType.Diamond:
				return "D";
			case SuitType.Heart:
				return "H";
			case SuitType.Spade:
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
