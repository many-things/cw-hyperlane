import { toNamedQueries, wrapMany } from '.';
import { OwnableQueryMsgs } from './ownable';

export const QUERIES = toNamedQueries('hpl_igp_oracle', [
  ...OwnableQueryMsgs,
  ...wrapMany(
    [
      {
        get_exchange_rate_and_gas_price: {
          dest_domain: 1,
        },
      },
    ],
    'oracle',
  ),
]);
