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
  LayoutConstraints,
  Margin,
  MouseButton,
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
