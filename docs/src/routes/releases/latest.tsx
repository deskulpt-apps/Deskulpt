import { releasesSource } from "@/lib/source";
import { createFileRoute, notFound, redirect } from "@tanstack/react-router";
import { createServerFn } from "@tanstack/react-start";
import { staticFunctionMiddleware } from "@tanstack/start-static-server-functions";

export const Route = createFileRoute("/releases/latest")({
  loader: async () => {
    const data = await loader();
    throw redirect({
      to: "/releases/$version",
      params: { version: data.version },
      replace: true,
    });
  },
});

const loader = createServerFn({
  method: "GET",
})
  .middleware([staticFunctionMiddleware])
  .handler(() => {
    const pages = releasesSource.getPages();
    const latestRelease = pages.find((page) => page.data.latest);
    if (!latestRelease) throw notFound();
    return { version: latestRelease.data.version };
  });
