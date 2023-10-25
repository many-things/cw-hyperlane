const codegen = require("@cosmwasm/ts-codegen").default;
const path = require("path");
const fs = require("fs");

const pkgRoot = path.join(__dirname, "..");

const schemaDir = path.join(pkgRoot, "..", "..", "schema");

const contracts = fs
  .readdirSync(schemaDir, { withFileTypes: true })
  .filter((c) => c.isDirectory())
  .map((c) => ({
    name: c.name,
    dir: path.join(schemaDir, c.name),
  }));

const outPath = path.join(pkgRoot, "src", "contracts");

fs.rmSync(outPath, { recursive: true, force: true });

codegen({
  contracts,
  outPath,
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
  console.log("✨ Typescript code is generated successfully!");
});
