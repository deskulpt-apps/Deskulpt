import { defineConfig } from "tsdown";
import { replacePlugin } from "rolldown/plugins";

export default defineConfig([
  {
    name: "internal",
    entry: {
      "raw-apis": "src/raw.ts",
    },
    outDir: "../../gen",
    format: "esm",
    fixedExtension: false,
    banner: "/*! Auto-generated from packages/apis. DO NOT EDIT! */",
    minify: true,
    clean: false,
    dts: false,
    failOnWarn: true,
  },
  {
    name: "wrapper",
    entry: {
      "apis.wrapper": "src/index.ts",
    },
    outDir: "../../crates/tauri-plugin-deskulpt-core/gen",
    format: "esm",
    fixedExtension: false,
    banner: "/*! Auto-generated from packages/apis. DO NOT EDIT! */",
    minify: true,
    clean: false,
    dts: false,
    failOnWarn: true,
    deps: {
      neverBundle: ["__RAW_APIS_URL__"],
    },
    plugins: [
      replacePlugin(
        { "./raw": "__RAW_APIS_URL__" },
        { delimiters: ["", ""], preventAssignment: true },
      ),
    ],
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
