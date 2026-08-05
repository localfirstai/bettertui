export { CellMatrix, type CellAttributes } from "./terminal/cell-matrix";
export {
  TestTerminal,
  type TestTerminalOptions,
  type CapturedFrame,
} from "./terminal/test-terminal";

export { MockKeyboard, type KeyboardModifiers, type TypeOptions } from "./keyboard/keyboard";
export { KeyCodes, type KeyCodeName } from "./keyboard/key-codes";

export { MockMouse, MouseButton, type MouseOptions } from "./mouse/mouse";

export { screen, ScreenQueryEngine, type TargetElement } from "./screen/screen";

export { render, type RenderOptions, type RenderResult } from "./react/render";

export { createUserEvent, UserEventInstance } from "./user/user-event";

export { PtyTestSession, type PtySessionOptions } from "./pty/pty-session";

export { setupMatchers } from "./matchers/index";

export { describeBehaviour } from "./specs/runner";
export { BetterTUIDriver, type FrameworkTestingDriver } from "./specs/drivers";

export { runBenchmark, type BenchmarkResult } from "./benchmark/benchmark";
