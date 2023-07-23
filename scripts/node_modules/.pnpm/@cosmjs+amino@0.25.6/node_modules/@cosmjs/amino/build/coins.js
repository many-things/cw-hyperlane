"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.parseCoins = exports.coins = exports.coin = void 0;
const math_1 = require("@cosmjs/math");
/**
 * Creates a coin.
 */
function coin(amount, denom) {
    return { amount: new math_1.Uint53(amount).toString(), denom: denom };
}
exports.coin = coin;
/**
 * Creates a list of coins with one element.
 */
function coins(amount, denom) {
    return [coin(amount, denom)];
}
exports.coins = coins;
/**
 * Takes a coins list like "819966000ucosm,700000000ustake" and parses it.
 */
function parseCoins(input) {
    return input
        .replace(/\s/g, "")
        .split(",")
        .filter(Boolean)
        .map((part) => {
        const match = part.match(/^([0-9]+)([a-zA-Z]+)/);
        if (!match)
            throw new Error("Got an invalid coin string");
        return {
            amount: math_1.Uint64.fromString(match[1]).toString(),
            denom: match[2],
        };
    });
}
exports.parseCoins = parseCoins;
//# sourceMappingURL=coins.js.map