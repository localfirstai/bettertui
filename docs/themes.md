# Themes

Theming is built into the Rust engine (`packages/core/crates/widgets/src/theme.rs`) 
and exposed through `@bettertui/shared` types.

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
import type { Theme } from "@bettertui/shared";

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

- `@bettertui/shared` — Theme, ThemeColors, ThemeSpacing, BorderStyle types
- `@bettertui/react` — Provider, useTheme
- `packages/core/crates/widgets/src/theme.rs` — Rust canonical Theme definition
