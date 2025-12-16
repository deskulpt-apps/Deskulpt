import {
  defineCollections,
  defineConfig,
  defineDocs,
  frontmatterSchema,
} from "fumadocs-mdx/config";
import lastModifiedPlugin from "fumadocs-mdx/plugins/last-modified";
import { z } from "zod";
import path from "node:path";

export const docs = defineDocs({
  dir: "content/docs",
  docs: {
    schema: frontmatterSchema.extend({
      description: z.string(),
    }),
    postprocess: {
      includeProcessedMarkdown: true,
    },
  },
});

export const releases = defineCollections({
  type: "doc",
  dir: "content/releases",
  schema: frontmatterSchema.extend({
    version: z.string(),
    date: z.date(),
    latest: z.boolean().default(false),
  }),
  postprocess: {
    includeProcessedMarkdown: true,
  },
});

export default defineConfig({
  plugins: [lastModifiedPlugin()],
  mdxOptions: {
    remarkImageOptions: {
      publicDir: path.resolve(__dirname, "../public"),
    },
  },
});
