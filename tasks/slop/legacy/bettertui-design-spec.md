# BetterTUI — Comprehensive UI Design Specification

## For AI Agent Implementation

- Project Repo is apps/website [SCOPE of Development]

---

## 1. PROJECT OVERVIEW

**BetterTUI** is a TypeScript-first terminal UI framework that works with any frontend framework (React, SolidJS, Astro, etc.). The landing page and documentation site must communicate:

- **Developer-first**: Code is the hero
- **Framework-agnostic**: Works with React, SolidJS, Astro, Vue, Svelte, etc.
- **Terminal-native**: Embraces the terminal aesthetic without being ugly
- **Modern tooling**: TypeScript, shadcn/ui, Tailwind CSS v4

**Tech Stack**: Astro + React + shadcn/ui + Tailwind CSS v4

---

## 2. DESIGN PHILOSOPHY (Synthesis of 3 Reference Sites)

### From shadcn/ui:

- **Minimalist, content-first layouts** — no heavy gradients or decorative noise
- **Semantic token-based theming** — `background`, `foreground`, `muted`, `accent`, `primary`
- **OKLCH color space** for perceptually uniform colors
- **Clean card-based UI previews** on the homepage
- **Monospace/code blocks as visual anchors**
- **Neutral/stone base palette** with subtle warmth

### From Tailwind CSS:

- **Massive, bold typography** — oversized headlines that command attention
- **Live interactive demos** embedded directly on the page
- **Code + visual side-by-side** — show the code, show the result
- **Dark mode as a first-class feature** — seamless toggle, not an afterthought
- **Generous whitespace** — sections breathe, content is never cramped
- **Subtle grid/dot patterns** in hero backgrounds
- **Real-world UI mockups** as proof-of-concept (not abstract illustrations)

### From T3 Code:

- **Terminal/command-line aesthetic** — monospace fonts, shell prompts, CLI commands
- **GitHub-style code blocks** with syntax highlighting
- **Trust signals** — GitHub stars, download counts, "used by" badges
- **Clear CTAs** — single primary action per section
- **Open-source ethos** — "Fork it", "Open source", community-driven
- **Clean, structured feature lists** with icons

---

## 3. COLOR SYSTEM (Exact shadcn/ui Theme)

### 3.1 Semantic Tokens (OKLCH)

Use the **exact shadcn/ui neutral theme** with these values:

#### Light Mode (`:root`):

```css
:root {
  --radius: 0.625rem;
  --background: oklch(1 0 0); /* #ffffff */
  --foreground: oklch(0.145 0 0); /* #242424 */
  --card: oklch(1 0 0); /* #ffffff */
  --card-foreground: oklch(0.145 0 0); /* #242424 */
  --popover: oklch(1 0 0); /* #ffffff */
  --popover-foreground: oklch(0.145 0 0); /* #242424 */
  --primary: oklch(0.205 0 0); /* #171717 */
  --primary-foreground: oklch(0.985 0 0); /* #fafafa */
  --secondary: oklch(0.97 0 0); /* #f7f7f7 */
  --secondary-foreground: oklch(0.205 0 0); /* #171717 */
  --muted: oklch(0.97 0 0); /* #f7f7f7 */
  --muted-foreground: oklch(0.556 0 0); /* #737373 */
  --accent: oklch(0.97 0 0); /* #f7f7f7 */
  --accent-foreground: oklch(0.205 0 0); /* #171717 */
  --destructive: oklch(0.577 0.245 27.325); /* #ef4444 */
  --border: oklch(0.922 0 0); /* #e5e5e5 */
  --input: oklch(0.922 0 0); /* #e5e5e5 */
  --ring: oklch(0.708 0 0); /* #a3a3a3 */
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0 0); /* #fafafa */
  --sidebar-foreground: oklch(0.145 0 0); /* #242424 */
  --sidebar-primary: oklch(0.205 0 0); /* #171717 */
  --sidebar-primary-foreground: oklch(0.985 0 0); /* #fafafa */
  --sidebar-accent: oklch(0.97 0 0); /* #f7f7f7 */
  --sidebar-accent-foreground: oklch(0.205 0 0); /* #171717 */
  --sidebar-border: oklch(0.922 0 0); /* #e5e5e5 */
  --sidebar-ring: oklch(0.708 0 0); /* #a3a3a3 */
}
```

