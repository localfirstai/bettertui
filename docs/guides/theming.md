# Theming

Theming is built into the Rust engine (`packages/core/crates/engine/src/theme.rs`) and exposed through `@bettertui/react`.

## Types

| Export | Source | Notes |
|--------|--------|-------|
| `Theme` | `@bettertui/react` (re-exported from shared) | `{ name, colors, spacing, borders }` |
| `ThemeColors` | `@bettertui/react` | 21 semantic color tokens |
| `ThemeSpacing` | `@bettertui/react` | 8 spacing values |

## Usage

```tsx
import { Provider, useTheme } from "@bettertui/react";

const dracula = {
  colors: {
    background: "#282a36",
    surface: "#44475a",
    primary: "#bd93f9",
    text: "#f8f8f2",
  },
};

function App() {
  return (
    <Provider theme={dracula}>
      {/* components */}
    </Provider>
  );
}
```

`Provider` accepts optional `theme: Partial<Theme>`. `useTheme()` returns `{ theme, setTheme }`.

## Status

Default dark theme built in. Overrides supported. Light and high-contrast presets not yet shipped.
