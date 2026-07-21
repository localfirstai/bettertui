# Themes

Theming is built into the Rust engine (`packages/core/crates/engine/src/theme.rs`) 
and exposed through `@bettertui/react` (which re-exports the types).

## Theme shape

```typescript
interface Theme {
  name: string;
  colors: ThemeColors;
  spacing: ThemeSpacing;
  borders: BorderStyle;
}
```

`ThemeColors` has 21 semantic color tokens (background, surface, primary, text, border, accent, scrollbar, etc.).
`ThemeSpacing` has 8 spacing values (none, xxs, xs, sm, md, lg, xl, xxl).
`BorderStyle` specifies the default border style + color.

## React usage

```tsx
import { Provider, useTheme } from "@bettertui/react";
import type { Theme } from "@bettertui/react";

const dracula: Partial<Theme> = {
  colors: {
    background: "#282a36",
    surface: "#44475a",
    primary: "#bd93f9",
    text: "#f8f8f2",
    border: "#6272a4",
    borderFocused: "#bd93f9",
    // ... other colors use defaults
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

## Status

The default dark theme is built into the Rust engine. The React `Provider` accepts arbitrary `Partial<Theme>` overrides. Built-in presets (light, high-contrast) are planned.

## See also

- `@bettertui/react` — Theme, ThemeColors, ThemeSpacing, BorderStyle types (re-exported from `@bettertui/shared`), Provider, useTheme
- `packages/core/crates/engine/src/theme.rs` — Rust canonical Theme definition