#### Dark Mode (`.dark`):

```css
.dark {
  --background: oklch(0.145 0 0); /* #242424 */
  --foreground: oklch(0.985 0 0); /* #fafafa */
  --card: oklch(0.205 0 0); /* #2e2e2e */
  --card-foreground: oklch(0.985 0 0); /* #fafafa */
  --popover: oklch(0.205 0 0); /* #2e2e2e */
  --popover-foreground: oklch(0.985 0 0); /* #fafafa */
  --primary: oklch(0.922 0 0); /* #fafafa */
  --primary-foreground: oklch(0.205 0 0); /* #171717 */
  --secondary: oklch(0.269 0 0); /* #3f3f3f */
  --secondary-foreground: oklch(0.985 0 0); /* #fafafa */
  --muted: oklch(0.269 0 0); /* #3f3f3f */
  --muted-foreground: oklch(0.708 0 0); /* #a3a3a3 */
  --accent: oklch(0.269 0 0); /* #3f3f3f */
  --accent-foreground: oklch(0.985 0 0); /* #fafafa */
  --destructive: oklch(0.704 0.191 22.216); /* #ef4444 */
  --border: oklch(1 0 0 / 10%); /* rgba(255,255,255,0.1) */
  --input: oklch(1 0 0 / 15%); /* rgba(255,255,255,0.15) */
  --ring: oklch(0.556 0 0); /* #737373 */
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.205 0 0); /* #2e2e2e */
  --sidebar-foreground: oklch(0.985 0 0); /* #fafafa */
  --sidebar-primary: oklch(0.488 0.243 264.376);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.269 0 0); /* #3f3f3f */
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%); /* rgba(255,255,255,0.1) */
  --sidebar-ring: oklch(0.556 0 0); /* #737373 */
}
```

### 3.2 Tailwind v4 @theme Configuration

```css
@import "tailwindcss";

@custom-variant dark (&:is(.dark *));

@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-card: var(--card);
  --color-card-foreground: var(--card-foreground);
  --color-popover: var(--popover);
  --color-popover-foreground: var(--popover-foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-secondary: var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-accent: var(--accent);
  --color-accent-foreground: var(--accent-foreground);
  --color-destructive: var(--destructive);
  --color-border: var(--border);
  --color-input: var(--input);
  --color-ring: var(--ring);
  --color-chart-1: var(--chart-1);
  --color-chart-2: var(--chart-2);
  --color-chart-3: var(--chart-3);
  --color-chart-4: var(--chart-4);
  --color-chart-5: var(--chart-5);
  --color-sidebar: var(--sidebar);
  --color-sidebar-foreground: var(--sidebar-foreground);
  --color-sidebar-primary: var(--sidebar-primary);
  --color-sidebar-primary-foreground: var(--sidebar-primary-foreground);
  --color-sidebar-accent: var(--sidebar-accent);
  --color-sidebar-accent-foreground: var(--sidebar-accent-foreground);
  --color-sidebar-border: var(--sidebar-border);
  --color-sidebar-ring: var(--sidebar-ring);

  --radius-sm: calc(var(--radius) * 0.6);
  --radius-md: calc(var(--radius) * 0.8);
  --radius-lg: var(--radius);
  --radius-xl: calc(var(--radius) * 1.4);
  --radius-2xl: calc(var(--radius) * 1.8);
  --radius-3xl: calc(var(--radius) * 2.2);
  --radius-4xl: calc(var(--radius) * 2.6);
}

@layer base {
  * {
    @apply border-border outline-ring/50;
  }
  body {
    @apply bg-background text-foreground;
  }
}
```

### 3.3 BetterTUI Brand Accents (Beyond shadcn defaults)

Add these brand-specific tokens for BetterTUI's terminal identity:

