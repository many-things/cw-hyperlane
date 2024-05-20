import { toNamedQueries } from '.';

export const QUERIES = toNamedQueries('hpl_validator_announce', [
  {
    query: {
      get_announce_storage_locations: {
        validators: [
          '26fa235bc3f9e1b00ac3a0db4d84abb6f34244da',
          '42b6de2edbaa62c2ea2309ad85d20b3e37d38acf',
          '42fa752defe92459370a052b6387a87f7de9b80c',
          '47aa126e05933b95c5eb90b26e6b668d84f4b25a',
          '54b2cca5091b098a1a993dec03c4d1ee9af65999',
          '60e890b34cb44ce3fa52f38684f613f31b47a1a6',
          '7885fae56dbcf5176657f54adbbd881dc6714132',
          'a9b8c1f4998f781f958c63cfcd1708d02f004ff0',
          'b65438a014fb05fbadcfe35bc6e25d372b6ba460',
          'c79503a3e3011535a9c60f6d21f76f59823a38bd',
          'e000fa4e466831db288290dd97e66560fb3d7d28',
        ],
      },
    },
  },
  {
    query: {
      get_announced_validators: {},
    },
  },
]);
