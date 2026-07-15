// Curated re-export of shared types (KeyEvent/MouseEvent come via ./platform)
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

// Keymap and validation utilities
export * from "./lib";

// Testing utilities
export * from "./testing";

// Platform (native engine bridge, events, runtime)
export * from "./platform";

// Framework-agnostic widget option types
export * from "./widgets";
