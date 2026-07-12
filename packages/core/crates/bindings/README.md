# bettertui-bindings

## Purpose

napi-rs bindings that expose the Rust rendering engine (`bettertui-engine`) to Node.js/TypeScript. This is the FFI boundary between the Rust and TypeScript layers of BetterTUI.

## Responsibilities

- **Native addon:** Exposes Rust structs as Node.js classes via `#[napi_derive::napi]`.
- **JSON command bridge:** Deserializes JSON command batches from TypeScript into Rust `Command` enums, processes them, and returns JSON results.
- **Node.js classes:** `NapiEngine`, `NapiEventBus`, `NapiFocusManager`, `NapiTextEngine`, `NapiScheduler`, `NapiCapabilities`.
- **Global functions:** `getVersion()`, `detectCapabilities()`.

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
  insert_text(text: String),
  delete_char_backward(),
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
```

### Global functions

```rust
#[napi]
fn get_version() -> String;

#[napi]
fn detect_capabilities() -> String; // JSON-serialized TerminalCapabilities
```

## Dependencies

- `bettertui-engine` — the Rust rendering engine
- `napi` — napi-rs runtime
- `napi-derive` — napi-rs derive macros
- `serde` / `serde_json` — JSON serialization
- `slotmap` — NodeId FFI conversion

## Consumers

- `@bettertui/core` (TypeScript) — loads this addon via `require("bettertui_bindings")` in its native bridge at `src/native/`

## Internal Structure

```
src/
  lib.rs   # All napi-rs bindings (1,976 lines)
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
pnpm build  # Triggers tsup which triggers napi-rs build
```

## Notes

- This crate uses `crate-type = ["cdylib"]` for Node.js addon loading.
- The `build.rs` file must call `napi_build::setup()` for napi-rs to work.
- The `CommandJson` enum with 60+ variants mirrors the Rust `Command` enum for JSON deserialization.
- No `package.json` is needed here — this crate is part of `packages/core/crates/bindings/` and is loaded by the napi-rs build system. The TypeScript bridge lives at `packages/core/src/native/`.
