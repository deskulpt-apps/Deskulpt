import { Link, createFileRoute, notFound } from "@tanstack/react-router";
import { baseOptions } from "@/lib/layout.shared";
import AnimatedGridPattern from "@/components/ui/animated-grid-pattern";
import { buttonVariants } from "fumadocs-ui/components/ui/button";
import { cn } from "fumadocs-ui/utils/cn";
import {
  ArrowDownIcon,
  ArrowRightIcon,
  CalendarIcon,
  CheckIcon,
  ChevronDownIcon,
  CopyIcon,
  DownloadIcon,
  TextIcon,
} from "lucide-react";
import { AppleLogo, LinuxLogo, WindowsLogo } from "@/components/logos";
import * as Select from "@radix-ui/react-select";
import {
  ComponentProps,
  FC,
  PropsWithChildren,
  ReactNode,
  useEffect,
  useMemo,
  useState,
} from "react";
import { createServerFn } from "@tanstack/react-start";
import { staticFunctionMiddleware } from "@tanstack/start-static-server-functions";
import { releasesSource } from "@/lib/source";
import browserCollections from "fumadocs-mdx:collections/browser";
import { TreeContextProvider } from "fumadocs-ui/contexts/tree";
import {
  DocsBody,
  DocsPage,
  PageLastUpdate,
} from "fumadocs-ui/layouts/notebook/page";
import defaultMdxComponents from "fumadocs-ui/mdx";
import * as TabsComponents from "fumadocs-ui/components/tabs";
import { useFumadocsLoader } from "fumadocs-core/source/client";
import { HomeLayout } from "fumadocs-ui/layouts/home";
import {
  DownloadOption,
  getLinuxDownloadOptions,
  getMacOSDownloadOptions,
  getWindowsDownloadOptions,
} from "@/lib/download-options";
import { DocsTitle } from "fumadocs-ui/layouts/notebook/page";
import * as TOCClerk from "fumadocs-ui/components/toc/clerk";
import { TOCProvider, TOCScrollArea } from "fumadocs-ui/components/toc/index";
import { TableOfContents } from "fumadocs-core/toc";
import Bowser from "bowser";
import { PageActions } from "@/components/page-actions";
import { PageFooter } from "@/components/page-footer";

export const Route = createFileRoute("/releases/$version")({
  component: Page,
  loader: async ({ params }) => {
    const data = await loader({ data: params.version });
    await clientLoader.preload(data.path);
    return data;
  },
});

const loader = createServerFn({
  method: "GET",
})
  .inputValidator((version: string) => version)
  .middleware([staticFunctionMiddleware])
  .handler(async ({ data: version }) => {
    const page = releasesSource.getPage([version]);
    if (!page) throw notFound();

    return {
      path: page.path,
      url: page.url,
      pageTree: await releasesSource.serializePageTree(releasesSource.pageTree),
    };
  });

const clientLoader = browserCollections.releases.createClientLoader<{
  path: string;
  url: string;
}>({
  component({ toc, frontmatter, default: MDX }, { path, url }) {
    return (
      <div>
        <Download
          version={frontmatter.version}
          date={frontmatter.date}
          latest={frontmatter.latest}
        />

        <DocsPage footer={{ enabled: false }}>
          <div className="mx-auto w-full max-w-270! px-2 py-6">
            <ReleaseNotes
              toc={toc}
              path={path}
              url={url}
              title={frontmatter.title}
              version={frontmatter.version}
            >
              <MDX
                components={{ ...defaultMdxComponents, ...TabsComponents }}
              />
              <PageFooter className="not-prose mt-12" />
            </ReleaseNotes>
          </div>
        </DocsPage>
      </div>
    );
  },
});

function Page() {
  const data = Route.useLoaderData();
  const Content = clientLoader.getComponent(data.path);
  const { pageTree } = useFumadocsLoader(data);

  return (
    <HomeLayout {...baseOptions()}>
      <TreeContextProvider tree={pageTree}>
        <Content path={data.path} url={data.url} />
      </TreeContextProvider>
    </HomeLayout>
  );
}

function Download({
  version,
  date,
  latest,
}: {
  version: string;
  date: Date;
  latest: boolean;
}) {
  return (
    <div className="relative flex min-h-[calc(100dvh-3.5rem)] items-center justify-center px-6">
      <AnimatedGridPattern
        numSquares={30}
        maxOpacity={0.1}
        duration={2}
        className="inset-0 skew-y-8 mask-[radial-gradient(50vw_circle_at_center,white,transparent)]"
      />

      <div className="relative z-10 mx-auto w-full max-w-4xl xl:max-w-5xl">
        <div className="grid items-center gap-x-16 md:grid-cols-2">
          <div className="relative md:order-1">
            <ReleaseHero version={version} date={date} latest={latest} />
          </div>

          <div className="md:pl-8 lg:pl-12">
            <PlatformDownload
              version={version}
              platform="Windows"
              platformLogo={<WindowsLogo />}
              options={getWindowsDownloadOptions(version)}
              caption="Requires Windows 10 or later."
            />
            <PlatformDownload
              version={version}
              platform="macOS"
              platformLogo={<AppleLogo />}
              options={getMacOSDownloadOptions(version)}
              caption="Requires Catalina 10.15 or later."
            />
            <PlatformDownload
              version={version}
              platform="Linux"
              platformLogo={<LinuxLogo />}
              options={getLinuxDownloadOptions(version)}
              caption="Requires X11. Wayland is not supported (yet)."
            />
          </div>
        </div>
      </div>
    </div>
  );
}

