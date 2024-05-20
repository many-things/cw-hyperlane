import { wrapMany } from '.';

export const HookQueryMsgs = ({
  quote_dispatch,
}: {
  quote_dispatch: { metadata: string; message: string }[];
}) =>
  wrapMany(
    [
      ...quote_dispatch.map((v) => ({
        quote_dispatch: {
          metadata: v.metadata,
          message: v.message,
        },
      })),
      {
        mailbox: {},
      },
    ],
    'hook',
  );
