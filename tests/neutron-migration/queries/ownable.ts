import { wrapMany } from '.';

export const OwnableQueryMsgs = wrapMany(
  [
    {
      get_owner: {},
    },

    {
      get_pending_owner: {},
    },
  ],
  'ownable',
);
