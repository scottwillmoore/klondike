export enum Suit {
	Club = 0,
	Diamond = 1,
	Heart = 2,
	Spade = 3,
}

export const toCharacter = (suit: Suit): string => {
	switch (suit) {
		case Suit.Club:
			return "C";
		case Suit.Diamond:
			return "D";
		case Suit.Heart:
			return "H";
		case Suit.Spade:
			return "S";
		// No error: Good!
		// default:
		// 	throw new Error();
	}
};

//

const a = Suit.Club;
const b = Suit.Diamond;

let c: number;
c = 0;
c = 4;
c = a; // No error: Okay...
c = b; // No error: Okay...

let d: Suit;
d = 0; // No error: Okay...
d = 4; // Error: Good!
d = a;
d = b;

export enum Rank {
	Ace = 0,
}

let e: Rank;
e = 0; // No error: Okay...
e = 1; // Error: Good!
e = a; // Error: Good!
e = b; // Error: Good!

const y = toCharacter(a);
const z = toCharacter(b);
