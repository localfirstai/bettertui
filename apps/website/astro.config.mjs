import mdx from "@astrojs/mdx";
import react from "@astrojs/react";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";
import { GITHUB_URL } from "./src/lib/constants.ts";

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