```css
:root {
  /* Terminal green accent for BetterTUI brand */
  --terminal: oklch(0.65 0.22 145); /* Terminal green */
  --terminal-foreground: oklch(0.145 0 0);
  --terminal-muted: oklch(0.65 0.22 145 / 0.15);

  /* Code block backgrounds */
  --code-bg: oklch(0.97 0 0);
  --code-border: oklch(0.922 0 0);

  /* Hero gradient subtle */
  --hero-gradient-start: oklch(0.97 0 0);
  --hero-gradient-end: oklch(1 0 0);
}

.dark {
  --terminal: oklch(0.75 0.18 145); /* Brighter green in dark */
  --terminal-foreground: oklch(0.145 0 0);
  --terminal-muted: oklch(0.75 0.18 145 / 0.15);

  --code-bg: oklch(0.205 0 0);
  --code-border: oklch(1 0 0 / 10%);

  --hero-gradient-start: oklch(0.145 0 0);
  --hero-gradient-end: oklch(0.205 0 0);
}

@theme inline {
  --color-terminal: var(--terminal);
  --color-terminal-foreground: var(--terminal-foreground);
  --color-terminal-muted: var(--terminal-muted);
  --color-code-bg: var(--code-bg);
  --color-code-border: var(--code-border);
}
```

---

## 4. TYPOGRAPHY SYSTEM

### 4.1 Font Stack

```css
@theme inline {
  --font-sans:
    "Inter", "Inter Fallback", system-ui, -apple-system, BlinkMacSystemFont,
    "Segoe UI", Roboto, sans-serif;
  --font-mono:
    "JetBrains Mono", "JetBrains Mono Fallback", "Fira Code", "SF Mono",
    "Cascadia Code", monospace;
  --font-display: "Inter", "Inter Fallback", system-ui, sans-serif;
}
```

**Font Loading**: Use `font-display: swap` with preconnect to Google Fonts or self-host Inter + JetBrains Mono.

### 4.2 Type Scale

| Token              | Size (Mobile)    | Size (Desktop ≥1024px) | Line Height | Letter Spacing | Weight |
| ------------------ | ---------------- | ---------------------- | ----------- | -------------- | ------ |
| `text-hero`        | 2.5rem (40px)    | 4.5rem (72px)          | 1.05        | -0.03em        | 700    |
| `text-hero-sub`    | 1.125rem (18px)  | 1.5rem (24px)          | 1.5         | -0.01em        | 400    |
| `text-section`     | 2rem (32px)      | 3rem (48px)            | 1.1         | -0.02em        | 700    |
| `text-section-sub` | 1rem (16px)      | 1.25rem (20px)         | 1.5         | 0              | 400    |
| `text-body-lg`     | 1.125rem (18px)  | 1.125rem (18px)        | 1.6         | 0              | 400    |
| `text-body`        | 1rem (16px)      | 1rem (16px)            | 1.6         | 0              | 400    |
| `text-small`       | 0.875rem (14px)  | 0.875rem (14px)        | 1.5         | 0              | 400    |
| `text-code`        | 0.8125rem (13px) | 0.875rem (14px)        | 1.6         | 0              | 400    |
| `text-label`       | 0.75rem (12px)   | 0.75rem (12px)         | 1.4         | 0.05em         | 500    |

**Implementation with Tailwind v4 fluid typography**:

```css
@theme inline {
  --text-hero: clamp(2.5rem, 1.5rem + 4vw, 4.5rem);
  --text-hero--line-height: 1.05;
  --text-hero--letter-spacing: -0.03em;
  --text-hero--font-weight: 700;

  --text-section: clamp(2rem, 1.5rem + 2vw, 3rem);
  --text-section--line-height: 1.1;
  --text-section--letter-spacing: -0.02em;
  --text-section--font-weight: 700;
}
```

### 4.3 Code Typography

- **Inline code**: `font-mono text-sm bg-muted px-1.5 py-0.5 rounded-md border border-border`
- **Code blocks**: `font-mono text-code bg-code-bg border border-code-border rounded-xl p-4 overflow-x-auto`
- **Syntax highlighting**: Use Shiki with the `github-light` / `github-dark` theme (or `vitesse-light` / `vitesse-dark`)

---

## 5. SPACING SYSTEM

### 5.1 Section Spacing

