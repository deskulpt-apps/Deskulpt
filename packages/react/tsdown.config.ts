import { defineConfig } from "tsdown";

export default defineConfig([
  {
    name: "internal",
    entry: {
      react: "src/index.ts",
    },
    outDir: "../../gen",
    format: "esm",
    platform: "browser",
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
    platform: "browser",
    failOnWarn: true,
    minify: true,
  },
]);
