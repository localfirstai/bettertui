export { Keymap } from "./keybinding";
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
  BindingInfo,
} from "./keybinding";

export { SystemClock } from "./clock";
export type { Clock, TimerHandle } from "./clock";

export {
  isValidColor,
  validateLayoutConstraints,
  validateStyle,
  validate,
  warnIfInvalid,
} from "./validation";
export type { ValidationError, ValidationResult } from "./validation";

export {
  parseKeypress,
  nonAlphanumericKeys,
  terminalNamedSingleStrokeKeys,
} from "./parseKeypress";
export type {
  ParsedKey,
  KeyEventType,
  ParseKeypressOptions,
} from "./parseKeypress";

export {
  parseKittyKeyboard,
  kittyNamedSingleStrokeKeys,
} from "./parseKeypressKitty";

export { MouseParser } from "./parseMouse";
export type { MouseEventType, RawMouseEvent, ScrollInfo } from "./parseMouse";

export { KeyHandler, InternalKeyHandler, PasteEvent } from "./KeyHandler";
export { KeyEvent as KeyboardEvent } from "./KeyHandler";
export type { KeyHandlerEventMap } from "./KeyHandler";

export { StdinParser } from "./stdinParser";
export type {
  StdinEvent,
  StdinParserOptions,
  StdinParserProtocolContext,
  StdinResponseProtocol,
  PasteMetadata,
} from "./stdinParser";

// ── New exports ───────────────────────────────────────────────────────────────

export { RGBA, parseColor, rgbaToEngineColor } from "./rgba";
export type { ColorInput } from "./rgba";

export {
  TextAttributes,
  StyledText,
  isStyledText,
  stringToStyledText,
  styledTextToAnsi,
  visibleWidth,
  t,
  // Style attribute helpers
  bold,
  italic,
  underline,
  strikethrough,
  dim,
  reverse,
  blink,
  // Named fg colors
  black,
  red,
  green,
  yellow,
  blue,
  magenta,
  cyan,
  white,
  brightBlack,
  brightRed,
  brightGreen,
  brightYellow,
  brightBlue,
  brightMagenta,
  brightCyan,
  brightWhite,
  // Named bg colors
  bgBlack,
  bgRed,
  bgGreen,
  bgBlue,
  bgYellow,
  bgCyan,
  bgMagenta,
  bgWhite,
  // Curried helpers
  fg,
  bg,
  link,
} from "./styledText";
export type { TextChunk, StylableInput } from "./styledText";

export {
  CliRenderEvents,
  RenderableEvents,
  InputRenderableEvents,
  SelectRenderableEvents,
  TabSelectRenderableEvents,
  SliderRenderableEvents,
  LayoutEvents,
} from "./renderableEvents";

export { env, registerEnvVar, getEnvVarConfig, getAllEnvVarConfigs } from "./env";
export type { EnvVarConfig } from "./env";

export { Timeline, createTimeline } from "./timeline";
export type { TimelineOptions, TweenConfig } from "./timeline";

export {
  h,
  instantiate,
  delegate,
  maybeMakeRenderable,
  // VNode factories (prefixed to avoid clash with widget classes)
  Box as VNodeBox,
  Text as VNodeText,
  Input as VNodeInput,
  Select as VNodeSelect,
  TabSelect as VNodeTabSelect,
  Code as VNodeCode,
  Generic,
  ScrollBox as VNodeScrollBox,
  ASCIIFont as VNodeASCIIFont,
  vstyles,
} from "./vnode";
export type { VNode } from "./vnode";
