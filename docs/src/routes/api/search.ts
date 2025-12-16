import { createFileRoute } from "@tanstack/react-router";
import { docsSource, releasesSource } from "@/lib/source";
import { createSearchAPI } from "fumadocs-core/search/server";

const server = createSearchAPI("advanced", {
  // https://docs.orama.com/docs/orama-js/supported-languages
  language: "english",
  indexes: [...docsSource.getPages(), ...releasesSource.getPages()].map(
    (page) => ({
      title: page.data.title,
      description: page.data.description,
      url: page.url,
      id: page.url,
      structuredData: page.data.structuredData,
    }),
  ),
});

export const Route = createFileRoute("/api/search")({
  server: {
    handlers: {
      GET: () => server.staticGET(),
    },
  },
});
