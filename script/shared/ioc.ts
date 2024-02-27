import { Container } from "inversify";

import { Context } from "./context";
import { Client, Config } from "./config";

export const CONTAINER = new Container({
  autoBindInjectable: true,
  defaultScope: "Singleton",
});

// referenced by tsoa
export const iocContainer = CONTAINER;

export class Dependencies {
  ctx: Context;
  client: Client;
  network: Config["networks"][number];
}
