# @bettertui/themes

Design-token system for BetterTUI: a default theme and a factory for building custom themes.

## What's inside

- `defaultTheme` — the built-in dark theme.
- `createTheme()` — derive a theme from partial overrides.

## Example

```ts
import { createTheme, defaultTheme } from "@bettertui/themes";

const highContrast = createTheme({ ...defaultTheme, /* overrides */ });
```

## Status

Partial. `defaultTheme` and `createTheme()` are implemented; preset themes (light, high-contrast) are planned.

See [`docs/api/packages/themes.md`](../../docs/api/packages/themes.md).
