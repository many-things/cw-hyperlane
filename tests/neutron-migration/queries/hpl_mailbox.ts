import { toNamedQueries, wrapMany } from '.';
import { OwnableQueryMsgs } from './ownable';

export const QUERIES = toNamedQueries('hpl_mailbox', [
  ...OwnableQueryMsgs,
  // ...wrapMany(
  //   [
  //     {
  //       quote_dispatch: {
  //         dest_domain: 169,
  //         recipient_addr:
  //           '0000000000000000000000005060eCD5dFAD300A90592C04e504600A7cdcF70b',
  //         msg_body: Buffer.from('hello').toString('hex'),
  //       },
  //     },
  //   ],
  //   'hook',
  // ),
  ...wrapMany(
    [
      {
        hrp: {},
      },
      {
        local_domain: {},
      },
      {
        message_delivered: {
          id: '',
        },
      },
      {
        default_ism: {},
      },
      {
        default_hook: {},
      },
      {
        required_hook: {},
      },
      {
        nonce: {},
      },
      {
        recipient_ism: {
          recipient_addr:
            // test mock receiver
            'neutron1v0t8nztzhdsan2cv23x66xjrak9844zz9hq7gkz0w4j6xl4lmpzq89kt5g',
        },
      },
      {
        latest_dispatch_id: {},
      },
    ],
    'mailbox',
  ),
]);