| Breakpoint            | Section Padding Y | Container Padding X | Max Width |
| --------------------- | ----------------- | ------------------- | --------- |
| Mobile (<640px)       | 4rem (64px)       | 1rem (16px)         | 100%      |
| Tablet (640-1023px)   | 5rem (80px)       | 1.5rem (24px)       | 100%      |
| Desktop (1024-1279px) | 6rem (96px)       | 2rem (32px)         | 1200px    |
| Wide (≥1280px)        | 8rem (128px)      | 2rem (32px)         | 1280px    |

### 5.2 Container

```css
.container {
  @apply mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 xl:px-8;
}
```

### 5.3 Component Spacing

- **Card padding**: `p-6` (24px)
- **Card gap in grids**: `gap-6` (24px)
- **Button padding**: `px-5 py-2.5` (20px × 10px)
- **Input padding**: `px-4 py-2.5` (16px × 10px)
- **Section title to content**: `mt-4` (16px)
- **Feature item gap**: `gap-4` (16px)

---

## 6. COMPONENT DESIGN SPECIFICATIONS

### 6.1 Navigation Bar

**Layout**:

- Fixed top, `z-50`, full width
- Height: `h-16` (64px)
- Background: `bg-background/80 backdrop-blur-xl border-b border-border`
- Container: centered, max-w-7xl

**Left**: Logo + "BetterTUI" wordmark

- Logo: Terminal icon (Lucide `Terminal` or custom SVG), `w-6 h-6 text-terminal`
- Wordmark: `font-sans text-lg font-semibold tracking-tight`

**Center**: Nav links (hidden on mobile, shown on md+)

- Links: Docs, Components, Examples, Blog
- Style: `text-sm font-medium text-muted-foreground hover:text-foreground transition-colors`
- Active state: `text-foreground`

**Right**:

- GitHub star count badge (shadcn `Badge` variant="secondary")
- Dark mode toggle (shadcn `Button` variant="ghost" size="icon")
- "Get Started" CTA button (shadcn `Button`)

**Mobile**: Hamburger menu → Sheet/drawer with nav links

### 6.2 Hero Section

**Layout**: Full-width, min-height `min-h-[80vh]`, centered content

**Background**:

- Subtle dot grid pattern (CSS `radial-gradient` or SVG)
- Color: `oklch(0.556 0 0 / 0.08)` dots on `bg-background`
- In dark mode: `oklch(0.556 0 0 / 0.12)` dots
- Optional: very subtle radial gradient from center: `radial-gradient(ellipse at center, var(--hero-gradient-start) 0%, var(--hero-gradient-end) 70%)`

**Content Structure** (centered, max-w-4xl):

1. **Announcement Badge** (optional, above headline):

   ```
   <Badge variant="secondary" className="rounded-full px-3 py-1">
     <span className="text-terminal mr-1.5">●</span>
     v1.0 is now available
   </Badge>
   ```

2. **Headline** (text-hero):

   ```
   "Build Terminal UIs with
   Any Framework"
   ```
   - Use `text-balance` for natural line breaks
   - The word "Any" should have `text-terminal` color for accent

3. **Subheadline** (text-hero-sub, text-muted-foreground):

   ```
   "BetterTUI is the TypeScript-first toolkit for building beautiful,
   interactive terminal interfaces. Works with React, SolidJS, Astro,
   Vue, Svelte, and more."
   ```
   - Max-width: `max-w-2xl`
   - `text-balance` applied

4. **CTA Group** (flex row, gap-3, centered):
   - Primary: `Button size="lg"` — "Get Started" → `/docs`
   - Secondary: `Button variant="outline" size="lg"` — "View on GitHub" → external
   - Tertiary (copy-paste style):
     ```
     <div className="flex items-center gap-2 rounded-lg border bg-muted px-4 py-2.5 font-mono text-sm">
       <span className="text-muted-foreground">$</span>
       <span>npm install bettertui</span>
       <Button variant="ghost" size="icon" className="h-6 w-6">
         <Copy className="h-3.5 w-3.5" />
       </Button>
     </div>
     ```

