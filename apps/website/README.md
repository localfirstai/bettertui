# BetterTUI Website

## Purpose

Project website and documentation portal for BetterTUI, hosted at `https://bettertui.dev`. Built with Astro and Starlight.

## Responsibilities

- Landing page with hero section, feature showcase, code examples, and call-to-action.
- Documentation portal with Getting Started, Core Concepts, Components, Guides, and API Reference.
- Responsive design with dark mode support.

## Tech Stack

- **Framework:** Astro 7 + Starlight
- **UI:** React 19, Tailwind CSS v4, shadcn UI components
- **Content:** MDX documentation pages
- **Icons:** Phosphor Icons
- **Animation:** Motion (Framer Motion successor)
- **Fonts:** Inter (body), JetBrains Mono (code)

## Structure

```
src/
  components/
    landing/       # Hero, Features, CodeExample, TerminalDemo, Testimonials, CTA
    ui/            # Button, Badge, Card, Input (shadcn-style)
    Navbar.astro   # Fixed header with theme toggle
    Footer.astro   # 4-column footer
  layouts/
    BaseLayout.astro  # HTML5 boilerplate with SEO meta tags
  pages/
    index.astro       # Landing page
    blog/             # Blog (empty)
  content/
    docs/             # Starlight documentation content
      getting-started/
      core-concepts/
      components/
      guides/
      api/
  styles/
    global.css        # Tailwind v4 with custom theme tokens (OKLCH)
  lib/
    constants.ts      # GitHub URLs, license links
    utils.ts          # cn() helper for Tailwind class merging
```

## Build

```bash
pnpm dev      # Start dev server
pnpm build    # Build for production
pnpm preview  # Preview production build
```

## Notes

- Documentation content is organized under `src/content/docs/` via Starlight's content collections.
- The `components.json` configures shadcn-style component generation.
- `GITHUB_URL` in `src/lib/constants.ts` should be verified — currently points to `localfirstai/bettertui`.
