import { wrapMany } from '.';

export const IsmQueryMsgs = wrapMany(
  [
    {
      module_type: {},
    },

    // {
    //   verify: {
    //     metadata: '',
    //     message: '',
    //   },
    // },

    // {
    //   verify_info: {
    //     message: '',
    //   },
    // },
  ],
  'ism',
);