5. **Hero Demo** (below CTAs, mt-16):
   - A live terminal mockup showing BetterTUI in action
   - Use a styled `div` that looks like a terminal window:
     - Top bar: 3 colored dots (red, yellow, green) + title
     - Content: Monospace text with syntax highlighting showing a component
   - Max-width: `max-w-3xl`
   - Shadow: `shadow-2xl shadow-black/5`
   - Border: `border border-border rounded-xl`
   - In dark mode: `shadow-black/20`

### 6.3 Framework Logos Strip

**Layout**: Full-width, `py-8`, `border-y border-border`, `bg-muted/30`

**Content**:

- Label: "Works with" — `text-xs font-medium text-muted-foreground uppercase tracking-wider text-center mb-6`
- Logo row: Flex, `justify-center`, `gap-8 md:gap-12`, `items-center`, `flex-wrap`
- Logos: React, SolidJS, Astro, Vue, Svelte, Preact, Angular (use SVG icons, `h-6 md:h-8`, `opacity-50 hover:opacity-100 transition-opacity`)
- Grayscale by default, full color on hover

### 6.4 Features Section

**Layout**: Container, `py-24`

**Header**:

- Eyebrow: `text-label text-terminal uppercase` — "Features"
- Title: `text-section` — "Everything you need for terminal UIs"
- Subtitle: `text-section-sub text-muted-foreground max-w-2xl` — "From interactive forms to real-time dashboards, BetterTUI provides the primitives you need."

**Grid**: `grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-12`

**Feature Card** (shadcn `Card`):

```
<Card className="group relative overflow-hidden border-border/60 bg-card/50 backdrop-blur-sm hover:border-border transition-all duration-300">
  <CardContent className="p-6">
    <div className="mb-4 inline-flex h-10 w-10 items-center justify-center rounded-lg bg-terminal-muted text-terminal">
      <Icon className="h-5 w-5" />
    </div>
    <h3 className="text-lg font-semibold">{title}</h3>
    <p className="mt-2 text-sm text-muted-foreground leading-relaxed">{description}</p>
  </CardContent>
</Card>
```

**Feature List** (6 cards):

1. **Framework Agnostic** — `Globe` icon — "Use React, SolidJS, Astro, or any framework. One API, infinite possibilities."
2. **Type-Safe** — `Shield` icon — "Full TypeScript support with autocomplete and compile-time safety."
3. **Interactive Components** — `MousePointerClick` icon — "Buttons, inputs, selects, tables, and more — all terminal-native."
4. **Theming** — `Palette` icon — "Built-in dark mode and custom theme support with CSS variables."
5. **Keyboard Navigation** — `Keyboard` icon — "Full keyboard support with focus management and shortcuts."
6. **Lightweight** — `Feather` icon — "Zero runtime overhead. Tree-shakeable and dependency-free core."

### 6.5 Code + Preview Section ("How it works")

**Layout**: Container, `py-24`, alternating left/right

**Pattern** (inspired by Tailwind CSS homepage):

- Two-column layout on desktop: `grid grid-cols-1 lg:grid-cols-2 gap-12 items-center`
- On mobile: stacked, code first then preview

**Example Block 1 — "Define your UI"**:

- Left: Code block showing a BetterTUI component

  ```tsx
  import { Box, Text, useInput } from "bettertui";

  function App() {
    const [count, setCount] = useState(0);

    useInput((input) => {
      if (input === "q") process.exit();
    });

    return (
      <Box borderStyle="round" padding={1}>
        <Text color="green">Count: {count}</Text>
        <Text dimColor>Press 'q' to quit</Text>
      </Box>
    );
  }
  ```

- Right: Visual mockup of the terminal output
  - Styled box with rounded border
  - Green text "Count: 0"
  - Dimmed text below

**Example Block 2 — "Any Framework"**:

- Left: Visual mockup (framework logos in a terminal)
- Right: Code block showing framework adapters

  ```tsx
  // React
  import { render } from "bettertui/react";

  // SolidJS
  import { render } from "bettertui/solid";

  // Astro
  import BetterTUI from "bettertui/astro";
  ```

**Code Block Styling**:

