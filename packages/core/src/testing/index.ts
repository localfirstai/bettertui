export { createTestRenderer, createTestRendererSync } from "./test-renderer";
export type {
  TestRendererOptions,
  TestRenderer,
  MockInput,
  MockMouse,
  TestRendererSetup,
} from "./test-renderer";

export { createMockKeys, KeyCodes } from "./mock-keys";
export type { TestKeyInput, MockKeysOptions, KeyModifiers } from "./mock-keys";

export { createMockMouse, MouseButtons } from "./mock-mouse";
export type {
  MouseButton,
  MousePosition,
  MouseModifiers,
  MouseEventType,
  MouseEventOptions,
} from "./mock-mouse";

export { createTestStdin, createTestStdout, TestReadStream, TestWriteStream } from "./test-streams";
export type { TestStdin, TestStdout } from "./test-streams";

export { createSpy } from "./spy";
export type { Spy } from "./spy";

export {
  createTerminalCapabilities,
  createMinimalTerminalCapabilities,
  createFullTerminalCapabilities,
  createKittyTerminalCapabilities,
  createITerm2TerminalCapabilities,
} from "./terminal-capabilities";
export type { TerminalCapabilitiesOptions } from "./terminal-capabilities";

export { createMockNativeKeymap, createTestKeymap } from "./testing";
export type { TestBinding } from "./testing";
