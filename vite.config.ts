import { defineConfig } from "vite";
import { resolve } from "path";
import react from "@vitejs/plugin-react";
import { version } from "./package.json";

export default defineConfig({
  define: {
    __VERSION__: JSON.stringify(version),
  },
  plugins: [
    react({
      jsxImportSource: "@emotion/react",
      babel: {
        plugins: ["@emotion/babel-plugin"],
      },
    }),
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  build: {
    rollupOptions: {
      input: {
        manager: resolve(__dirname, "packages/deskulpt-manager/index.html"),
        canvas: resolve(__dirname, "packages/deskulpt-canvas/index.html"),
        // Make the scripts entrypoints so that they are preserved even if not imported
        "gen/jsx-runtime": resolve(__dirname, "gen/jsx-runtime.js"),
        "gen/raw-apis": resolve(__dirname, "gen/raw-apis.js"),
        "gen/react": resolve(__dirname, "gen/react.js"),
        "gen/ui": resolve(__dirname, "gen/ui.js"),
      },
      output: {
        // Make sure scripts are at the root of the build output so that their
        // import paths are consistent with in the dev server
        entryFileNames: ({ name }) =>
          name.startsWith("gen/") ? "[name].js" : "assets/[name].js",
      },
      // Make sure exports of the scripts are preserved so that they can be imported
      // deterministically
      preserveEntrySignatures: "allow-extension",
    },
  },
});
