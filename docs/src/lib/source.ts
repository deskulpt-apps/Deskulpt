import { loader } from "fumadocs-core/source";
import { lucideIconsPlugin } from "fumadocs-core/source/lucide-icons";
import { toFumadocsSource } from "fumadocs-mdx/runtime/server";
import { docs, releases } from "fumadocs-mdx:collections/server";

export const docsSource = loader({
  source: docs.toFumadocsSource(),
  baseUrl: "/docs",
  plugins: [lucideIconsPlugin()],
});

export const releasesSource = loader({
  source: toFumadocsSource(releases, []),
  baseUrl: "/releases",
  plugins: [lucideIconsPlugin()],
});

export type GenericSourceType = typeof docsSource | typeof releasesSource;
