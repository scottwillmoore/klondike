export var Rank;
(function (Rank) {
    Rank[Rank["Ace"] = 0] = "Ace";
    Rank[Rank["Two"] = 1] = "Two";
    Rank[Rank["Three"] = 2] = "Three";
    Rank[Rank["Four"] = 3] = "Four";
    Rank[Rank["Five"] = 4] = "Five";
    Rank[Rank["Six"] = 5] = "Six";
    Rank[Rank["Seven"] = 6] = "Seven";
    Rank[Rank["Eight"] = 7] = "Eight";
    Rank[Rank["Nine"] = 8] = "Nine";
    Rank[Rank["Ten"] = 9] = "Ten";
    Rank[Rank["Jack"] = 10] = "Jack";
    Rank[Rank["Queen"] = 11] = "Queen";
    Rank[Rank["King"] = 12] = "King";
})(Rank || (Rank = {}));
(function (Rank) {
    function toCharacter(rank) {
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
    Rank.toCharacter = toCharacter;
})(Rank || (Rank = {}));
export var Suit;
(function (Suit) {
    Suit[Suit["Club"] = 0] = "Club";
    Suit[Suit["Diamond"] = 1] = "Diamond";
    Suit[Suit["Heart"] = 2] = "Heart";
    Suit[Suit["Spade"] = 3] = "Spade";
})(Suit || (Suit = {}));
(function (Suit) {
    function toCharacter(suit) {
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
    Suit.toCharacter = toCharacter;
})(Suit || (Suit = {}));
export var Card;
(function (Card) {
    function toIdentifier(card) {
        return `${Rank.toCharacter(card.rank)}${Suit.toCharacter(card.suit)}`;
    }
    Card.toIdentifier = toIdentifier;
})(Card || (Card = {}));
const card = {
    rank: Rank.Ace,
    suit: Suit.Spade,
};
console.log(card);
console.log(Card.toIdentifier(card));