function ReleaseHero({
  version,
  date,
  latest,
}: {
  version: string;
  date: Date;
  latest: boolean;
}) {
  return (
    <div className="relative flex flex-col items-center justify-center text-center">
      <div className="pointer-events-none absolute inset-0 -z-10 flex items-center justify-center">
        <div className="to-fd-primary-shifted/10 size-104 rounded-full bg-linear-to-tr from-fd-primary/35 blur-[120px]" />
        <div className="from-fd-primary-shifted/40 absolute size-72 animate-pulse rounded-full bg-linear-to-bl to-fd-primary/10 opacity-80 blur-[72px]" />
      </div>

      <img
        src="/deskulpt.svg"
        alt="Deskulpt"
        className="relative z-10 size-36 dark:hue-rotate-180 dark:invert-90"
      />

      <h1 className="mt-6 text-2xl font-semibold tracking-tighter sm:text-3xl md:text-4xl lg:text-5xl">
        Deskulpt{" "}
        <span className="animate-shimmer bg-linear-to-r from-primary-shifted via-fd-primary to-primary-shifted bg-size-(--bg-size-shimmer) bg-clip-text text-transparent">
          v{version}
        </span>
      </h1>

      <p className="mt-2 inline-flex items-center gap-x-2 text-fd-foreground/80">
        <CalendarIcon size={16} />
        {date.toLocaleDateString("en-US", {
          year: "numeric",
          month: "long",
          day: "numeric",
        })}
      </p>

      <ReleaseHeroQuickLinks version={version} latest={latest} />
    </div>
  );
}

function useCurrentPlatform() {
  const [platform, setPlatform] = useState<string | null>(null);

  useEffect(() => {
    const parser = Bowser.getParser(window.navigator.userAgent);
    setPlatform(parser.getOSName(true));
  }, []);

  return platform;
}

function PlatformDownload({
  version,
  platform,
  platformLogo,
  options,
  caption,
}: {
  version: string;
  platform: string;
  platformLogo: ReactNode;
  options: DownloadOption[];
  caption: string;
}) {
  const [option, setOption] = useState<DownloadOption>(options[0]);

  const currentPlatform = useCurrentPlatform();
  const isCurrentPlatform = currentPlatform === platform.toLowerCase();

  const handleValueChange = (value: string) => {
    const option = options.find((opt) => opt.value === value);
    if (option !== undefined) {
      setOption(option);
    }
  };

  return (
    <div className="mt-6 flex flex-col gap-y-2">
      <div className="flex items-center justify-between gap-x-4">
        <div className="flex items-center gap-2 [&>svg]:size-4">
          {platformLogo}
          <h2 className="font-medium">{platform}</h2>
        </div>

        <Select.Root value={option.value} onValueChange={handleValueChange}>
          <Select.Trigger className="flex cursor-pointer items-center gap-1 text-sm text-fd-primary outline-hidden">
            <Select.Value className="truncate" />
            <Select.Icon>
              <ChevronDownIcon size={16} />
            </Select.Icon>
          </Select.Trigger>
          <Select.Portal>
            <Select.Content
              position="popper"
              side="bottom"
              align="end"
              className="z-50 min-w-24 overflow-hidden rounded-lg bg-fd-popover shadow-lg"
              sideOffset={4}
            >
              <Select.Viewport className="p-1">
                {options.map((option) => (
                  <Select.Item
                    key={option.value}
                    value={option.value}
                    className="relative flex cursor-pointer items-center rounded-md px-2 py-1 text-sm outline-none select-none data-disabled:pointer-events-none data-disabled:opacity-50 data-highlighted:bg-fd-accent data-highlighted:text-fd-accent-foreground"
                  >
                    <Select.ItemText>{option.label}</Select.ItemText>
                  </Select.Item>
                ))}
              </Select.Viewport>
            </Select.Content>
          </Select.Portal>
        </Select.Root>
      </div>

      {option.download && (
        <a
          href={`https://github.com/deskulpt-apps/Deskulpt/releases/download/v${version}/${option.download}`}
          download
          className={cn(
            buttonVariants(),
            "flex w-full items-center gap-2 bg-fd-primary/10 hover:bg-fd-primary/20",
            isCurrentPlatform && "outline-2 outline-fd-primary",
          )}
        >
          <DownloadIcon size={16} /> Download for {platform}
        </a>
      )}

      {option.code && (
        <DownloadCodeBlock
          code={option.code}
          className={cn(isCurrentPlatform && "outline")}
        />
      )}

      <div className="text-sm">
        {isCurrentPlatform && (
          <div className="font-medium text-fd-primary">
            This is detected as your current platform.
          </div>
        )}
        <div className="text-fd-muted-foreground">{caption}</div>
      </div>
    </div>
  );
}

