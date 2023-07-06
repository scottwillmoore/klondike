declare const type: unique symbol;
type NewType<T, Type> = T & { [type]: Type };

type SuitDiscriminant = 0 | 1 | 2 | 3;

declare const suitType: unique symbol;
type SuitType = typeof suitType;

export type Suit = NewType<SuitDiscriminant, SuitType>;

export const Club = 0 as Suit;
export const Diamond = 1 as Suit;
export const Heart = 2 as Suit;
export const Spade = 3 as Suit;

export const Suit = {
	Club: Club,
	Diamond: Diamond,
	Heart: Heart,
	Spade: Spade,
};

export const toCharacter = (suit: Suit): string => {
	switch (suit) {
		case Club:
			return "C";
		case Diamond:
			return "D";
		case Heart:
			return "H";
		case Spade:
			return "S";
		// Error! No default case.
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
d = 0; // Error: Good!
d = 4; // Error: Good!
d = a;
d = b;

type RankIndex = 0;

declare const rankType: unique symbol;
type RankType = typeof rankType;

export type Rank = NewType<RankIndex, RankType>;

export const Ace = 0 as Rank;

export const Rank = {
	Ace: Ace,
	// ...
};

let e: Rank;
e = 0; // Error: Good!
e = 1; // Error: Good!
e = a; // Error: Good!
e = b; // Error: Good!

const y = toCharacter(a);
const z = toCharacter(b);
