import { getSuffix, loadLibrary } from "./ffi";

export type NativeHandle = number;

export interface NativeEngineLib {
  ffi_engine_create: (width: number, height: number) => NativeHandle;
  ffi_engine_destroy: (handle: NativeHandle) => void;
  ffi_engine_process_commands: (handle: NativeHandle, jsonPtr: bigint, jsonLen: number) => string;
  ffi_engine_begin_frame: (handle: NativeHandle) => void;
  ffi_engine_commit_frame: (handle: NativeHandle) => void;
  ffi_engine_render: (handle: NativeHandle) => string;
  ffi_engine_render_full: (handle: NativeHandle) => string;
  ffi_engine_node_count: (handle: NativeHandle) => number;
  ffi_engine_frame_count: (handle: NativeHandle) => bigint;
  ffi_engine_create_node: (handle: NativeHandle, kindPtr: bigint, kindLen: number) => bigint;
  ffi_engine_append_child: (handle: NativeHandle, parent: bigint, child: bigint) => number;
  ffi_engine_remove_node: (handle: NativeHandle, id: bigint) => void;
  ffi_engine_set_text: (handle: NativeHandle, id: bigint, textPtr: bigint, textLen: number) => void;
  ffi_engine_resize: (handle: NativeHandle, width: number, height: number) => void;
  ffi_engine_shutdown: (handle: NativeHandle) => void;
  ffi_engine_print_tree: (handle: NativeHandle) => string;
  ffi_engine_root: (handle: NativeHandle) => bigint;
  ffi_engine_validate: (handle: NativeHandle) => number;
  ffi_engine_generation: (handle: NativeHandle) => bigint;
  ffi_engine_dimensions: (handle: NativeHandle, width: bigint, height: bigint) => number;
  ffi_engine_request_frame: (handle: NativeHandle) => void;
  ffi_engine_tree_summary: (handle: NativeHandle) => string;
  ffi_engine_should_render: (handle: NativeHandle) => string;

  ffi_event_bus_create: () => NativeHandle;
  ffi_event_bus_destroy: (handle: NativeHandle) => void;
  ffi_event_bus_push_key: (
    handle: NativeHandle,
    keyPtr: bigint,
    keyLen: number,
    ctrl: number,
    shift: number,
    alt: number,
    target: bigint,
  ) => void;
  ffi_event_bus_push_mouse: (
    handle: NativeHandle,
    buttonPtr: bigint,
    buttonLen: number,
    x: number,
    y: number,
    target: bigint,
  ) => void;
  ffi_event_bus_push_mouse_motion: (
    handle: NativeHandle,
    x: number,
    y: number,
    target: bigint,
  ) => void;
  ffi_event_bus_push_resize: (
    handle: NativeHandle,
    width: number,
    height: number,
    prevWidth: number,
    prevHeight: number,
  ) => void;
  ffi_event_bus_drain: (handle: NativeHandle) => string;
  ffi_event_bus_len: (handle: NativeHandle) => number;
  ffi_event_bus_push_paste: (
    handle: NativeHandle,
    textPtr: bigint,
    textLen: number,
    target: bigint,
  ) => void;
  ffi_event_bus_is_empty: (handle: NativeHandle) => number;
  ffi_event_bus_clear: (handle: NativeHandle) => void;

  ffi_focus_manager_create: () => NativeHandle;
  ffi_focus_manager_destroy: (handle: NativeHandle) => void;
  ffi_focus_manager_focus: (handle: NativeHandle, id: bigint) => number;
  ffi_focus_manager_blur: (handle: NativeHandle, id: bigint) => number;
  ffi_focus_manager_blur_current: (handle: NativeHandle) => number;
  ffi_focus_manager_focused: (handle: NativeHandle) => bigint;
  ffi_focus_manager_is_focused: (handle: NativeHandle, id: bigint) => number;
  ffi_focus_manager_traverse: (handle: NativeHandle, dirPtr: bigint, dirLen: number) => bigint;
  ffi_focus_manager_focus_order: (handle: NativeHandle) => string;
  ffi_focus_manager_set_scope: (handle: NativeHandle, scopeId: bigint) => void;
  ffi_focus_manager_clear_scope: (handle: NativeHandle) => void;
  ffi_focus_manager_scope_id: (handle: NativeHandle) => bigint;
  ffi_focus_manager_focused_in_scope: (handle: NativeHandle) => bigint;

