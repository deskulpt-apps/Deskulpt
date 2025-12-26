import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";
import {
  BookOpenIcon,
  DownloadIcon,
  EllipsisIcon,
  FileTextIcon,
  RocketIcon,
} from "lucide-react";

export function baseOptions(): BaseLayoutProps {
  return {
    githubUrl: "https://github.com/deskulpt-apps/Deskulpt",
    themeSwitch: {
      enabled: true,
      mode: "light-dark",
    },
    nav: {
      title: (
        <div className="flex items-center space-x-2">
          <img
            src="/deskulpt.png"
            className="w-7 dark:hue-rotate-180 dark:invert-90"
          />
          <span className="text-[15px] font-semibold">Deskulpt</span>
        </div>
      ),
    },
    links: [
      {
        type: "menu",
        icon: <FileTextIcon />,
        text: "Documentation",
        url: "/docs",
        active: "nested-url",
        items: [
          {
            icon: <BookOpenIcon />,
            text: "User Guide",
            url: "/docs/guide",
            active: "nested-url",
            description: "Learn how to use Deskulpt effectively.",
          },
        ],
      },
      {
        type: "menu",
        icon: <EllipsisIcon />,
        text: "Resources",
        active: "nested-url",
        items: [
          {
            icon: <RocketIcon />,
            text: "Releases",
            url: "/releases",
            active: "nested-url",
            description: "Download and explore the latest Deskulpt releases.",
          },
        ],
      },
      {
        type: "icon",
        icon: <DownloadIcon />,
        text: "Download",
        label: "Download Deskulpt",
        url: "/releases/latest",
        active: "nested-url",
        secondary: true,
      },
    ],
  };
}
