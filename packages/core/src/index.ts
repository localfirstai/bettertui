// Re-export used types from shared
export type {
  Point,
  Rect,
  LayoutConstraints,
  KeyEvent,
  MouseButton,
  MouseEvent,
  ColorValue,
  Style,
  BorderStyle,
  Theme,
  ThemeColors,
  ThemeSpacing,
} from "@bettertui/shared";

// Framework-agnostic command protocol and tree manipulation
export type {
  HostContext,
  Instance,
  TextInstance,
  HostConfig,
  Command,
  CommandBufferConsumer,
} from "./command-buffer";
export {
  CommandBuffer,
  generateId,
  createInstance,
  createTextInstance,
  appendChild,
  removeChild,
  insertBefore,
  prepareUpdate,
  commitUpdate,
  commitTextUpdate,
  finalizeInitialChildren,
  resetAfterCommit,
} from "./command-buffer";

// Framework-agnostic reconciler (wraps tree ops with command emission)
export { createReconciler } from "./reconciler";

// Framework-agnostic runtime
export { Runtime } from "./runtime";

// Keymap
export { Keymap } from "./keymap";
export type {
  KeymapEvent,
  CommandHandler,
  CommandContext,
  CommandEntry,
  InterceptHandler,
  InterceptContext,
  KeyListener,
  KeymapOptions,
  ActiveKeyInfo,
} from "./keymap";

// Testing utilities
export { createTestKeymap, createMockNativeKeymap } from "./testing";
export type { TestBinding } from "./testing";

// Validation utilities
export {
  isValidColor,
  validateLayoutConstraints,
  validateStyle,
  validate,
  warnIfInvalid,
} from "./validation";
export type { ValidationError, ValidationResult } from "./validation";

// Engine (Rust napi-rs bindings)
export type {
  BindingInfo,
  NapiEngine,
  NapiEventBus,
  NapiFocusManager,
  NapiKeymap,
  NapiTextEngine,
  NapiScheduler,
  ProcessResult,
  TerminalCapabilities,
  SchedulerStats,
} from "./engine/types";
export {
  createEngine,
  createEventBus,
  createFocusManager,
  createKeymap,
  createTextEngine,
  createScheduler,
  detectCapabilities,
  getVersion,
  highlightCode,
} from "./engine";
export type { HighlightSegment } from "./engine";
export { createRuntime } from "./engine/runtime";
export type {
  Runtime as NativeRuntime,
  RuntimeOptions as NativeRuntimeOptions,
} from "./engine/runtime";
export { createEventLoop } from "./engine/events";
export type { EventLoop, EventCallback } from "./engine/events";
