"use client";

import { ComponentProps, useMemo, useState } from "react";
import {
  CheckIcon,
  ChevronDownIcon,
  CopyIcon,
  ExternalLinkIcon,
} from "lucide-react";
import { useCopyButton } from "fumadocs-ui/utils/use-copy-button";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "fumadocs-ui/components/ui/popover";
import { buttonVariants } from "fumadocs-ui/components/ui/button";
import { ClaudeLogo, GitHubLogo, OpenAILogo } from "@/components/logos";
import { cn } from "fumadocs-ui/utils/cn";

const cache = new Map<string, string>();

export function PageActions({
  markdownUrl,
  githubUrl,
  className,
  ...props
}: {
  markdownUrl: string;
  githubUrl: string;
} & ComponentProps<"div">) {
  const [isLoading, setLoading] = useState(false);

  const [checked, onCopyClick] = useCopyButton(async () => {
    const cached = cache.get(markdownUrl);
    if (cached) return navigator.clipboard.writeText(cached);

    setLoading(true);
    try {
      await navigator.clipboard.write([
        new ClipboardItem({
          "text/plain": fetch(markdownUrl).then(async (res) => {
            const content = await res.text();
            cache.set(markdownUrl, content);
            return content;
          }),
        }),
      ]);
    } finally {
      setLoading(false);
    }
  });

  const items = useMemo(() => {
    const fullMarkdownUrl =
      typeof window === "undefined"
        ? "loading"
        : new URL(markdownUrl, window.location.origin);
    const q = `Read ${fullMarkdownUrl}, I want to ask questions about it.`;

    return [
      {
        title: "View on GitHub",
        href: githubUrl,
        icon: <GitHubLogo />,
      },
      {
        title: "Open in ChatGPT",
        href: `https://chatgpt.com/?${new URLSearchParams({
          hints: "search",
          q,
        })}`,
        icon: <OpenAILogo />,
      },
      {
        title: "Open in Claude",
        href: `https://claude.ai/new?${new URLSearchParams({ q })}`,
        icon: <ClaudeLogo />,
      },
    ];
  }, [githubUrl, markdownUrl]);

  return (
    <Popover>
      <div className={cn("inline-flex", className)} {...props}>
        <button
          type="button"
          disabled={isLoading}
          onClick={onCopyClick}
          className={buttonVariants({
            color: "secondary",
            size: "sm",
            className:
              "cursor-pointer gap-1.5 rounded-l-lg rounded-r-none px-3",
          })}
        >
          {checked ? <CheckIcon size={14} /> : <CopyIcon size={14} />}
          Copy page
        </button>

        <PopoverTrigger asChild>
          <button
            type="button"
            className={buttonVariants({
              color: "secondary",
              size: "sm",
              className:
                "-ms-px cursor-pointer rounded-l-none rounded-r-lg px-2",
            })}
          >
            <ChevronDownIcon size={14} />
          </button>
        </PopoverTrigger>
      </div>

      <PopoverContent className="flex flex-col">
        {items.map((item) => (
          <a
            key={item.href}
            href={item.href}
            target="_blank"
            rel="noreferrer noopener"
            className="inline-flex items-center gap-2 rounded-lg p-2 text-sm hover:bg-fd-accent hover:text-fd-accent-foreground [&_svg]:size-4"
          >
            {item.icon}
            {item.title}
            <ExternalLinkIcon className="ms-auto size-3.5 text-fd-muted-foreground" />
          </a>
        ))}
      </PopoverContent>
    </Popover>
  );
}