  ffi_text_engine_create: (textPtr: bigint, textLen: number) => NativeHandle;
  ffi_text_engine_destroy: (handle: NativeHandle) => void;
  ffi_text_engine_insert_char: (handle: NativeHandle, ch: number) => void;
  ffi_text_engine_insert_str: (handle: NativeHandle, textPtr: bigint, textLen: number) => void;
  ffi_text_engine_delete_char: (handle: NativeHandle) => void;
  ffi_text_engine_delete_char_forward: (handle: NativeHandle) => void;
  ffi_text_engine_cursor_left: (handle: NativeHandle) => void;
  ffi_text_engine_cursor_right: (handle: NativeHandle) => void;
  ffi_text_engine_length: (handle: NativeHandle) => number;
  ffi_text_engine_get_text: (handle: NativeHandle) => string;
  ffi_text_engine_cursor_position: (handle: NativeHandle) => number;
  ffi_text_engine_can_undo: (handle: NativeHandle) => number;
  ffi_text_engine_can_redo: (handle: NativeHandle) => number;
  ffi_text_engine_line_count: (handle: NativeHandle) => number;
  ffi_text_engine_is_empty: (handle: NativeHandle) => number;
  ffi_text_engine_clear: (handle: NativeHandle) => void;
  ffi_text_engine_undo: (handle: NativeHandle) => number;
  ffi_text_engine_redo: (handle: NativeHandle) => number;
  ffi_text_engine_set_cursor_position: (handle: NativeHandle, pos: number) => void;
  ffi_text_engine_insert_at: (
    handle: NativeHandle,
    pos: number,
    textPtr: bigint,
    textLen: number,
  ) => void;
  ffi_text_engine_delete_at: (handle: NativeHandle, pos: number, len: number) => number;
  ffi_text_engine_cursor_up: (handle: NativeHandle) => void;
  ffi_text_engine_cursor_down: (handle: NativeHandle) => void;
  ffi_text_engine_cursor_line_start: (handle: NativeHandle) => void;
  ffi_text_engine_cursor_line_end: (handle: NativeHandle) => void;
  ffi_text_engine_delete_word_backward: (handle: NativeHandle) => void;
  ffi_text_engine_delete_word_forward: (handle: NativeHandle) => void;

  ffi_scheduler_create: (fps: number) => NativeHandle;
  ffi_scheduler_destroy: (handle: NativeHandle) => void;
  ffi_scheduler_request_frame: (handle: NativeHandle) => void;
  ffi_scheduler_begin_frame: (handle: NativeHandle) => number;
  ffi_scheduler_end_frame: (handle: NativeHandle) => void;
  ffi_scheduler_should_render: (handle: NativeHandle) => string;
  ffi_scheduler_is_idle: (handle: NativeHandle) => number;
  ffi_scheduler_frame_count: (handle: NativeHandle) => bigint;
  ffi_scheduler_dropped_frames: (handle: NativeHandle) => bigint;
  ffi_scheduler_fps: (handle: NativeHandle) => number;
  ffi_scheduler_frame_budget_ms: (handle: NativeHandle) => number;

