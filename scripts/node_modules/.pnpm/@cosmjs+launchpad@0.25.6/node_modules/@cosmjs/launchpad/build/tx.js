"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.isWrappedStdTx = exports.makeStdTx = exports.isStdTx = void 0;
function isStdTx(txValue) {
    const { memo, msg, fee, signatures } = txValue;
    return (typeof memo === "string" && Array.isArray(msg) && typeof fee === "object" && Array.isArray(signatures));
}
exports.isStdTx = isStdTx;
function makeStdTx(content, signatures) {
    return {
        msg: content.msgs,
        fee: content.fee,
        memo: content.memo,
        signatures: Array.isArray(signatures) ? signatures : [signatures],
    };
}
exports.makeStdTx = makeStdTx;
function isWrappedStdTx(wrapped) {
    return wrapped.type === "cosmos-sdk/StdTx" && isStdTx(wrapped.value);
}
exports.isWrappedStdTx = isWrappedStdTx;
//# sourceMappingURL=tx.js.map