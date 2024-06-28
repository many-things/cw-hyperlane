import { toNamedQueries, wrapMany } from '.';
import { ConnectionQueryMsgs } from './connection';
import { OwnableQueryMsgs } from './ownable';
import { RouterQueryMsgs } from './router';

export const QUERIES = toNamedQueries('hpl_warp_native', [
  ...OwnableQueryMsgs,
  ...RouterQueryMsgs({ get_route: { domains: [] } }),
  ...ConnectionQueryMsgs,
  ...wrapMany(
    [
      {
        token_type: {},
      },
      {
        token_mode: {},
      },
    ],
    'token_default',
  ),
  ...wrapMany(
    [
      {
        interchain_security_module: [],
      },
    ],
    'ism_specifier',
  ),
]);
