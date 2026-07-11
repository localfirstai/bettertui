# @bettertui/themes

**Theme definitions and factory.** Depends on `@bettertui/shared`. Implemented.

## Exports

| Export | Type | Notes |
|--------|------|-------|
| `defaultTheme` | `Theme` | name `"default"`; colors `primary, secondary, success, warning, danger, background, foreground, border`; borders `{ style: "single", fg: "#666666" }` |
| `createTheme(overrides: Partial<Theme>): Theme` | function | shallow merge over `defaultTheme` |
| `Theme` | type (re-export) | from `@bettertui/shared` |

## Status

Small but functional. Built-in presets beyond `defaultTheme` are not yet shipped.
