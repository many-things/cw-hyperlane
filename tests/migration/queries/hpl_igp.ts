import { toNamedQueries, wrapMany } from '.';
import { HookQueryMsgs } from './hook';
import { OwnableQueryMsgs } from './ownable';
import { RouterQueryMsgs } from './router';

export const QUERIES = toNamedQueries('hpl_igp', [
  ...OwnableQueryMsgs,
  ...HookQueryMsgs({ quote_dispatch: [] }), // TODO
  ...RouterQueryMsgs({ get_route: { domains: [] } }),
  ...wrapMany(
    [
      {
        get_exchange_rate_and_gas_price: { dest_domain: 1 },
      },
    ],
    'oracle',
  ),
  ...wrapMany(
    [
      {
        default_gas: {},
      },
      {
        gas_for_domain: {
          domains: [],
        },
      },
      {
        list_gas_for_domains: {},
      },
      { beneficiary: {} },
      {
        quote_gas_payment: {
          dest_domain: 0,
          gas_amount: '0',
        },
      },
    ],
    'igp',
  ),
]);
