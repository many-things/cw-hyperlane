// @ts-check

import { terser } from "rollup-plugin-terser";
import typescript2 from "rollup-plugin-typescript2";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";

import pkg from "./package.json";

/**
 * Comment with library information to be appended in the generated bundles.
 */
const banner = `/*!
 * ${pkg.name} v${pkg.version}
 * (c) ${pkg.author.name}
 * Released under the ${pkg.license} License.
 */
`;

/**
 * Creates an output options object for Rollup.js.
 * @param {import('rollup').OutputOptions} options
 * @returns {import('rollup').OutputOptions}
 */
function createOutputOptions(options) {
  return {
    banner,
    name: "counter-sdk",
    exports: "named",
    sourcemap: true,
    ...options,
  };
}

/**
 * @type {import('rollup').RollupOptions}
 */
const options = {
  input: "./src/index.ts",
  output: [
    createOutputOptions({
      file: "./dist/index.js",
      format: "commonjs",
    }),
    createOutputOptions({
      file: "./dist/index.cjs",
      format: "commonjs",
    }),
    createOutputOptions({
      file: "./dist/index.mjs",
      format: "esm",
    }),
    createOutputOptions({
      file: "./dist/index.esm.js",
      format: "esm",
    }),
    createOutputOptions({
      file: "./dist/index.umd.js",
      format: "umd",
    }),
    createOutputOptions({
      file: "./dist/index.umd.min.js",
      format: "umd",
      plugins: [terser()],
    }),
  ],
  plugins: [
    resolve(),
    commonjs({
      dynamicRequireTargets: [
        // include using a glob pattern (either a string or an array of strings)
        "node_modules/cosmjs-types/*",
      ],
    }),
    typescript2({
      clean: true,
      useTsconfigDeclarationDir: true,
      tsconfig: "./tsconfig.bundle.json",
    }),
  ],
  external: ["cosmjs-types", "@cosmjs/encoding", "@cosmjs/cosmwasm-stargate"],
};

export default options;
