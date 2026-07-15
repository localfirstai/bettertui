// Re-export all hooks and providers
export {
  Provider,
  useTheme,
  FocusProvider,
  useFocus,
  useKeyboard,
  TerminalProvider,
  useTerminal,
  useResize,
  useFrame,
  useClipboard,
  useAnimation,
  useMouse,
  SelectionProvider,
  useSelection,
  CapabilitiesProvider,
  useCapabilities,
  useTimeline,
  easings,
  KeymapProvider,
  useKeymap,
  useKeymapEvent,
  useActiveBindings,
  usePendingSequence,
  useCommand,
  useKeyIntercept,
  useKeymapMode,
  Keymap,
} from "./hooks";

export type {
  Theme,
  ThemeColors,
  ThemeSpacing,
  ProviderProps,
  MouseState,
  EasingFunction,
  UseAnimationOptions,
  TimelineAnimation,
  Timeline,
  KeymapEvent,
  KeymapOptions,
  CommandHandler,
  BindingInfo,
} from "./hooks";

// Runtime (render + RuntimeProvider + useRuntime)
export { render, RuntimeProvider, useRuntime } from "./runtime";
export type { RenderHandle } from "./runtime";

// Re-export core types that users need
export type {
  Command,
  CommandBuffer,
  Instance,
  CommandRuntime,
  KeyEvent,
  MouseEvent,
  MouseButton,
  Style,
  ColorValue,
  BorderStyle,
  Rect,
  Point,
} from "@bettertui/core";

// Re-export widget option types (owned by @bettertui/shared)
export type {
  BoxOptions,
  TextOptions,
  CodeOptions,
  InputOptions,
  TextareaOptions,
  SelectOptions,
  SliderOptions,
  TabSelectOptions,
  MarkdownOptions,
  DiffOptions,
  TextTableOptions,
  ScrollBarOptions,
  ScrollBoxOptions,
} from "@bettertui/shared";

// Re-export layout types (re-exported from @bettertui/core which owns shared re-exports)
export type {
  FlexDirection as SharedFlexDirection,
  JustifyContent as SharedJustifyContent,
  AlignItems as SharedAlignItems,
  AlignSelf as SharedAlignSelf,
  Position as SharedPosition,
  Sizing as SharedSizing,
  Overflow as SharedOverflow,
  Padding as SharedPadding,
  Margin as SharedMargin,
  Inset as SharedInset,
  Gap as SharedGap,
  LayoutConstraints,
} from "@bettertui/core";

// Re-export all components
export {
  Box,
  Text,
  Code,
  Input,
  Textarea,
  Select,
  Slider,
  TabSelect,
  ScrollBar,
  ScrollBox,
  Markdown,
  Diff,
  TextTable,
} from "./components";

export type {
  FlexDirection,
  JustifyContent,
  AlignItems,
  BoxProps,
  TextProps,
  CodeProps,
  InputProps,
  TextareaProps,
  SliderProps,
  SelectProps,
  TabSelectProps,
  ScrollBarProps,
  ScrollBoxProps,
  MarkdownProps,
  DiffProps,
  TextTableProps,
} from "./components";
export { renderToStringAsync, type TestRendererOptions } from "./testing";
