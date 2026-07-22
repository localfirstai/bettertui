// Curated re-export of shared types
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
  KeyEventSource,
  KeyEventType,
  LayoutConstraints,
  Margin,
  MouseButton,
  MouseEvent,
  Overflow,
  Padding,
  Position,
  Sizing,
  Style,
  Theme,
  ThemeColors,
  ThemeSpacing,
} from "@bettertui/shared";

// Geometry types (core-only: not needed by framework adapters)
export type { Point, Rect, Size } from "./geometry.types";

// Command protocol, buffer, and tree operations
export * from "./command";

// Reconciler (wraps tree ops with command emission)
export { createReconciler } from "./reconciler";

// Command runtime (frame loop over CommandBuffer)
export { CommandRuntime } from "./runtime";
export type { CommandRuntimeOptions } from "./runtime";

export { Renderable } from "./renderable";
export type { WidgetContext, WidgetLifecycle, ImperativeContext } from "./renderable";

// Keymap, clock, and validation utilities
export * from "./lib";

// Platform (native engine bridge, events, runtime)
export * from "./platform";

// Testing utilities (explicit re-exports to avoid conflicts)
export {
  createTestRenderer,
  createTestRendererSync,
  createMockKeys,
  KeyCodes,
  createMockMouse,
  MouseButtons,
  createTestStdin,
  createTestStdout,
  TestReadStream,
  TestWriteStream,
  createSpy,
  createTerminalCapabilities,
  createMinimalTerminalCapabilities,
  createFullTerminalCapabilities,
  createKittyTerminalCapabilities,
  createITerm2TerminalCapabilities,
  createMockNativeKeymap,
  createTestKeymap,
} from "./testing";
export type {
  TestRendererOptions,
  TestRenderer,
  MockInput,
  MockMouse,
  TestRendererSetup,
  TestKeyInput,
  MockKeysOptions,
  KeyModifiers,
  MousePosition,
  MouseModifiers,
  MouseEventType,
  MouseEventOptions,
  TestStdin,
  TestStdout,
  Spy,
  TerminalCapabilitiesOptions,
  TestBinding,
} from "./testing";

// Framework-agnostic widget option types
export * from "./widgets";

// Animation utilities: easing, Tween, Spring, lerp helpers
export * from "./animations";

// Terminal graphics utilities: PixelBuffer, Canvas, color helpers
export * from "./graphics";

// In-core debug tooling (moved from the retired @bettertui/devtools package).
// `Logger`, `LogLevel`, and `TerminalCapabilities` already ship from
// `./platform`; the devtools facade re-exports its own variants under
// `DevTools*` aliases (see devtools/index.ts) to avoid the name collision.
export * from "./devtools";
