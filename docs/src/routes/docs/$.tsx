import { createFileRoute, notFound } from "@tanstack/react-router";
import { DocsLayout } from "fumadocs-ui/layouts/notebook";
import {
  DocsBody,
  DocsDescription,
  DocsPage,
  DocsTitle,
  PageLastUpdate,
} from "fumadocs-ui/layouts/notebook/page";
import { createServerFn } from "@tanstack/react-start";
import { docsSource } from "@/lib/source";
import browserCollections from "fumadocs-mdx:collections/browser";
import defaultMdxComponents from "fumadocs-ui/mdx";
import { baseOptions } from "@/lib/layout.shared";
import { staticFunctionMiddleware } from "@tanstack/start-static-server-functions";
import { useFumadocsLoader } from "fumadocs-core/source/client";
import { PageActions } from "@/components/page-actions";

export const Route = createFileRoute("/docs/$")({
  component: Page,
  loader: async ({ params }) => {
    const slugs = params._splat?.split("/") ?? [];
    const data = await loader({ data: slugs });
    await clientLoader.preload(data.path);
    return data;
  },
});

const loader = createServerFn({
  method: "GET",
})
  .inputValidator((slugs: string[]) => slugs)
  .middleware([staticFunctionMiddleware])
  .handler(async ({ data: slugs }) => {
    const page = docsSource.getPage(slugs);
    if (!page) throw notFound();

    return {
      path: page.path,
      url: page.url,
      lastModified: page.data.lastModified,
      pageTree: await docsSource.serializePageTree(docsSource.pageTree),
    };
  });

const clientLoader = browserCollections.docs.createClientLoader<{
  path: string;
  url: string;
  lastModified?: Date;
}>({
  component({ toc, frontmatter, default: MDX }, { path, url, lastModified }) {
    return (
      <DocsPage toc={toc} tableOfContent={{ style: "clerk" }}>
        <DocsTitle>{frontmatter.title}</DocsTitle>
        <DocsDescription className="mb-2">
          {frontmatter.description}
        </DocsDescription>

        <PageActions
          markdownUrl={`${url}.md`}
          githubUrl={`https://github.com/deskulpt-apps/Deskulpt/blob/main/docs/content/docs/${path}`}
          className="mb-4 border-b pb-4"
        />

        <DocsBody>
          <MDX components={{ ...defaultMdxComponents }} />
        </DocsBody>

        <div className="mt-8">
          {lastModified && <PageLastUpdate date={lastModified} />}
        </div>
      </DocsPage>
    );
  },
});

function Page() {
  const data = Route.useLoaderData();
  const Content = clientLoader.getComponent(data.path);
  const { pageTree } = useFumadocsLoader(data);

  return (
    <DocsLayout {...baseOptions()} tree={pageTree}>
      <Content path={data.path} url={data.url} />
    </DocsLayout>
  );
}
