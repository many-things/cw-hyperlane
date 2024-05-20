import { wrapMany } from '.';

export const ConnectionQueryMsgs = wrapMany(
  [{ get_mailbox: {} }, { get_hook: {} }, { get_ism: {} }],
  'connection',
);
