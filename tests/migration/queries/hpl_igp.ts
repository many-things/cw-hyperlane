import { toNamedQueries, wrapMany } from '.';
import { HookQueryMsgs } from './hook';
import { OwnableQueryMsgs } from './ownable';
import { RouterQueryMsgs } from './router';

export const QUERIES = toNamedQueries('hpl_igp', [
  ...OwnableQueryMsgs,
  ...HookQueryMsgs({ quote_dispatch: [] }), // TODO
  ...RouterQueryMsgs({ get_route: { domains: [] } }),
  ...wrapMany(
    [169, 42161].map((domain) => ({
      get_exchange_rate_and_gas_price: {
        dest_domain: domain,
      },
    })),
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
      ...[169, 42161].map((domain) => ({
        quote_gas_payment: {
          dest_domain: domain,
          gas_amount: `${300_000}`,
        },
      })),
    ],
    'igp',
  ),
]);
