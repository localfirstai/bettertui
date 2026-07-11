# Themes

Theming is provided by `@bettertui/themes` on top of the `Theme` type from `@bettertui/shared`.

## Engine theme shape

```ts
interface Theme {
  name: string;
  colors: Record<string, string>;   // primary, secondary, success, warning, danger, background, foreground, border
  borders: { style: string; fg?: string };
}
```

## API

- `defaultTheme` — the built-in default.
- `createTheme(overrides: Partial<Theme>): Theme` — shallow merge over the default.

See [Guides: Theming](guides/theming.md) and the [API doc](api/packages/themes.md).

## Status

Implemented. Built-in presets beyond `defaultTheme` are not yet shipped. The React `Provider`/`useTheme` use a richer, separate theme shape documented in the [react](react.md) doc.
