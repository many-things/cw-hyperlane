import { toNamedQueries, wrapMany } from '.';
import { HookQueryMsgs } from './hook';
import { OwnableQueryMsgs } from './ownable';

export const QUERIES = toNamedQueries('hpl_hook_merkle', [
  ...OwnableQueryMsgs,
  ...HookQueryMsgs({ quote_dispatch: [] }), // TODO
  ...wrapMany(
    [
      {
        count: {},
      },
      {
        root: {},
      },
      {
        branch: {},
      },
      {
        tree: {},
      },
      {
        check_point: {},
      },
    ],
    'merkle_hook',
  ),
]);
