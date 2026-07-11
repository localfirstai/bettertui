# @bettertui/shared

**Type-only package. Zero runtime code.** The shared vocabulary used by every other package.

## Exports (all `type`)

| Type | Shape |
|------|-------|
| `NodeId` | `string` |
| `Point` | `{ x, y }` |
| `Size` | `{ width, height }` |
| `Rect` | `{ x, y, width, height }` |
| `Direction` | `"horizontal" \| "vertical"` |
| `Alignment` | `"start" \| "center" \| "end" \| "stretch"` |
| `Overflow` | `"visible" \| "hidden" \| "scroll"` |
| `LayoutConstraints` | `{ minWidth?, maxWidth?, minHeight?, maxHeight? }` |
| `LayoutResult` | `{ rect, children[] }` |
| `RenderCommand` | `{ type: "text"\|"rect"\|"clear", rect?, text?, style? }` |
| `EventType` | `"key" \| "mouse" \| "resize" \| "focus" \| "blur" \| "custom"` |
| `Event` | `{ type, timestamp, data }` |
| `KeyEvent` | `{ key, code, ctrl, shift, alt, meta }` |
| `MouseButton` | `"left" \| "right" \| "middle" \| "none"` |
| `MouseEvent` | `{ button, position, ctrl, shift, alt }` |
| `ResizeEvent` | `{ columns, rows }` |
| `ColorValue` | `string` |
| `Color` | `{ r, g, b, a? }` |
| `Style` | `{ fg?, bg?, bold?, italic?, underline?, dim?, strikethrough?, inverse? }` |
| `BorderStyle` | `{ style, fg? }` where `style: "none"\|"single"\|"double"\|"rounded"\|"thick"\|"block"` |
| `Theme` | `{ name, colors, borders }` |
| `Frame` | `{ width, height, cells[] }` |
| `FrameCell` | `{ char, style }` |
| `RenderNode` | `{ id, type, props, children[], style, layout }` |

## Notes

- This is the leaf of the dependency graph — it depends on nothing internal.
- `NodeId` is `string` on the TS side; the Rust engine uses `slotmap::DefaultKey` (8 bytes), transmuted to `u64` across FFI.
