# @bettertui/shared

**Type-only package. Zero runtime code.** The shared vocabulary used by every other package.

## Type exports (`src/types`)

| Type | Shape |
|------|-------|
| `NodeId` | `string` (TS side; the Rust engine uses `slotmap::DefaultKey`, transmuted to `u64` across FFI) |
| `Point` | `{ x: number; y: number }` |
| `Size` | `{ width: number; height: number }` |
| `Rect` | `{ x: number; y: number; width: number; height: number }` |
| `FlexDirection` | `"row" \| "column" \| "rowReverse" \| "columnReverse"` |
| `JustifyContent` | `"flexStart" \| "center" \| "flexEnd" \| "spaceBetween" \| "spaceAround" \| "spaceEvenly"` |
| `AlignItems` | `"flexStart" \| "center" \| "flexEnd" \| "stretch"` |
| `AlignSelf` | `"auto" \| "flexStart" \| "center" \| "flexEnd" \| "stretch"` |
| `Position` | `"static" \| "relative" \| "absolute"` |
| `Sizing` | `"auto" \| "content" \| "fill"` |
| `Overflow` | `"visible" \| "hidden" \| "scroll"` |
| `Padding` | `{ top?; right?; bottom?; left? }` |
| `Margin` | `{ top?; right?; bottom?; left? }` |
| `Inset` | `{ top?; right?; bottom?; left? }` |
| `Gap` | `number \| { row?; column? }` |
| `LayoutConstraints` | `{ minWidth?; maxWidth?; minHeight?; maxHeight? }` |
| `KeyEvent` | `{ key: string; code?: string; ctrl?; shift?; alt?; meta? }` |
| `MouseButton` | `"left" \| "right" \| "middle" \| "none"` |
| `MouseEvent` | `{ button: MouseButton; x: number; y: number; ctrl?; shift?; alt?; meta? }` |
| `ColorValue` | `string` |
| `Style` | `{ color?; background?; bold?; italic?; underline?; dim?; strikethrough?; inverse? }` |
| `BorderStyleKind` | `"none" \| "single" \| "double" \| "rounded" \| "thick" \| "block"` |
| `BorderStyle` | `{ kind?: BorderStyleKind; color?: ColorValue }` |
| `ThemeColors` | 21 semantic color tokens (background, surface, primary, text, border, …) |
| `ThemeSpacing` | 8 spacing values (none, xxs, xs, sm, md, lg, xl, xxl) |
| `Theme` | `{ name: string; colors: ThemeColors; spacing: ThemeSpacing; borders: BorderStyle }` |
| `ValidationError` | `{ field: string; message: string }` |
| `ValidationResult` | `{ valid: boolean; errors: ValidationError[] }` |

## Constant exports (`src/consts`)

`COLOR_REGEX`, `RGB_REGEX`, `RGBA_REGEX`, `NAMED_COLORS`, `DEFAULT_THEME`, `VALID_ALIGN_ITEMS`, `VALID_ALIGN_SELVES`, `VALID_FLEX_DIRECTIONS`, `VALID_FLEX_WRAPS`, `VALID_JUSTIFY_CONTENTS`, `VALID_OVERFLOWS`, `VALID_POSITIONS`.

## Function exports (`src/utils`)

`generateId()`, `isValidColor(value)`, `mergeTheme(base, override)`, `validate(layout, style)`, `validateLayoutConstraints(layout)`, `validateStyle(style)`, `warnIfInvalid(layout, style, context?)`.

## Notes

- This is the leaf of the dependency graph — it depends on nothing internal.
- `@bettertui/core` and `@bettertui/react` re-export these types; consumers should import from those packages rather than from `@bettertui/shared` directly (it is an internal package).
