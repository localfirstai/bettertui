# Theming

Theming lives in `@bettertui/themes`. It is a small, implemented package built on the `Theme` type from `@bettertui/shared`.

## API

```ts
import { defaultTheme, createTheme } from "@bettertui/themes";
import type { Theme } from "@bettertui/shared";
```

| Export | Type | Notes |
|--------|------|-------|
| `defaultTheme` | `Theme` | name `"default"`; colors `primary, secondary, success, warning, danger, background, foreground, border`; borders `{ style: "single", fg: "#666666" }` |
| `createTheme(overrides: Partial<Theme>): Theme` | function | shallow-merges over `defaultTheme` |
| `Theme` | type (re-export) | from `@bettertui/shared` |

## Usage

```ts
import { createTheme } from "@bettertui/themes";

const dracula = createTheme({
  colors: {
    background: "#282a36",
    foreground: "#f8f8f2",
    primary: "#bd93f9",
    border: "#44475a",
  },
});
```

## React side

`@bettertui/react` provides its own richer `Theme`/`ThemeColors`/`ThemeSpacing` shape via `Provider` and `useTheme()`. That is distinct from the shared `Theme` type — the React theme is a component-authored token set, while `@bettertui/themes` deals with the engine's `Theme` (colors + borders). When integrating, map the React theme onto the engine `Theme` at the render boundary.

```tsx
import { Provider, useTheme } from "@bettertui/react";

function App() {
  return (
    <Provider>
      <Box>...</Box>
    </Provider>
  );
}
```

## Status

Themes are partially implemented: the default theme and factory exist, but built-in presets (light, high-contrast) are not yet shipped.
