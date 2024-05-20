import { toNamedQueries, wrapMany } from '.';
import { IsmQueryMsgs } from './ism';
import { OwnableQueryMsgs } from './ownable';

export const QUERIES = toNamedQueries('hpl_ism_multisig', [
  ...OwnableQueryMsgs,
  ...IsmQueryMsgs,
  ...wrapMany(
    [169, 42161].map((domain) => ({
      enrolled_validators: {
        domain,
      },
    })),
    'multisig_ism',
  ),
]);
