import { defineConfig } from "tsdown";

export default defineConfig([
  {
    name: "internal",
    entry: {
      ui: "src/index.ts",
      "jsx-runtime": "src/jsx-runtime.ts",
    },
    outDir: "../../gen",
    format: "esm",
    platform: "browser",
    banner: "/*! Auto-generated from packages/ui. DO NOT EDIT! */",
    minify: true,
    clean: false,
    dts: false,
    failOnWarn: true,
  },
  {
    name: "package",
    entry: "src/index.ts",
    format: "esm",
    platform: "browser",
    failOnWarn: true,
    minify: true,
  },
]);
