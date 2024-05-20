import { wrapMany } from '.';

export const RouterQueryMsgs = ({
  get_route,
}: {
  get_route: {
    domains: number[];
  };
}) =>
  wrapMany(
    [
      {
        domains: {},
      },
      ...get_route.domains.map((v) => ({ get_route: { domain: v } })),
      { list_routes: {} },
    ],
    'router',
  );
