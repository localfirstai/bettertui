# @bettertui/widgets

**High-level widget library (intended).** Depends on `@bettertui/core`, `@bettertui/shared`. Currently a **stub**.

## Exports

| Export | Type | Notes |
|--------|------|-------|
| `Widget` | `interface` | `{ type: string; render(): unknown }` |
| `WIDGET_VERSION` | `const` | `"0.0.0"` |

## Status

Only the interface and version constant exist. The real widget framework lives in the Rust engine (`widgets` module, ~200 tests) but is not exposed here yet. Do not document concrete widgets on the TS side.
