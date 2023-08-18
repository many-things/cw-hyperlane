import { readdirSync } from "fs";
import { CONTAINER } from "../ioc";
import { Context, Migration } from "../types";
import 'reflect-metadata'
import { saveContext } from "../load_context";


const MIGRATIONS: {[key: string]: any } = readdirSync(__dirname)
  .filter((f) => f !== "index.ts")
  .map((f) => f.replace(".ts", ""))
  .reduce((acc, cur) =>{
    acc[cur] = require(`./${cur}`).default
    return acc;
  }, {} as {[key: string]: any});

export async function runMigrations(network: string, dryRun: boolean) {
  const migraiotnMap: { [key: string]: any } = {};

  // migration maps
  Object.keys(MIGRATIONS).forEach((key) => {
    const cls = MIGRATIONS[key];
    CONTAINER.bind<typeof cls>(cls).toSelf();

    // save
    const obj = CONTAINER.get<typeof cls>(cls);
    migraiotnMap[obj.name] = obj;
  });


  // find initials
  const availableInitials = Object.keys(migraiotnMap).map((key) => migraiotnMap[key]).filter((obj) => obj.after === "");
  if(availableInitials.length != 1) {
    throw new Error("There must be one initial migration");
  }

  let current = availableInitials[0];
  let migrationOrder = [current];

  while(true) {
    let next: any | undefined = undefined;

    Object.keys(migraiotnMap).forEach((key) => {
      const obj = migraiotnMap[key];

      if(obj.after === current.name) {
        (next === undefined) ? (next = obj) : (() => { throw new Error("There must be one next migration") })();
      }
    });

    if(next === undefined) break;

    migrationOrder.push(next);
    current = next;
  }

  // get contexts and check after;
  let ctx = CONTAINER.get<Context>(Context);
  const migStartAt = migrationOrder.findIndex((obj) => obj.name === ctx.latestMigration) + 1;

  // migrate
  if (migrationOrder.slice(migStartAt).length === 0) {
    console.log("[INFO]".gray, "No migrations to run.");
    return;
  }

  console.log("\nrun migrations...\n")

  for(let obj of migrationOrder.slice(migStartAt)) {
    process.stdout.write("[MIGRATION]".gray);
    process.stdout.write(` ${obj.name} ... `);

    if (dryRun) {
      console.log("PASS".yellow);
      continue;
    }

    try {
      obj.setContext(ctx);
      ctx = await obj.run();

      ctx.latestMigration = obj.name;
      saveContext(network, ctx);

      console.log("OK".green);
    } catch(err) {
      console.log("FAIL".red);
      throw err;
    }
  }

  console.log("\n[INFO]".gray, "All migrations are done.");
  console.log("\n============= Migration Result =============\n")

  Object.keys(ctx.contracts).forEach((key) => {
    const contract = ctx.contracts[key];
    console.log(`${key}`.padEnd(30), '=>', `${contract.address}`);
  });
}

export async function runContract(network: string) {

}
