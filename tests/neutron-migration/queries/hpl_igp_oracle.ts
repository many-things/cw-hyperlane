import { toNamedQueries, wrapMany } from '.';
import { OwnableQueryMsgs } from './ownable';

export const QUERIES = toNamedQueries('hpl_igp_oracle', [
  ...OwnableQueryMsgs,
  ...wrapMany(
    [169, 42161].map((domain) => ({
      get_exchange_rate_and_gas_price: {
        dest_domain: domain,
      },
    })),
    'oracle',
  ),
]);
