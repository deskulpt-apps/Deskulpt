import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Deskulpt",
  description: "A cross-platform desktop customization tool.",
  base: "/",
  srcDir: "src",
  lastUpdated: true,
  head: [
    [
      "link",
      {
        rel: "icon",
        type: "image/svg+xml",
        href: "/deskulpt.svg",
        media: "(prefers-color-scheme: light)",
      },
    ],
    [
      "link",
      {
        rel: "icon",
        type: "image/svg+xml",
        href: "/deskulpt-dark.svg",
        media: "(prefers-color-scheme: dark)",
      },
    ],
    ["link", { rel: "icon", type: "image/png", href: "/deskulpt.png" }],
    ["meta", { property: "og:type", content: "website" }],
    ["meta", { property: "og:site_name", content: "Deskulpt" }],
    [
      "meta",
      {
        property: "og:title",
        content: "Deskulpt | Desktop Customization Tool",
      },
    ],
    [
      "meta",
      {
        property: "og:url",
        content: "https://deskulpt-apps.github.io/",
      },
    ],
  ],
  themeConfig: {
    logo: {
      light: "/deskulpt.svg",
      dark: "/deskulpt-dark.svg",
    },
    nav: [
      {
        text: "User Guide",
        link: "/guide/introduction",
        activeMatch: "/guide/",
      },
      {
        text: "Contribute",
        link: "/contribute/overview",
        activeMatch: "/contribute/",
      },
    ],
    sidebar: {
      "/guide/": [
        {
          text: "Getting Started",
          items: [
            { text: "Introduction", link: "/guide/introduction" },
            { text: "Quick Start", link: "/guide/quick-start" },
          ],
        },
      ],
      "/contribute/": [
        {
          text: "Contribution Guide",
          items: [
            { text: "Overview", link: "/contribute/overview" },
            { text: "Build and Run", link: "/contribute/build-and-run" },
            { text: "Testing", link: "/contribute/testing" },
            {
              text: "Quality Assurance",
              link: "/contribute/quality-assurance",
            },
            { text: "Documentation", link: "/contribute/documentation" },
          ],
        },
        {
          text: "Developers' Reference",
          items: [
            {
              text: "Backend Rustdoc",
              link: "https://deskulpt-apps.github.io/rustdoc/deskulpt/",
            },
          ],
        },
      ],
    },
    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/deskulpt-apps/Deskulpt",
      },
    ],
    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2024-2025 The Deskulpt Development Team",
    },
    search: {
      provider: "local",
    },
  },
});
