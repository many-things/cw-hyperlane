import * as fs from "fs";
import codegen from "@cosmwasm/ts-codegen";
import path from "path";

const SCHEMA_DIR = process.env.SCHEMA_DIR || path.join(process.cwd(), "schema");

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const capitalize = (str: string): string =>
  str.charAt(0).toUpperCase() + str.slice(1);

const contracts = fs
  .readdirSync(SCHEMA_DIR, { withFileTypes: true })
  .filter((c) => !c.isDirectory())
  .map((c) => ({
    name: c.name.replace(".json", ""),
    dir: SCHEMA_DIR,
  }));

codegen({
  contracts,
  outPath: "./dist/",

  // options are completely optional ;)
  options: {
    bundle: {
      bundleFile: "index.ts",
      scope: "contracts",
    },
    client: {
      enabled: true,
    },
    messageComposer: {
      enabled: true,
    },
  },
}).then(() => {
  console.log("✨ all done!");
});