  ffi_keymap_create: () => NativeHandle;
  ffi_keymap_destroy: (handle: NativeHandle) => void;
  ffi_keymap_add_binding: (
    handle: NativeHandle,
    layerPtr: bigint,
    layerLen: number,
    idPtr: bigint,
    idLen: number,
    keysPtr: bigint,
    keysLen: number,
    cmdPtr: bigint,
    cmdLen: number,
    descPtr: bigint,
    descLen: number,
    priority: number,
  ) => number;
  ffi_keymap_set_mode: (handle: NativeHandle, modePtr: bigint, modeLen: number) => void;
  ffi_keymap_current_mode: (handle: NativeHandle) => string;
  ffi_keymap_handle_key: (handle: NativeHandle, keyPtr: bigint, keyLen: number) => string;
  ffi_keymap_has_pending: (handle: NativeHandle) => number;
  ffi_keymap_clear_pending: (handle: NativeHandle) => void;
  ffi_keymap_remove_layer: (handle: NativeHandle, namePtr: bigint, nameLen: number) => number;
  ffi_keymap_clear_mode: (handle: NativeHandle) => void;
  ffi_keymap_set_chord_timeout: (handle: NativeHandle, ms: bigint) => void;
  ffi_keymap_chord_timeout_ms: (handle: NativeHandle) => bigint;
  ffi_keymap_command_history: (handle: NativeHandle) => string;
  ffi_keymap_clear_history: (handle: NativeHandle) => void;
  ffi_keymap_active_bindings: (handle: NativeHandle) => string;
  ffi_keymap_parse_key: (handle: NativeHandle, keyPtr: bigint, keyLen: number) => string;
  ffi_keymap_parse_sequence: (handle: NativeHandle, seqPtr: bigint, seqLen: number) => string;

  ffi_get_version: () => string;
  ffi_detect_capabilities: () => string;
  ffi_highlight_code: (
    codePtr: bigint,
    codeLen: number,
    langPtr: bigint,
    langLen: number,
  ) => string;
  ffi_create_dark_theme: () => string;
  ffi_create_light_theme: () => string;
  ffi_create_default_theme: () => string;
  ffi_free_string: (ptr: string) => void;
  ffi_free_bytes: (ptr: bigint) => void;
}

export function getNativeLibPath(): string {
  const platform = process.platform;
  const arch = process.arch;
  const ext = getSuffix();

  const candidates: string[] = [];

  if (platform === "darwin") {
    const dirs = [
      process.cwd(),
      ...(process.env["BETTERTUI_LIB_PATH"] ? [process.env["BETTERTUI_LIB_PATH"]] : []),
    ];
    for (const dir of dirs) {
      candidates.push(`${dir}/target/release/libbettertui_engine${ext}`);
      candidates.push(`${dir}/target/debug/libbettertui_engine${ext}`);
    }
    if (arch === "arm64") {
      candidates.push(`@bettertui/core-darwin-arm64/libbettertui_engine${ext}`);
    }
    if (arch === "x64") {
      candidates.push(`@bettertui/core-darwin-x64/libbettertui_engine${ext}`);
    }
  }

  if (platform === "linux") {
    const dirs = [
      process.cwd(),
      ...(process.env["BETTERTUI_LIB_PATH"] ? [process.env["BETTERTUI_LIB_PATH"]] : []),
    ];
    for (const dir of dirs) {
      candidates.push(`${dir}/target/release/libbettertui_engine${ext}`);
      candidates.push(`${dir}/target/debug/libbettertui_engine${ext}`);
    }
    const libc = process.env["BETTERTUI_LIBC"] ?? "gnu";
    if (arch === "arm64") {
      candidates.push(`@bettertui/core-linux-arm64-${libc}/libbettertui_engine${ext}`);
    }
    if (arch === "x64") {
      candidates.push(`@bettertui/core-linux-x64-${libc}/libbettertui_engine${ext}`);
    }
  }

  if (platform === "win32") {
    const dirs = [
      process.cwd(),
      ...(process.env["BETTERTUI_LIB_PATH"] ? [process.env["BETTERTUI_LIB_PATH"]] : []),
    ];
    for (const dir of dirs) {
      candidates.push(`${dir}/target/release/bettertui_engine${ext}`);
      candidates.push(`${dir}/target/debug/bettertui_engine${ext}`);
    }
    if (arch === "arm64") {
      candidates.push(`@bettertui/core-win32-arm64/bettertui_engine${ext}`);
    }
    if (arch === "x64") {
      candidates.push(`@bettertui/core-win32-x64/bettertui_engine${ext}`);
    }
  }

  const fs = require("node:fs");
  for (const candidate of candidates) {
    try {
      if (fs.existsSync(candidate)) return candidate;
    } catch {}
  }

  // try resolving from optional platform package
  const platformKey =
    platform === "linux"
      ? `${platform}-${arch}-${process.env["BETTERTUI_LIBC"] ?? "gnu"}`
      : `${platform}-${arch}`;
  const pkgName = `@bettertui/core-${platformKey}`;
  try {
    const pkgPath = require.resolve(pkgName);
    return pkgPath;
  } catch {}

  throw new Error(
    `Cannot find native library for ${platform}-${arch}. Build it first: cargo build -p bettertui-engine --release`,
  );
}

