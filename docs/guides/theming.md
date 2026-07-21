# Theming

Theming is built into the Rust engine (`packages/core/crates/engine/src/theme.rs`)
and exposed through `@bettertui/react` (which re-exports the types). The React `Provider` accepts
`Partial<Theme>` directly.

## Types

| Export | Source | Notes |
|--------|--------|-------|
| `Theme` | `@bettertui/react` | Internal package; re-exported for public consumption |
| `ThemeColors` | `@bettertui/react` | 21 semantic color tokens |
| `ThemeSpacing` | `@bettertui/react` | 8 spacing values |
| `BorderStyle` | `@bettertui/react` | Default border style + color |

## Usage

```tsx
import { Provider } from "@bettertui/react";
import type { Theme } from "@bettertui/react";

const dracula: Partial<Theme> = {
  colors: {
    background: "#282a36",
    surface: "#44475a",
    primary: "#bd93f9",
    text: "#f8f8f2",
    border: "#6272a4",
  },
};

function App() {
  return (
    <Provider theme={dracula}>
      <Box>...</Box>
    </Provider>
  );
}
```

## React Provider

`Provider` accepts an optional `theme: Partial<Theme>` prop. If omitted, the default dark
theme is used. `useTheme()` returns `{ theme, setTheme }` where `setTheme(partial)`
deep-merges the partial overrides into the current theme.

```tsx
import { Provider, useTheme } from "@bettertui/react";

function ThemeSwitcher() {
  const { theme, setTheme } = useTheme();

  return (
    <Button onClick={() => setTheme({ colors: { primary: "#ff0000" } })}>
      Red Primary
    </Button>
  );
}
```

## Status

The default dark theme is built in. The React Provider supports arbitrary overrides.
Built-in presets (light, high-contrast) are not yet shipped.
