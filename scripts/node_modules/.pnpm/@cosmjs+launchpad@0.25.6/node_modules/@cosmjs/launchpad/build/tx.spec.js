"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
/* eslint-disable @typescript-eslint/naming-convention */
const amino_1 = require("@cosmjs/amino");
const signdoc_1 = require("@cosmjs/amino/build/signdoc");
const tx_1 = require("./tx");
describe("tx", () => {
    describe("makeStdTx", () => {
        it("can make an StdTx from a SignDoc and one signature", () => {
            const fee = { amount: amino_1.coins(123, "ucosm"), gas: "22" };
            const signDoc = signdoc_1.makeSignDoc([], fee, "chain-xy", "hello", 3, 4);
            const signature = {
                pub_key: {
                    type: "tendermint/PubKeySecp256k1",
                    value: "AtQaCqFnshaZQp6rIkvAPyzThvCvXSDO+9AzbxVErqJP",
                },
                signature: "1nUcIH0CLT0/nQ0mBTDrT6kMG20NY/PsH7P2gc4bpYNGLEYjBmdWevXUJouSE/9A/60QG9cYeqyTe5kFDeIPxQ==",
            };
            const signedTx = tx_1.makeStdTx(signDoc, signature);
            expect(signedTx).toEqual({
                msg: [],
                memo: "hello",
                fee: fee,
                signatures: [signature],
            });
        });
        it("can make an StdTx from a SignDoc and multiple signatures", () => {
            const fee = { amount: amino_1.coins(123, "ucosm"), gas: "22" };
            const signDoc = signdoc_1.makeSignDoc([], fee, "chain-xy", "hello", 3, 4);
            const signature1 = {
                pub_key: {
                    type: "tendermint/PubKeySecp256k1",
                    value: "AtQaCqFnshaZQp6rIkvAPyzThvCvXSDO+9AzbxVErqJP",
                },
                signature: "1nUcIH0CLT0/nQ0mBTDrT6kMG20NY/PsH7P2gc4bpYNGLEYjBmdWevXUJouSE/9A/60QG9cYeqyTe5kFDeIPxQ==",
            };
            const signature2 = {
                pub_key: {
                    type: "tendermint/PubKeySecp256k1",
                    value: "A5qFcJBJvEK/fOmEAY0DHNWwSRZ9TEfNZyH8VoVvDtAq",
                },
                signature: "NK1Oy4EUGAsoC03c1wi9GG03JC/39LEdautC5Jk643oIbEPqeXHMwaqbdvO/Jws0X/NAXaN8SAy2KNY5Qml+5Q==",
            };
            const signedTx = tx_1.makeStdTx(signDoc, [signature1, signature2]);
            expect(signedTx).toEqual({
                msg: [],
                memo: "hello",
                fee: fee,
                signatures: [signature1, signature2],
            });
        });
    });
});
//# sourceMappingURL=tx.spec.js.map