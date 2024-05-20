import { QUERIES as HplHookMerkleQuery } from './hpl_hook_merkle';
import { QUERIES as HplIgpQuery } from './hpl_igp';
import { QUERIES as HplIgpOracleQuery } from './hpl_igp_oracle';
import { QUERIES as HplIsmMultisigQuery } from './hpl_ism_multisig';
import { QUERIES as HplMailboxQuery } from './hpl_mailbox';
import { QUERIES as HplValidatorAnnounceQuery } from './hpl_va';
import { QUERIES as HplWrapNativeQuery } from './hpl_warp_native';

export function toNamedQueries(contract: string, queries: { query: object }[]) {
  return { contract, queries: queries.map((v) => v.query) };
}

export function wrap(query: object, name: string) {
  return { query: { [name]: query } };
}

export function wrapMany(queries: object[], name: string) {
  return queries.map((q) => wrap(q, name));
}

export const QUERIES = [
  HplHookMerkleQuery,
  HplIgpQuery,
  HplIgpOracleQuery,
  HplIsmMultisigQuery,
  HplMailboxQuery,
  HplValidatorAnnounceQuery,
  HplWrapNativeQuery,
];
