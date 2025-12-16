import { createFileRoute } from "@tanstack/react-router";
import { GenericSourceType, docsSource, releasesSource } from "@/lib/source";

function formatCollection(collection: GenericSourceType) {
  const ROOT_KEY = "__ROOT_KEY__";
  const scanned = [];
  const sectionMap = new Map<string, string[]>();

  for (const page of collection.getPages()) {
    const segments = page.path.split("/");
    const sectionKey = segments.length > 1 ? segments[0] : ROOT_KEY;
    const section = sectionMap.get(sectionKey) ?? [];
    let entry = `- [${page.data.title}](${page.url}.md)`;
    if (page.data.description) {
      entry += `: ${page.data.description}`;
    }
    section.push(entry);
    sectionMap.set(sectionKey, section);
  }

  for (const [sectionKey, section] of sectionMap.entries()) {
    if (sectionKey !== ROOT_KEY) {
      scanned.push(`### ${sectionKey}`);
    }
    scanned.push(section.join("\n"));
  }

  return scanned.join("\n\n");
}

export const Route = createFileRoute("/llms.txt")({
  server: {
    handlers: {
      GET: () => {
        const scanned = [];
        scanned.push("# Deskulpt");

        scanned.push("## Documentation");
        scanned.push(formatCollection(docsSource));

        scanned.push("## Releases");
        scanned.push(formatCollection(releasesSource));

        return new Response(scanned.join("\n\n"));
      },
    },
  },
});
