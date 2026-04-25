import { defineConfig } from "tsdown";

export default defineConfig([
  {
    name: "internal",
    entry: {
      react: "src/index.ts",
    },
    outDir: "../../gen",
    format: "esm",
    fixedExtension: false,
    banner: "/*! Auto-generated from packages/react. DO NOT EDIT! */",
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
