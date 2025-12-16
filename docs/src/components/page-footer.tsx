"use client";

import { ComponentProps, useMemo } from "react";
import { ChevronLeftIcon, ChevronRightIcon } from "lucide-react";
import Link from "fumadocs-core/link";
import { cn } from "fumadocs-ui/utils/cn";
import type * as PageTree from "fumadocs-core/page-tree";
import { usePathname } from "fumadocs-core/framework";
import { isActive } from "fumadocs-ui/utils/is-active";
import { useFooterItems } from "fumadocs-ui/utils/use-footer-items";

export function PageFooter({ className, ...props }: ComponentProps<"div">) {
  const footerList = useFooterItems();
  const pathname = usePathname();

  const { previous, next } = useMemo(() => {
    const idx = footerList.findIndex((item) =>
      isActive(item.url, pathname, false),
    );
    if (idx === -1) return {};
    return { previous: footerList[idx - 1], next: footerList[idx + 1] };
  }, [footerList, pathname]);

  return (
    <div
      className={cn(
        "@container grid gap-4",
        previous && next ? "grid-cols-2" : "grid-cols-1",
        className,
      )}
      {...props}
    >
      {previous ? <FooterItem item={previous} index={-1} /> : null}
      {next ? <FooterItem item={next} index={1} /> : null}
    </div>
  );
}

function FooterItem({
  item,
  index,
}: {
  item: Pick<PageTree.Item, "name" | "description" | "url">;
  index: -1 | 1;
}) {
  const Icon = index === -1 ? ChevronLeftIcon : ChevronRightIcon;

  return (
    <Link
      href={item.url}
      className={cn(
        "flex flex-col gap-2 rounded-lg border p-4 text-sm transition-colors hover:bg-fd-accent/80 hover:text-fd-accent-foreground @max-lg:col-span-full",
        index === 1 && "text-end",
      )}
    >
      <div
        className={cn(
          "inline-flex items-center gap-1.5 font-medium",
          index === 1 && "flex-row-reverse",
        )}
      >
        <Icon className="-mx-1 size-4 shrink-0 rtl:rotate-180" />
        <p>{item.name}</p>
      </div>
      <p className="truncate text-fd-muted-foreground">
        {item.description ?? (index === -1 ? "Previous Page" : "Next Page")}
      </p>
    </Link>
  );
}
