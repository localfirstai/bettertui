// Re-export all types from shared (internal package, not for direct public consumption)
export type {
  AlignItems,
  AlignSelf,
  BorderStyle,
  BorderStyleKind,
  ColorValue,
  FlexDirection,
  Gap,
  Inset,
  JustifyContent,
  KeyEvent,
  LayoutConstraints,
  Margin,
  MouseButton,
  MouseEvent,
  Overflow,
  Padding,
  Point,
  Position,
  Rect,
  Size,
  Sizing,
  Style,
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
} from "./command";
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
} from "./command";

// Framework-agnostic reconciler (wraps tree ops with command emission)
export { createReconciler } from "./reconciler";

// Framework-agnostic runtime
export { Runtime } from "./runtime";

// Keymap
export { Keymap } from "./lib/keybinding";
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
} from "./lib/keybinding";

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
} from "./lib/validation";
export type { ValidationError, ValidationResult } from "./lib/validation";

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
} from "./platform/types";
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
} from "./platform";
export type { HighlightSegment } from "./platform";
export { createRuntime } from "./platform/runtime";
export type {
  Runtime as NativeRuntime,
  RuntimeOptions as NativeRuntimeOptions,
} from "./platform/runtime";
export { createEventLoop } from "./platform/events";
export type { EventLoop, EventCallback } from "./platform/events";