```
<div className="relative rounded-xl border border-code-border bg-code-bg overflow-hidden">
  <div className="flex items-center gap-1.5 px-4 py-3 border-b border-code-border">
    <div className="h-3 w-3 rounded-full bg-red-400/80" />
    <div className="h-3 w-3 rounded-full bg-yellow-400/80" />
    <div className="h-3 w-3 rounded-full bg-green-400/80" />
    <span className="ml-2 text-xs text-muted-foreground font-mono">App.tsx</span>
  </div>
  <pre className="p-4 overflow-x-auto">
    <code className="font-mono text-sm">{/* Syntax highlighted code */}</code>
  </pre>
</div>
```

### 6.6 Terminal Demo Section

**Layout**: Full-width, `py-24`, `bg-muted/30`

**Content**:

- Header: "See it in action" + subtitle
- Large terminal mockup (max-w-4xl centered):
  - Actual interactive demo if possible, or animated GIF/video
  - Shows a multi-step form, a dashboard, or a real-time app
  - Terminal chrome: title bar, scrollable content area
  - Use `shadow-2xl` for depth

### 6.7 Testimonials / Social Proof

**Layout**: Container, `py-24`

**Content**:

- Eyebrow: "Loved by developers"
- Grid of 3 tweet-style cards
- Each card: Avatar, name, handle, tweet text, timestamp
- Use shadcn `Card` with `variant="outline"`
- Border: `border-border/60`

### 6.8 CTA Section

**Layout**: Container, `py-24`

**Content**:

- Large centered card with `bg-primary text-primary-foreground`
- Or: `border border-border rounded-2xl p-12 md:p-16 text-center`
- Title: "Ready to build better terminal apps?"
- Subtitle: "Get started in minutes with our CLI installer."
- Buttons: "Get Started" (primary) + "Read the Docs" (outline)
- Background: Subtle terminal pattern or gradient

### 6.9 Footer

**Layout**: Full-width, `border-t border-border`, `bg-muted/30`, `py-12`

**Content**:

- Grid: `grid grid-cols-2 md:grid-cols-4 gap-8`
- Column 1: Logo + description + social links
- Columns 2-4: Link groups (Product, Resources, Community)
- Bottom bar: Copyright + "Built with Astro" + GitHub link
- Link style: `text-sm text-muted-foreground hover:text-foreground transition-colors`

---

## 7. INTERACTIONS & ANIMATIONS

### 7.1 Global Transitions

```css
@theme inline {
  --ease-out-expo: cubic-bezier(0.16, 1, 0.3, 1);
  --ease-in-out-expo: cubic-bezier(0.87, 0, 0.13, 1);
}
```

- Default transition: `transition-all duration-200 ease-out`
- Button hover: `transition-colors duration-200`
- Card hover: `transition-all duration-300 hover:shadow-lg hover:border-border`
- Link hover: `transition-colors duration-150`

### 7.2 Scroll Animations

Use a lightweight intersection observer or `framer-motion` (if React islands):

- Sections fade in + translate-y(20px → 0) on scroll
- Stagger children in grids (0.05s delay per item)
- Duration: 0.6s, Easing: `cubic-bezier(0.16, 1, 0.3, 1)`

### 7.3 Hero Entrance

- Headline: Fade in + translate-y, 0.6s delay
- Subtitle: Fade in, 0.3s after headline
- CTAs: Fade in, 0.3s after subtitle
- Terminal demo: Scale(0.95 → 1) + fade in, 0.4s after CTAs

### 7.4 Hover Effects

- Feature cards: `hover:-translate-y-0.5 hover:shadow-lg`
- Buttons: Standard shadcn hover states
- Code blocks: `hover:border-terminal/30 transition-colors`
- Terminal dots in code blocks: Pulse animation on the green dot

### 7.5 Dark Mode Toggle

- Smooth transition between modes: `transition-colors duration-300` on `html`
- Use `next-themes` or a custom theme provider
- Store preference in localStorage
- Respect `prefers-color-scheme` on first visit

---

## 8. RESPONSIVE BREAKPOINTS

Tailwind v4 defaults:

- `sm`: 640px
- `md`: 768px
- `lg`: 1024px
- `xl`: 1280px
- `2xl`: 1536px

### Key Responsive Rules:

