import { Plugin } from "vite";
import path from "node:path";
import fs, { glob } from "node:fs/promises";

const WEBSITE_URL = "https://deskulpt-apps.github.io";

async function getLLMDoc(file: string, url: string) {
  const content = await fs.readFile(file, "utf-8");
  return `${content.trim()}

---

> Source: ${WEBSITE_URL}${url.slice(0, -3)}
>
> To discover other pages of this site, fetch the LLM index: ${WEBSITE_URL}/llms.txt`;
}

export default function llmDocs() {
  const PLUGIN_NAME = "deskulpt:llm-docs";

  const OUTPUT_DIR = path.resolve(__dirname, "dist/client");
  const CONTENT_DIR = path.resolve(__dirname, "content");

  const devPlugin: Plugin = {
    name: `${PLUGIN_NAME}:dev`,
    apply: "serve",

    async configureServer(server) {
      server.middlewares.use(async (req, res, next) => {
        if (!req.url || req.method !== "GET") {
          return next();
        }

        const url = new URL(req.url, `http://${req.headers.host}`);
        if (!url.pathname.endsWith(".md")) {
          return next();
        }

        const relNoExt = url.pathname
          .slice(0, -3) // Remove .md
          .replace(/^\/+/, ""); // Avoid UNIX treating it as absolute
        const candidates = [
          path.join(CONTENT_DIR, relNoExt + ".mdx"),
          path.join(CONTENT_DIR, relNoExt, "index.mdx"),
        ];

        for (const candidate of candidates) {
          try {
            const content = await getLLMDoc(candidate, url.pathname);
            res.statusCode = 200;
            res.setHeader("Content-Type", "text/markdown; charset=utf-8");
            res.end(content);
            return;
          } catch {
            // Silently ignore
          }
        }

        return next();
      });
    },
  };

  const buildPlugin: Plugin = {
    name: `${PLUGIN_NAME}:build`,
    apply: "build",

    async writeBundle({ dir }) {
      if (dir === undefined || path.resolve(dir) !== OUTPUT_DIR) {
        return;
      }

      for await (const relPath of glob("**/*.mdx", { cwd: CONTENT_DIR })) {
        const relNoExt = relPath.slice(0, -4); // Remove .mdx
        const segments = relNoExt.split(path.sep);
        if (segments[segments.length - 1] === "index") {
          segments.pop();
        }

        const contentPath = path.join(CONTENT_DIR, relPath);
        const urlPath = "/" + segments.join("/") + ".md";
        const content = await getLLMDoc(contentPath, urlPath);

        const outputPath = path.join(OUTPUT_DIR, ...segments) + ".md";
        await fs.mkdir(path.dirname(outputPath), { recursive: true });
        await fs.writeFile(outputPath, content, "utf-8");
        this.info(`written: ${urlPath}`);
      }
    },
  };

  return [devPlugin, buildPlugin];
}
