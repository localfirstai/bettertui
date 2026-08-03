export type {
  TerminalCapabilities,
  CommandResult,
  RenderResult,
  NapiEngine,
  NapiEventBus,
  NapiFocusManager,
  NapiTextEngine,
  NapiScheduler,
  NapiKeymap,
  HighlightSegment,
  HighlightedLine,
  NapiWidgetHost,
  NativeSpanFeedOptions,
  NapiSpanFeedStats,
  NapiThemeColors,
  NapiThemeSpacing,
  NapiThemeBorders,
  NapiTheme,
  NapiLoggerConfig,
  NapiDiagnosticSnapshot,
  PluginStateName,
  SlotMode,
  GraphicsFormat,
} from "./binding";

export {
  createEngine,
  createEventBus,
  createFocusManager,
  createKeymap,
  createScheduler,
  createTextEngine,
  createSpanFeed,
  NapiSpanFeed,
  createHitGrid,
  NapiHitGrid,
  createPluginHost,
  NapiPluginHost,
  createTimeline as createNativeTimeline,
  NapiTimeline,
  detectCapabilities,
  getVersion,
  getNativePackageName,
  createWidgetHost,
  createDarkTheme,
  createLightTheme,
  loggerInit,
  loggerSetLevel,
  loggerGetLevel,
  loggerSetModuleFilter,
  loggerGetDiagnostics,
  loggerFlush,
  highlightCode,
  graphicsKittyWrite,
  graphicsKittyDelete,
  graphicsKittyDeleteAll,
  graphicsItermWrite,
  graphicsSixelWrite,
  graphicsQuery,
  clipboardSetSequence,
  clipboardQuerySequence,
  clipboardDecode,
} from "./binding";

export { CliRenderer, createCliRenderer, TerminalConsole } from "./cliRenderer";
export { KeyInput } from "../lib/keyInput";
export type { CliRendererOptions, RawKeyEvent, ThemeMode } from "./cliRenderer";
export { CliRenderEvents } from "./cliRenderer";
export type { ScreenMode, ExternalOutputMode } from "./platform.types";
export type { MouseEvent } from "./events";
export * from "./logger";
export { layoutToEngineJson } from "./layoutSerializer";
