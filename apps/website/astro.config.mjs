import { cpSync, existsSync, mkdirSync, rmSync } from "node:fs";
import { resolve } from "node:path";
import mdx from "@astrojs/mdx";
import react from "@astrojs/react";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";
import { GITHUB_URL } from "./src/lib/constants.ts";

/** Copies the pagefind index from `dist/` into `public/` so Vite serves it in dev mode. */
function devSearchSync() {
  return {
    name: "dev-search-sync",
    hooks: {
      "astro:server:start": async ({ logger }) => {
        const distIndex = resolve("./dist/pagefind");
        const publicIndex = resolve("./public/pagefind");

        if (!existsSync(distIndex)) {
          logger.warn(
            "[search] No index found — run `pnpm build:search` once to enable search in dev.",
          );
          return;
        }

        if (existsSync(publicIndex)) rmSync(publicIndex, { recursive: true });
        mkdirSync(publicIndex, { recursive: true });
        cpSync(distIndex, publicIndex, { recursive: true });
        logger.info("[search] Pagefind index synced — search is available.");
      },
    },
  };
}

export default defineConfig({
  site: "https://bettertui.dev",
  integrations: [
    starlight({
      title: "BetterTUI",
      description: "High-performance terminal UI framework powered by Rust and TypeScript",
      logo: {
        src: "./src/assets/logo.svg",
        alt: "BetterTUI",
      },
      customCss: ["./src/styles/docs.css"],
      head: [{ tag: "script", attrs: { defer: true, src: "/theme-sync.js" } }],
      social: [{ icon: "github", label: "GitHub", href: GITHUB_URL }],
      sidebar: [
        {
          label: "Getting Started",
          items: [
            { label: "Introduction", slug: "getting-started/introduction" },
            { label: "Installation", slug: "getting-started/installation" },
            { label: "Quick Start", slug: "getting-started/quick-start" },
          ],
        },
        {
          label: "Core Concepts",
          items: [
            { label: "Architecture", slug: "core-concepts/architecture" },
            {
              label: "Rendering Pipeline",
              slug: "core-concepts/rendering-pipeline",
            },
            { label: "Layout System", slug: "core-concepts/layout-system" },
            { label: "Event System", slug: "core-concepts/event-system" },
          ],
        },
        {
          label: "Components",
          items: [
            { label: "Overview", slug: "components/overview" },
            { label: "Text", slug: "components/text" },
            { label: "Box", slug: "components/box" },
            { label: "Button", slug: "components/button" },
            { label: "Input", slug: "components/input" },
            { label: "Table", slug: "components/table" },
            { label: "Tree", slug: "components/tree" },
            { label: "Dialog", slug: "components/dialog" },
          ],
        },
        {
          label: "Guides",
          items: [
            { label: "Theming", slug: "guides/theming" },
            { label: "Custom Widgets", slug: "guides/custom-widgets" },
            { label: "Animations", slug: "guides/animations" },
            { label: "Migration from Ink", slug: "guides/migration-from-ink" },
          ],
        },
        {
          label: "API Reference",
          items: [
            { label: "Core API", slug: "api/core" },
            { label: "React Bindings", slug: "api/react" },
            { label: "CLI", slug: "api/cli" },
          ],
        },
      ],
    }),
    react(),
    mdx(),
    sitemap(),
    devSearchSync(),
  ],
  vite: {
    plugins: [tailwindcss()],
    resolve: {
      alias: {
        "~": "/src",
      },
    },
  },
  image: {
    service: { entrypoint: "astro/assets/services/sharp" },
  },
});
