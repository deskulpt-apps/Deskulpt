import { RollupLog, defineConfig } from "rollup";
import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";
import { fileURLToPath } from "url";
import { join } from "path";
import { Dir, opendirSync, rmSync } from "fs";

// Cleanup `dist/` directory
const __dirname = fileURLToPath(new URL(".", import.meta.url));
cleanDir(join(__dirname, "./dist"));

export default defineConfig([
  // ESM build of the emotion JSX runtime to be used internally
  {
    input: "src/jsx-runtime.ts",
    output: {
      format: "esm",
      file: "../deskulpt/gen/jsx-runtime.js",
      banner: "/*! Auto-generated from packages/ui. DO NOT EDIT! */",
    },
    external: ["@emotion/react/jsx-runtime"],
    plugins: [typescript(), terser()],
    onwarn,
  },
  // ESM build to be used internally
  {
    input: "src/index.ts",
    output: {
      format: "esm",
      file: "../deskulpt/gen/ui.js",
      banner: "/*! Auto-generated from packages/ui. DO NOT EDIT! */",
    },
    external: ["@emotion/react", "@radix-ui/themes"],
    plugins: [typescript(), terser()],
    onwarn,
  },
  // ESM build for publishing
  {
    input: "src/index.ts",
    output: {
      format: "esm",
      file: "./dist/index.js",
    },
    external: ["@emotion/react", "@radix-ui/themes"],
    plugins: [
      typescript({
        declaration: true,
        declarationDir: "./dist",
        rootDir: "./src",
      }),
      terser(),
    ],
    onwarn,
  },
]);

function onwarn(warning: RollupLog) {
  throw Object.assign(new Error(), warning);
}

function cleanDir(path: string) {
  let dir: Dir;
  try {
    dir = opendirSync(path);
  } catch (err) {
    if (err instanceof Error && "code" in err) {
      switch (err.code) {
        case "ENOENT":
          return;
        case "ENOTDIR":
          throw new Error(`'${path}' is not a directory`, { cause: err });
        default:
          throw err;
      }
    } else {
      throw err;
    }
  }

  let file = dir.readSync();
  while (file) {
    const filePath = join(path, file.name);
    rmSync(filePath, { recursive: true });
    file = dir.readSync();
  }
  dir.closeSync();
}
