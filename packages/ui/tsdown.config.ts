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
    fixedExtension: false,
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
    fixedExtension: false,
    failOnWarn: true,
    minify: true,
  },
]);
