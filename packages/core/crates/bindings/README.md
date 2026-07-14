# bettertui-bindings

## Purpose

napi-rs bindings that expose the Rust rendering engine (`bettertui-engine`) to Node.js/TypeScript. This is the FFI boundary between the Rust and TypeScript layers of BetterTUI.

## Responsibilities

- **Native addon:** Exposes Rust structs as Node.js classes via `#[napi_derive::napi]`.
- **JSON command bridge:** Deserializes JSON command batches from TypeScript into Rust `Command` enums, processes them, and returns JSON results.
- **Node.js classes:** `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiCapabilities`, `NapiKeymap`, `NapiWidgetHost`, plus theme value types (`NapiTheme`, `NapiThemeColors`, `NapiThemeSpacing`, `NapiThemeBorders`).
- **Global functions:** `getVersion()`, `detectCapabilities()`, `createDarkTheme()`, `createLightTheme()`, `createDefaultTheme()`.

## Public API

### Classes

```rust
#[napi]
struct NapiEngine {
  new(width: u32, height: u32),
  process_commands(commands_json: String) -> String,
  begin_frame(),
  commit_frame(),
  render() -> String,
  resize(width: u32, height: u32),
  node_count() -> u32,
  tree_summary() -> String,
  print_tree() -> String,
  validate() -> String,
}

#[napi]
struct NapiEventBus {
  push_key(key: String, ctrl: bool, shift: bool, alt: bool, target_id: u32),
  push_mouse(button: String, x: i32, y: i32, target_id: u32),
  drain() -> String,
  len() -> u32,
  is_empty() -> bool,
  clear(),
}

#[napi]
struct NapiFocusManager {
  focus(node_id: u32) -> bool,
  blur_current() -> bool,
  focused() -> u32,
  traverse(direction: String) -> u32,
}

#[napi]
struct NapiTextEngine {
  new(text?: String),
  insert_char(ch: String),
  insert_str(text: String),
  insert_text(text: String),
  delete_char_backward(),
  delete_char_forward(),
  cursor_left(),
  cursor_right(),
  text() -> String,
  undo() -> bool,
  redo() -> bool,
}

#[napi]
struct NapiScheduler {
  begin_frame() -> bool,
  end_frame(),
  request_frame(),
  fps() -> String,
  is_idle() -> bool,
}

#[napi]
struct NapiCapabilities {
  // Factory: NapiCapabilities.detect()
  brand: String,
  true_color: bool,
  kitty_keyboard: bool,
  bracketed_paste: bool,
  mouse_support: bool,
  osc52_clipboard: bool,
  osc8: bool,
  synchronized_output: bool,
  underline_color: bool,
  strikethrough: bool,
  cursor_style: bool,
  alternate_scroll: bool,
  kitty_graphics: bool,
  sixel: bool,
  iterm_images: bool,
  focus_events: bool,
  csi_u: bool,
  term_width: u32,
  term_height: u32,
  pixel_width: u32,
  pixel_height: u32,
  has_pixel_size: bool,
}

#[napi]
struct NapiKeymap {
  new(),
  add_binding(layer: String, id: String, keys: String, command: String, description?: String, priority: i32) -> bool,
  set_mode(mode: String),
  current_mode() -> Option<String>,
  clear_mode(),
  remove_layer(name: String) -> bool,
  set_chord_timeout(ms: f64),
  chord_timeout() -> f64,
  handle_key(key_str: String) -> Option<String>,
  has_pending() -> bool,
  clear_pending(),
  pending_keys() -> Vec<String>,
}

#[napi]
struct NapiWidgetHost {
  new(),
  widget_count() -> u32,
  register_widget_type(kind: String) -> Result<()>, // Err: registration from JS unsupported
}

// Theme value types
#[napi]
struct NapiTheme { name: String, colors: NapiThemeColors, spacing: NapiThemeSpacing, borders: NapiThemeBorders }
#[napi]
struct NapiThemeColors { /* color fields */ }
#[napi]
struct NapiThemeSpacing { /* spacing fields */ }
#[napi]
struct NapiThemeBorders { /* border fields */ }
```

### Global functions

```rust
#[napi]
fn get_version() -> String;

#[napi]
fn detect_capabilities() -> String; // JSON-serialized TerminalCapabilities

#[napi]
fn create_dark_theme() -> NapiTheme;

#[napi]
fn create_light_theme() -> NapiTheme;

#[napi]
fn create_default_theme() -> NapiTheme; // dark
```

## Dependencies

- `bettertui-engine` — the Rust rendering engine
- `napi` — napi-rs runtime
- `napi-derive` — napi-rs derive macros
- `serde` / `serde_json` — JSON serialization
- `slotmap` — NodeId FFI conversion

## Consumers

- `@bettertui/core` (TypeScript) — loads this addon via `require("bettertui_bindings")` in its engine bridge at `src/platform/`

## Internal Structure

```
src/
  lib.rs   # All napi-rs bindings (2,756 lines)
build.rs   # napi_build::setup()
```

## Design Principles

- **Single FFI boundary.** All Rust ↔ TypeScript communication goes through this crate. No direct engine access from TypeScript.
- **JSON command protocol.** Commands are serialized as JSON at the TypeScript boundary. This keeps the FFI simple and debuggable.
- **NodeId transmutation.** `slotmap::DefaultKey` (8 bytes) is transmuted to/from `u64` for FFI. This is safe within the current slotmap implementation but should be documented.

## Build

```bash
# Build the native addon
cargo build -p bettertui-bindings

# Or via the workspace
pnpm build  # Triggers tsdown which triggers napi-rs build
```

## Notes

- This crate uses `crate-type = ["cdylib"]` for Node.js addon loading.
- The `build.rs` file must call `napi_build::setup()` for napi-rs to work.
- The `CommandJson` enum with 69 variants mirrors the Rust `Command` enum for JSON deserialization.
- No `package.json` is needed here — this crate is part of `packages/core/crates/bindings/` and is loaded by the napi-rs build system. The TypeScript bridge lives at `packages/core/src/platform/`.