function DownloadCodeBlock({
  code,
  className,
  ...props
}: ComponentProps<"div"> & { code: string }) {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    try {
      navigator.clipboard.writeText(code);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (error) {
      console.error("Failed to copy code block:", error);
    }
  };

  return (
    <div
      className={cn(
        "flex h-9 items-center rounded-sm bg-fd-primary/10 text-sm",
        className,
      )}
      {...props}
    >
      <span className="mr-2 ml-3 text-fd-muted-foreground select-none">$</span>
      <span className="flex-1 overflow-x-auto font-mono whitespace-nowrap [scrollbar-width:none]">
        {code}
      </span>
      <button
        onClick={handleCopy}
        className="px-3 text-fd-muted-foreground hover:text-fd-accent-foreground"
        title="Copy to clipboard"
      >
        {copied ? <CheckIcon size={16} /> : <CopyIcon size={16} />}
      </button>
    </div>
  );
}

const RELEASE_NOTES_ANCHOR = "nav-release-notes";

function ReleaseHeroQuickLinks({
  version,
  latest,
}: {
  version: string;
  latest: boolean;
}) {
  return (
    <div className="my-8 flex flex-col text-fd-foreground/80 *:inline-flex *:items-center *:gap-3 [&_svg]:size-4.5 [&>a]:hover:text-fd-primary [&>a]:hover:underline [&>a]:hover:underline-offset-2">
      {latest ? (
        <span>
          <CheckIcon />
          <span>
            You are on the{" "}
            <span className="animate-shimmer bg-linear-to-r from-primary-shifted via-fd-primary to-primary-shifted bg-size-(--bg-size-shimmer) bg-clip-text font-medium text-transparent">
              latest stable release
            </span>
          </span>
        </span>
      ) : (
        <Link to="/releases/latest">
          <ArrowRightIcon /> Go to the latest stable release
        </Link>
      )}
      <Link to="/releases">
        <ArrowRightIcon /> Pick a different release
      </Link>
      <a
        href={`https://github.com/deskulpt-apps/Deskulpt/releases/tag/v${version}`}
        target="_blank"
        rel="noopener noreferrer"
      >
        <ArrowRightIcon /> View this release on GitHub
      </a>

      <a href={`#${RELEASE_NOTES_ANCHOR}`}>
        <ArrowDownIcon /> Scroll down for release notes
      </a>
    </div>
  );
}

function ReleaseNotes({
  toc,
  path,
  url,
  title,
  version,
  children,
}: PropsWithChildren<{
  toc: TableOfContents;
  path: string;
  url: string;
  title: string;
  version: string;
}>) {
  const refinedTOC: TableOfContents = useMemo(() => {
    return [
      {
        title: `Download Deskulpt v${version}`,
        depth: 1,
        url: "#",
      },
      ...toc,
    ];
  }, [version, toc]);

  return (
    <div className="lg:grid lg:grid-cols-[minmax(0,1fr)_260px] lg:gap-10 lg:pt-10">
      <div className="flex min-w-0 flex-col gap-4">
        <DocsTitle id={RELEASE_NOTES_ANCHOR}>{title}</DocsTitle>
        <PageActions
          markdownUrl={`${url}.md`}
          githubUrl={`https://github.com/deskulpt-apps/Deskulpt/blob/main/docs/content/releases/${path}`}
        />
        <ReleaseNotesTOC toc={refinedTOC} className="mt-4 lg:hidden" />
        <DocsBody className="mt-4">{children}</DocsBody>
      </div>

      <ReleaseNotesTOC
        toc={refinedTOC}
        className="hidden lg:block"
        containerClassName="sticky top-24 max-h-[calc(100dvh-6rem)]"
      />
    </div>
  );
}

function ReleaseNotesTOC({
  toc,
  containerClassName,
  ...props
}: ComponentProps<"aside"> & {
  toc: TableOfContents;
  containerClassName?: string;
}) {
  return (
    <aside {...props}>
      <div className={containerClassName}>
        <TOCProvider toc={toc}>
          <span className="inline-flex items-center gap-2 text-sm text-fd-muted-foreground">
            <TextIcon size={14} /> On this page
          </span>
          <TOCScrollArea>
            <TOCClerk.TOCItems />
          </TOCScrollArea>
        </TOCProvider>
      </div>
    </aside>
  );
}
