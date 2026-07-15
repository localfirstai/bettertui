export { createRuntime } from "./runtime";
export type { Runtime, RuntimeOptions } from "./runtime";

export { createEventLoop } from "./events";
export type { EventLoop, EventCallback, KeyEvent, MouseEvent } from "./events";

export {
  createEngine,
  createEventBus,
  createFocusManager,
  createTextEngine,
  createScheduler,
  detectCapabilities,
  getVersion,
  createDefaultTheme,
  createDarkTheme,
  createLightTheme,
  createWidgetHost,
  createKeymap,
  highlightCode,
  getNativePackageName,
} from "./binding";

export type { HighlightSegment } from "./binding";

export type {
  BindingInfo,
  NapiEngine,
  NapiEventBus,
  NapiFocusManager,
  NapiKeymap,
  NapiTextEngine,
  NapiScheduler,
  NapiTheme,
  NapiThemeBorders,
  NapiThemeColors,
  NapiThemeSpacing,
  NapiWidgetHost,
  ProcessResult,
  TerminalCapabilities,
  SchedulerStats,
} from "./types";
