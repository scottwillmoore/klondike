const card_wasm = require("./package/card_wasm");

const color = card_wasm.Color.Black;
console.log(color);
console.log(card_wasm.color_to_char(color));

const rank = card_wasm.Rank.Ace;
console.log(rank);
console.log(card_wasm.rank_to_char(rank));

const suit = card_wasm.Suit.Club;
console.log(suit);
console.log(card_wasm.suit_to_ascii_char(suit));
console.log(card_wasm.suit_to_char(suit));
console.log(card_wasm.suit_color(suit));

const card = new card_wasm.Card(rank, suit);
console.log(card);
console.log(card.color());
// console.log(card.rank());
// console.log(card.suit());
