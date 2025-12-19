import { defineConfig } from "vitest/config";
import { fileURLToPath } from "url";
import { dirname, resolve } from "path";
import react from "@vitejs/plugin-react";
import { readFileSync } from "fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const packageJson = JSON.parse(
  readFileSync(resolve(__dirname, "../..", "package.json"), "utf-8"),
);

export default defineConfig({
  define: {
    __VERSION__: JSON.stringify(packageJson.version),
  },
  plugins: [
    react({
      jsxImportSource: "@emotion/react",
      babel: {
        plugins: ["@emotion/babel-plugin"],
      },
    }),
  ],
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: [
      resolve(__dirname, "../..", "vitest.setup.ts"),
      resolve(__dirname, "src/test-utils/setup.ts"),
    ],
    include: ["src/**/*.{test,spec}.{ts,tsx}"],
  },
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
    },
  },
});
