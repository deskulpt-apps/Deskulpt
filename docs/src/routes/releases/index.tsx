import { Link, createFileRoute } from "@tanstack/react-router";
import { HomeLayout } from "fumadocs-ui/layouts/home";
import { baseOptions } from "@/lib/layout.shared";
import AnimatedGridPattern from "@/components/ui/animated-grid-pattern";
import { ArrowUpRightIcon, TagIcon } from "lucide-react";
import { createServerFn } from "@tanstack/react-start";
import { staticFunctionMiddleware } from "@tanstack/start-static-server-functions";
import { releasesSource } from "@/lib/source";
import semver from "semver";
import { GitHubLogo } from "@/components/logos";

export const Route = createFileRoute("/releases/")({
  component: Releases,
  loader: async () => {
    return await loader();
  },
});

const loader = createServerFn({
  method: "GET",
})
  .middleware([staticFunctionMiddleware])
  .handler(() => {
    const pages = releasesSource.getPages();
    return {
      releases: pages
        .map((page) => ({
          url: page.url,
          version: page.data.version,
          date: page.data.date,
          latest: page.data.latest,
        }))
        .toSorted((a, b) => semver.rcompare(a.version, b.version)),
    };
  });

function Releases() {
  const data = Route.useLoaderData();
  const latestRelease = data.releases.find((release) => release.latest);

  return (
    <HomeLayout {...baseOptions()}>
      <Hero latestVersion={latestRelease?.version} />
      <ReleasesGrid releases={data.releases} />
    </HomeLayout>
  );
}

function Hero({ latestVersion }: { latestVersion?: string }) {
  return (
    <div className="relative flex items-center justify-center px-6">
      <AnimatedGridPattern
        numSquares={15}
        maxOpacity={0.1}
        duration={2}
        className="inset-0 mask-[radial-gradient(50vw_circle_at_center,white,transparent)]"
      />
      <div className="relative z-10 max-w-lg text-center">
        <h1 className="mt-12 text-xl font-semibold tracking-tighter sm:text-2xl md:text-3xl lg:text-4xl">
          <span className="animate-shimmer bg-linear-to-r from-primary-shifted via-fd-primary to-primary-shifted bg-size-(--bg-size-shimmer) bg-clip-text text-transparent">
            Deskulpt
          </span>{" "}
          Releases
        </h1>

        <p className="mt-10 mb-6 text-fd-foreground/80">
          Keep up with the latest Deskulpt releases! Explore new features,
          improvements, and bug fixes, and experience the best of Deskulpt!
        </p>

        <Link
          to="/releases/latest"
          className="inline-flex translate-y-1/2 animate-shimmer items-center gap-x-2 rounded-full bg-linear-to-r from-primary-shifted via-fd-primary to-primary-shifted bg-size-(--bg-size-shimmer) px-4 py-2 font-medium text-fd-primary-foreground"
        >
          Latest Release: v{latestVersion}
          <ArrowUpRightIcon size={20} />
        </Link>
      </div>
    </div>
  );
}

function ReleasesGrid({
  releases,
}: {
  releases: {
    url: string;
    version: string;
    date: Date;
    latest: boolean;
  }[];
}) {
  return (
    <div className="mx-auto grid w-full max-w-5xl grid-cols-1 gap-4 px-6 py-18 sm:grid-cols-2 lg:grid-cols-3">
      {releases.map(({ url, version, date }) => (
        <div
          key={url}
          className="group relative flex flex-col justify-between overflow-hidden rounded-lg border bg-fd-accent/50 p-4 text-fd-secondary-foreground transition-colors duration-200 hover:border-fd-primary hover:bg-fd-accent"
        >
          <div className="flex items-start justify-between">
            <h3 className="inline-flex items-center gap-x-2 text-lg font-semibold">
              <TagIcon size={16} /> v{version}
            </h3>
            <a
              href={`https://github.com/deskulpt-apps/Deskulpt/releases/tag/v${version}`}
              target="_blank"
              rel="noreferrer noopener"
              className="relative z-10 text-fd-muted-foreground transition-colors hover:text-fd-accent-foreground"
            >
              <GitHubLogo className="size-5" />
            </a>
          </div>

          <div className="mt-2 text-sm text-fd-muted-foreground">
            {date.toLocaleDateString(undefined, {
              year: "numeric",
              month: "2-digit",
              day: "2-digit",
            })}
          </div>

          <a href={url} className="absolute inset-0 z-0">
            <span className="sr-only">View release v{version}</span>
          </a>
        </div>
      ))}
    </div>
  );
}
