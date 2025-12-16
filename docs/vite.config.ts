import { defineConfig } from "vite";
import { tanstackStart } from "@tanstack/react-start/plugin/vite";
import react from "@vitejs/plugin-react";
import tsConfigPaths from "vite-tsconfig-paths";
import tailwindcss from "@tailwindcss/vite";
import fumadocsMdx from "fumadocs-mdx/vite";
import path from "node:path";
import llmDocs from "./vite.plugin-llm-docs";

export default defineConfig({
  publicDir: path.resolve(__dirname, "../public"),
  plugins: [
    fumadocsMdx(await import("./source.config")),
    tailwindcss(),
    tsConfigPaths({ projects: ["./tsconfig.json"] }),
    tanstackStart({
      router: {
        generatedRouteTree: "route-tree.gen.ts",
        quoteStyle: "double",
        semicolons: true,
      },
      prerender: {
        enabled: true,
        crawlLinks: true,
      },
      sitemap: {
        host: "https://deskulpt-apps.github.io",
      },
      pages: [{ path: "/api/search" }, { path: "/llms.txt" }],
    }),
    llmDocs(),
    react(),
  ],
});
