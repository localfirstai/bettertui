## Project Overview

This is an Astro project. The primary goals are:

- **Responsive Design** — Make the website highly responsive across all device sizes using Tailwind CSS v4 responsive utilities.
- **shadcn UI Components** — Use shadcn standard UI components for both Astro and React. All base UI components live in `src/components/ui/`.
- **SEO Optimization** — Maximize SEO using Astro's built-in features (static page generation, sitemap, canonical URLs, meta tags, structured data). Prefer `.astro` components over `.tsx` for content-oriented pages since they are server-rendered with zero client JS.

## Critical Rules — Comments

These rules are **critical** and apply to every change:

- **Always avoid writing unnecessary comments.** Never write comments that restate what the code already says (e.g. `// set x to 1` next to `x = 1`). Self-explanatory code gets no comment at all.
- **TypeScript:** follow **JSDoc style only** — `/** ... */` doc comments. No other documentation style.
- **No comments inside the code body** to explain logic; if a concept genuinely needs explaining, write it as a proper JSDoc comment on the relevant function, type, or module instead of an inline `//` comment.

## Development

Start dev server:

```sh
pnpm dev          # foreground (standard)
pnpm dev -- --background   # background for CI/automation
```

Stop a background server: `pnpm exec astro dev stop`

The dev server runs at **http://localhost:4321** with live HMR.

## SVG Download

Add SVGs using the shadcn/ui CLI

```bash
pnpm dlx shadcn@latest add @svgl/sanity
```

Add multiple SVGs at once:

```bash
pnpm dlx shadcn@latest add @svgl/sanity @svgl/github @svgl/supabase @svgl/vercel
```

Or use Shadcn MCP Server (Recommendation)

```
# Prompts
> Can you add the "GitHub" SVG from SVGL registry?
> Please add React, Svelte and Vue SVGs from SVGL registry.
```

## Documentation

Full documentation: https://docs.astro.build

Consult these guides before working on related tasks:

- [Adding pages, dynamic routes, or middleware](https://docs.astro.build/en/guides/routing/)
- [Working with Astro components](https://docs.astro.build/en/basics/astro-components/)
- [Using React, Vue, Svelte, or other framework components](https://docs.astro.build/en/guides/framework-components/)
- [Adding or managing content](https://docs.astro.build/en/guides/content-collections/)
- [Adding styles or using Tailwind](https://docs.astro.build/en/guides/styling/)
- [Supporting multiple languages](https://docs.astro.build/en/guides/internationalization/)