| Element           | Mobile             | Tablet     | Desktop      |
| ----------------- | ------------------ | ---------- | ------------ |
| Hero headline     | 2.5rem             | 3.5rem     | 4.5rem       |
| Section headline  | 2rem               | 2.5rem     | 3rem         |
| Feature grid      | 1 col              | 2 col      | 3 col        |
| Code + Preview    | Stacked            | Stacked    | Side-by-side |
| Nav links         | Hidden (hamburger) | Hidden     | Visible      |
| Container padding | 16px               | 24px       | 32px         |
| Section padding Y | 64px               | 80px       | 96-128px     |
| Terminal demo     | Full width         | Full width | max-w-3xl    |

---

## 9. SHADCN/UI COMPONENT USAGE

Install these shadcn components:

```bash
npx shadcn add button
npx shadcn add badge
npx shadcn add card
npx shadcn add sheet
npx shadcn add separator
npx shadcn add scroll-area
npx shadcn add tabs
npx shadcn add tooltip
```

### Component Customizations:

**Button**:

- Default: `bg-primary text-primary-foreground hover:bg-primary/90`
- Outline: `border-border bg-background hover:bg-muted hover:text-foreground`
- Ghost: `hover:bg-muted hover:text-foreground`
- Size lg: `h-11 px-8 text-base`

**Card**:

- Default: `bg-card text-card-foreground border-border/60`
- Use `backdrop-blur-sm` for glassmorphism effect on feature cards

**Badge**:

- Secondary: `bg-secondary text-secondary-foreground hover:bg-secondary/80`
- Terminal accent variant: `bg-terminal-muted text-terminal border-terminal/20`

**Sheet** (mobile nav):

- `bg-background border-border`
- Width: `max-w-sm`

---

## 10. ASSETS & ICONOGRAPHY

### 10.1 Icons

Use **Lucide React** for all icons (consistent with shadcn/ui):

- Navigation: `Terminal`, `Menu`, `X`, `Moon`, `Sun`, `Github`
- Features: `Globe`, `Shield`, `MousePointerClick`, `Palette`, `Keyboard`, `Feather`
- UI: `Copy`, `Check`, `ChevronRight`, `ExternalLink`, `ArrowRight`

Icon sizing:

- Nav icons: `h-5 w-5`
- Feature icons: `h-5 w-5`
- Inline icons: `h-4 w-4`
- Social icons: `h-5 w-5`

### 10.2 Logo

BetterTUI logo:

- Terminal window icon with a subtle `>` prompt
- Primary color: `text-terminal` (green accent)
- Size: `h-6 w-6` in nav, `h-8 w-8` in footer
- Consider a simple SVG: rounded rectangle with `>_` inside

### 10.3 Framework Logos

Use official SVGs for:

- React, SolidJS, Astro, Vue, Svelte, Preact
- Display in grayscale (`filter: grayscale(100%) opacity(0.5)`)
- On hover: `filter: grayscale(0%) opacity(1)`

### 10.4 OG Image / Meta

- 1200×630px
- Dark background with terminal green accent
- "BetterTUI" headline + "Terminal UIs for any framework" subtitle
- Use Inter Bold for headline, JetBrains Mono for code snippet

---

## 11. PERFORMANCE & ACCESSIBILITY

### 11.1 Performance

- Use Astro's static generation (`output: 'static'`)
- Lazy load below-fold images with `loading="lazy"`
- Use `font-display: swap` for web fonts
- Preconnect to font CDN: `<link rel="preconnect" href="https://fonts.googleapis.com">`
- Minimize JavaScript — use React islands only for interactive components
- Code splitting per page

### 11.2 Accessibility

- All interactive elements must be keyboard accessible
- Focus rings: `ring-2 ring-ring ring-offset-2 ring-offset-background`
- Color contrast: Minimum 4.5:1 for body text, 3:1 for large text
- `prefers-reduced-motion`: Disable animations, instant transitions
- Semantic HTML: `<nav>`, `<main>`, `<section>`, `<footer>`, `<article>`
- Alt text for all images
- `aria-label` for icon-only buttons
- Skip-to-content link

### 11.3 SEO

