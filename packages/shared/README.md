# @bettertui/shared

**Type-only package. Zero runtime code.** The shared vocabulary used by every other package.

## Exports

Types: `NodeId`, `Point`, `Size`, `Rect`, `FlexDirection`, `JustifyContent`, `AlignItems`, `AlignSelf`, `Position`, `Sizing`, `Overflow`, `Padding`, `Margin`, `Inset`, `Gap`, `LayoutConstraints`, `KeyEvent`, `KeyEventSource`, `KeyEventType`, `MouseButton`, `MouseEvent`, `ColorValue`, `Style`, `BorderStyleKind`, `BorderStyle`, `Display`, `FlexWrap`, `ThemeColors`, `ThemeSpacing`, `Theme`, `ValidationError`, `ValidationResult`

Constants: `COLOR_REGEX`, `RGB_REGEX`, `RGBA_REGEX`, `NAMED_COLORS`, `DEFAULT_THEME`, `VALID_ALIGN_ITEMS`, `VALID_ALIGN_SELVES`, `VALID_FLEX_DIRECTIONS`, `VALID_FLEX_WRAPS`, `VALID_JUSTIFY_CONTENTS`, `VALID_OVERFLOWS`, `VALID_POSITIONS`

Functions: `generateId()`, `isValidColor(value)`, `mergeTheme(base, override)`, `validate(layout, style)`, `validateLayoutConstraints(layout)`, `validateStyle(style)`, `warnIfInvalid(layout, style, context?)`

Widget types: `TimelineOptions`, `TweenConfig`

## Notes

- This is the leaf of the dependency graph — depends on nothing internal
- `@bettertui/core` and `@bettertui/react` re-export these types; consumers should import from those packages (shared is an internal package)