let nativeLib: NativeEngineLib | null = null;

export function getNativeLib(): NativeEngineLib {
  if (nativeLib) return nativeLib;
  const libPath = getNativeLibPath();
  nativeLib = loadLibrary<NativeEngineLib>(libPath, getSymbolDefinitions());
  return nativeLib;
}

function getSymbolDefinitions() {
  return {
    ffi_engine_create: { arguments: ["u32", "u32"], returns: "u64" },
    ffi_engine_destroy: { arguments: ["u64"], returns: "void" },
    ffi_engine_process_commands: {
      arguments: ["u64", "pointer", "u32"],
      returns: "string",
    },
    ffi_engine_begin_frame: { arguments: ["u64"], returns: "void" },
    ffi_engine_commit_frame: { arguments: ["u64"], returns: "void" },
    ffi_engine_render: { arguments: ["u64"], returns: "string" },
    ffi_engine_render_full: { arguments: ["u64"], returns: "string" },
    ffi_engine_node_count: { arguments: ["u64"], returns: "u32" },
    ffi_engine_frame_count: { arguments: ["u64"], returns: "u64" },
    ffi_engine_create_node: {
      arguments: ["u64", "pointer", "u32"],
      returns: "u64",
    },
    ffi_engine_append_child: {
      arguments: ["u64", "u64", "u64"],
      returns: "i32",
    },
    ffi_engine_remove_node: { arguments: ["u64", "u64"], returns: "void" },
    ffi_engine_set_text: {
      arguments: ["u64", "u64", "pointer", "u32"],
      returns: "void",
    },
    ffi_engine_resize: { arguments: ["u64", "u32", "u32"], returns: "void" },
    ffi_engine_shutdown: { arguments: ["u64"], returns: "void" },
    ffi_engine_print_tree: { arguments: ["u64"], returns: "string" },
    ffi_engine_root: { arguments: ["u64"], returns: "u64" },
    ffi_engine_validate: { arguments: ["u64"], returns: "i32" },
    ffi_engine_generation: { arguments: ["u64"], returns: "u64" },
    ffi_engine_dimensions: {
      arguments: ["u64", "pointer", "pointer"],
      returns: "i32",
    },
    ffi_engine_request_frame: { arguments: ["u64"], returns: "void" },
    ffi_engine_tree_summary: { arguments: ["u64"], returns: "string" },
    ffi_engine_should_render: { arguments: ["u64"], returns: "string" },

    ffi_event_bus_create: { arguments: [], returns: "u64" },
    ffi_event_bus_destroy: { arguments: ["u64"], returns: "void" },
    ffi_event_bus_push_key: {
      arguments: ["u64", "pointer", "u32", "i32", "i32", "i32", "u64"],
      returns: "void",
    },
    ffi_event_bus_push_mouse: {
      arguments: ["u64", "pointer", "u32", "u32", "u32", "u64"],
      returns: "void",
    },
    ffi_event_bus_push_mouse_motion: {
      arguments: ["u64", "u32", "u32", "u64"],
      returns: "void",
    },
    ffi_event_bus_push_resize: {
      arguments: ["u64", "u32", "u32", "u32", "u32"],
      returns: "void",
    },
    ffi_event_bus_drain: { arguments: ["u64"], returns: "string" },
    ffi_event_bus_len: { arguments: ["u64"], returns: "u32" },
    ffi_event_bus_push_paste: {
      arguments: ["u64", "pointer", "u32", "u64"],
      returns: "void",
    },
    ffi_event_bus_is_empty: { arguments: ["u64"], returns: "i32" },
    ffi_event_bus_clear: { arguments: ["u64"], returns: "void" },

    ffi_focus_manager_create: { arguments: [], returns: "u64" },
    ffi_focus_manager_destroy: { arguments: ["u64"], returns: "void" },
    ffi_focus_manager_focus: { arguments: ["u64", "u64"], returns: "i32" },
    ffi_focus_manager_blur: { arguments: ["u64", "u64"], returns: "i32" },
    ffi_focus_manager_blur_current: { arguments: ["u64"], returns: "i32" },
    ffi_focus_manager_focused: { arguments: ["u64"], returns: "u64" },
    ffi_focus_manager_is_focused: {
      arguments: ["u64", "u64"],
      returns: "i32",
    },
    ffi_focus_manager_traverse: {
      arguments: ["u64", "pointer", "u32"],
      returns: "u64",
    },
    ffi_focus_manager_focus_order: { arguments: ["u64"], returns: "string" },
    ffi_focus_manager_set_scope: { arguments: ["u64", "u64"], returns: "void" },
    ffi_focus_manager_clear_scope: { arguments: ["u64"], returns: "void" },
    ffi_focus_manager_scope_id: { arguments: ["u64"], returns: "u64" },
    ffi_focus_manager_focused_in_scope: {
      arguments: ["u64"],
      returns: "u64",
    },

    ffi_text_engine_create: {
      arguments: ["pointer", "u32"],
      returns: "u64",
    },
    ffi_text_engine_destroy: { arguments: ["u64"], returns: "void" },
    ffi_text_engine_insert_char: { arguments: ["u64", "u32"], returns: "void" },
    ffi_text_engine_insert_str: {
      arguments: ["u64", "pointer", "u32"],
      returns: "void",
    },
    ffi_text_engine_delete_char: { arguments: ["u64"], returns: "void" },
    ffi_text_engine_delete_char_forward: {
      arguments: ["u64"],
      returns: "void",
    },
    ffi_text_engine_cursor_left: { arguments: ["u64"], returns: "void" },
    ffi_text_engine_cursor_right: { arguments: ["u64"], returns: "void" },
    ffi_text_engine_length: { arguments: ["u64"], returns: "u32" },
    ffi_text_engine_get_text: { arguments: ["u64"], returns: "string" },
    ffi_text_engine_cursor_position: { arguments: ["u64"], returns: "u32" },
    ffi_text_engine_can_undo: { arguments: ["u64"], returns: "i32" },
    ffi_text_engine_can_redo: { arguments: ["u64"], returns: "i32" },
    ffi_text_engine_line_count: { arguments: ["u64"], returns: "u32" },
    ffi_text_engine_is_empty: { arguments: ["u64"], returns: "i32" },
    ffi_text_engine_clear: { arguments: ["u64"], returns: "void" },
    ffi_text_engine_undo: { arguments: ["u64"], returns: "i32" },
    ffi_text_engine_redo: { arguments: ["u64"], returns: "i32" },
    ffi_text_engine_set_cursor_position: {
      arguments: ["u64", "u32"],
      returns: "void",
    },
    ffi_text_engine_insert_at: {
      arguments: ["u64", "u32", "pointer", "u32"],
      returns: "void",
    },
    ffi_text_engine_delete_at: {
      arguments: ["u64", "u32", "u32"],
      returns: "i32",
    },
    ffi_text_engine_cursor_up: { arguments: ["u64"], returns: "void" },
    ffi_text_engine_cursor_down: { arguments: ["u64"], returns: "void" },
    ffi_text_engine_cursor_line_start: { arguments: ["u64"], returns: "void" },
    ffi_text_engine_cursor_line_end: { arguments: ["u64"], returns: "void" },
    ffi_text_engine_delete_word_backward: {
      arguments: ["u64"],
      returns: "void",
    },
    ffi_text_engine_delete_word_forward: {
      arguments: ["u64"],
      returns: "void",
    },

    ffi_scheduler_create: { arguments: ["u32"], returns: "u64" },
    ffi_scheduler_destroy: { arguments: ["u64"], returns: "void" },
    ffi_scheduler_request_frame: { arguments: ["u64"], returns: "void" },
    ffi_scheduler_begin_frame: { arguments: ["u64"], returns: "i32" },
    ffi_scheduler_end_frame: { arguments: ["u64"], returns: "void" },
    ffi_scheduler_should_render: { arguments: ["u64"], returns: "string" },
    ffi_scheduler_is_idle: { arguments: ["u64"], returns: "i32" },
    ffi_scheduler_frame_count: { arguments: ["u64"], returns: "u64" },
    ffi_scheduler_dropped_frames: { arguments: ["u64"], returns: "u64" },
    ffi_scheduler_fps: { arguments: ["u64"], returns: "f64" },
    ffi_scheduler_frame_budget_ms: { arguments: ["u64"], returns: "f64" },

    ffi_keymap_create: { arguments: [], returns: "u64" },
    ffi_keymap_destroy: { arguments: ["u64"], returns: "void" },
    ffi_keymap_add_binding: {
      arguments: [
        "u64",
        "pointer",
        "u32",
        "pointer",
        "u32",
        "pointer",
        "u32",
        "pointer",
        "u32",
        "pointer",
        "u32",
        "i32",
      ],
      returns: "i32",
    },
    ffi_keymap_set_mode: {
      arguments: ["u64", "pointer", "u32"],
      returns: "void",
    },
    ffi_keymap_current_mode: { arguments: ["u64"], returns: "string" },
    ffi_keymap_handle_key: {
      arguments: ["u64", "pointer", "u32"],
      returns: "string",
    },
    ffi_keymap_has_pending: { arguments: ["u64"], returns: "i32" },
    ffi_keymap_clear_pending: { arguments: ["u64"], returns: "void" },
    ffi_keymap_remove_layer: {
      arguments: ["u64", "pointer", "u32"],
      returns: "i32",
    },
    ffi_keymap_clear_mode: { arguments: ["u64"], returns: "void" },
    ffi_keymap_set_chord_timeout: { arguments: ["u64", "u64"], returns: "void" },
    ffi_keymap_chord_timeout_ms: { arguments: ["u64"], returns: "u64" },
    ffi_keymap_command_history: { arguments: ["u64"], returns: "string" },
    ffi_keymap_clear_history: { arguments: ["u64"], returns: "void" },
    ffi_keymap_active_bindings: { arguments: ["u64"], returns: "string" },
    ffi_keymap_parse_key: {
      arguments: ["u64", "pointer", "u32"],
      returns: "string",
    },
    ffi_keymap_parse_sequence: {
      arguments: ["u64", "pointer", "u32"],
      returns: "string",
    },

    ffi_get_version: { arguments: [], returns: "string" },
    ffi_detect_capabilities: { arguments: [], returns: "string" },
    ffi_highlight_code: {
      arguments: ["pointer", "u32", "pointer", "u32"],
      returns: "string",
    },
    ffi_create_dark_theme: { arguments: [], returns: "string" },
    ffi_create_light_theme: { arguments: [], returns: "string" },
    ffi_create_default_theme: { arguments: [], returns: "string" },
    ffi_free_string: { arguments: ["pointer"], returns: "void" },
    ffi_free_bytes: { arguments: ["pointer"], returns: "void" },
  };
}
