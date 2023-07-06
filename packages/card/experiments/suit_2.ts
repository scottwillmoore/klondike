declare const type: unique symbol;
type NewType<T, Type> = T & { [type]: Type };

declare const suitType: unique symbol;
type SuitType = typeof suitType;

export type Club = NewType<0, SuitType>;
export type Diamond = NewType<1, SuitType>;
export type Heart = NewType<2, SuitType>;
export type Spade = NewType<3, SuitType>;

export type Suit = Club | Diamond | Heart | Spade;

export const Club = 0 as Club;
export const Diamond = 1 as Club;
export const Heart = 2 as Club;
export const Spade = 3 as Club;

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
		// Error: Bad!
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

declare const rankType: unique symbol;
type RankType = typeof rankType;

export type Ace = NewType<0, RankType>;
// ...

export type Rank = Ace;

export const Ace = 0 as Ace;
// ...

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
