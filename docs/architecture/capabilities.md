# Capabilities

Capability detection identifies what a terminal supports so the renderer and input system only use features the terminal understands. Code: `packages/core/crates/engine/src/terminal/capabilities.rs`.

## Detection

- `CapabilityDetector::detect()` plus `update_from_queries(&[QueryResult])` (queries from `terminal/query.rs`: DA1/DA2/DA3/DSR/DECID/XTVersion/Kitty).
- `global_capabilities() -> &'static CapabilityDetector` — `OnceLock` singleton.
- `TerminalBrand` (Kitty, Ghostty, WezTerm, Alacritty, Foot, ITerm2, Tmux, WindowsTerminal, VSCodeTerminal, Unknown) via `brand_from_da2_model`.

## Feature matrix

`FeatureMatrix` tracks:

| Group | Fields |
|-------|--------|
| Render | `trueColor`, `underlineColor`, `strikethrough`, `cursorStyle`, `synchronizedOutput` |
| Input | `kittyKeyboard`, `csi_u`, `bracketedPaste`, `focusEvents`, `mouse` |
| Graphics | `osc52`, `osc8`, `kittyGraphics`, `sixel`, `itermImages`, `alternateScroll` |

`UnicodeCapabilities` covers `EmojiWidth` and `UnicodeVersion`. Clipboard capability specifics in `capabilities/clipboard.rs`.

## TypeScript surface

`@bettertui/core` exposes `detectCapabilities() -> TerminalCapabilities` and `NapiCapabilities` (17 fields).
