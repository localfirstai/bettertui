# @bettertui/themes

## Purpose

Theme definitions and utilities for BetterTUI. Provides a default theme and a `createTheme()` helper for building custom themes.

## Responsibilities

- **Default theme:** Predefined color palette for terminal UIs.
- **Theme creation:** `createTheme()` merges user overrides with the default theme.
- **Type re-export:** Re-exports `Theme` from `@bettertui/shared`.

## Public API

```typescript
const defaultTheme: Theme;

function createTheme(overrides: Partial<Theme>): Theme;

export type { Theme };
```

### Default theme colors

| Token        | Value     |
|-------------|-----------|
| primary     | `#007acc` |
| secondary   | `#6c757d` |
| success     | `#28a745` |
| warning     | `#ffc107` |
| danger      | `#dc3545` |
| background  | `#1e1e1e` |
| foreground  | `#d4d4d4` |
| border      | `#3c3c3c` |

## Dependencies

- `@bettertui/shared` — imports `Theme`, `BorderStyle`, `ColorValue`

## Consumers

- None currently. The React adapter (`@bettertui/react`) defines its own `Theme` type in its hooks layer.

## Internal Structure

```
src/
  index.ts   # defaultTheme constant, createTheme() helper
```

## Design Principles

- **Framework-agnostic.** Themes are pure data — no React or framework-specific code.
- **Composable.** `createTheme()` accepts partial overrides, allowing incremental customization.

## Example Usage

```typescript
import { createTheme, defaultTheme } from "@bettertui/themes";

const myTheme = createTheme({
  name: "my-app",
  colors: {
    ...defaultTheme.colors,
    primary: "#ff6b6b",
    background: "#0a0a0a",
  },
});
```

## Notes

- The `Theme` type in this package (`colors: Record<string, ColorValue>`) differs from the `Theme` type defined in `@bettertui/react` hooks (`ThemeColors` with specific color fields). A unified theme type should be established before v1.0.
- This package is not currently consumed by `@bettertui/react`. The react adapter uses its own inline theme definition. Integration with this package is planned.