- Title: "BetterTUI — Terminal UIs for Any Framework"
- Meta description: "Build beautiful, interactive terminal user interfaces with React, SolidJS, Astro, and more. TypeScript-first. Zero runtime overhead."
- Open Graph tags
- Twitter Card tags
- Structured data (JSON-LD) for SoftwareApplication

---

## 12. PAGE STRUCTURE (Astro)

```
src/
├── layouts/
│   └── Layout.astro          # Root layout with theme provider
├── pages/
│   ├── index.astro           # Landing page
│   └── docs/
│       └── [...slug].astro   # Documentation pages
├── components/
│   ├── ui/                   # shadcn components
│   ├── Navbar.tsx            # React island
│   ├── HeroSection.astro
│   ├── FeatureCard.astro
│   ├── CodeBlock.astro       # With Shiki syntax highlighting
│   ├── TerminalDemo.tsx      # React island (interactive)
│   ├── FrameworkLogos.astro
│   ├── Footer.astro
│   ├── ThemeProvider.tsx     # next-themes wrapper
│   └── ThemeToggle.tsx       # React island
├── styles/
│   └── globals.css           # Tailwind v4 + theme tokens
└── content/
    └── docs/                 # MDX documentation
```

---

## 13. IMPLEMENTATION CHECKLIST FOR AI AGENT

### Phase 1: Setup

- [ ] Initialize Astro project with React integration
- [ ] Install Tailwind CSS v4 (`npm install tailwindcss @tailwindcss/vite`)
- [ ] Configure `vite.config.ts` with Tailwind plugin
- [ ] Install and configure shadcn/ui (`npx shadcn@latest init`)
- [ ] Set up `globals.css` with exact theme tokens from Section 3
- [ ] Install `next-themes` or custom theme provider
- [ ] Configure fonts (Inter + JetBrains Mono) via Google Fonts or self-host

### Phase 2: Core Components

- [ ] Create `Layout.astro` with proper `<head>` (fonts, meta, OG tags)
- [ ] Create `ThemeProvider.tsx` wrapping the app
- [ ] Create `Navbar.tsx` (React island) with mobile Sheet menu
- [ ] Create `ThemeToggle.tsx` (React island)
- [ ] Create `Footer.astro`

### Phase 3: Landing Page Sections

- [ ] Hero section with dot grid background, headline, CTAs, terminal mockup
- [ ] Framework logos strip
- [ ] Features grid (6 cards)
- [ ] Code + Preview alternating sections (2-3 blocks)
- [ ] Terminal demo section
- [ ] Testimonials section (optional)
- [ ] CTA section

### Phase 4: Polish

- [ ] Add scroll animations (Framer Motion or CSS)
- [ ] Add copy-to-clipboard for install command
- [ ] Verify dark mode works perfectly (all tokens switch)
- [ ] Test responsive breakpoints (mobile, tablet, desktop)
- [ ] Verify accessibility (keyboard nav, focus rings, contrast)
- [ ] Add SEO meta tags and OG image
- [ ] Performance audit (Lighthouse)

### Phase 5: Documentation (Optional)

- [ ] Set up MDX content collection
- [ ] Create docs layout with sidebar
- [ ] Style prose content with Tailwind typography plugin

---

## 14. KEY DESIGN DECISIONS SUMMARY

| Decision                         | Rationale                                                   |
| -------------------------------- | ----------------------------------------------------------- |
| **shadcn/ui neutral theme**      | Clean, professional, works perfectly for developer tools    |
| **Terminal green accent**        | Brand identity — signals "terminal" without being gimmicky  |
| **OKLCH color space**            | Perceptually uniform, modern standard, Tailwind v4 default  |
| **CSS-first Tailwind v4**        | No JS config, native CSS variables, runtime theme switching |
| **Astro + React islands**        | Fast static site with interactive components where needed   |
| **JetBrains Mono**               | Best-in-class programming font, free, widely loved          |
| **No heavy gradients**           | shadcn/ui philosophy — content-first, minimal decoration    |
| **Live code blocks**             | Tailwind CSS pattern — show, don't tell                     |
| **Terminal mockups as hero**     | T3 Code pattern — CLI aesthetic as visual identity          |
| **Framework-agnostic messaging** | Core value prop — must be immediately clear                 |

---

_End of Design Specification_
