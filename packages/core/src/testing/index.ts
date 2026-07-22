export { createTestRenderer, createTestRendererSync } from "./testRenderer";
export type {
  TestRendererOptions,
  TestRenderer,
  MockInput,
  MockMouse,
  TestRendererSetup,
} from "./testRenderer";

export { createMockKeys, KeyCodes } from "./mockKeys";
export type { TestKeyInput, MockKeysOptions, KeyModifiers } from "./mockKeys";

export { createMockMouse, MouseButtons } from "./mockMouse";
export type {
  MouseButton,
  MousePosition,
  MouseModifiers,
  MouseEventType,
  MouseEventOptions,
} from "./mockMouse";

export { createTestStdin, createTestStdout, TestReadStream, TestWriteStream } from "./testStreams";
export type { TestStdin, TestStdout } from "./testStreams";

export { createSpy } from "./spy";
export type { Spy } from "./spy";

export {
  createTerminalCapabilities,
  createMinimalTerminalCapabilities,
  createFullTerminalCapabilities,
  createKittyTerminalCapabilities,
  createITerm2TerminalCapabilities,
} from "./terminalCapabilities";
export type { TerminalCapabilitiesOptions } from "./terminalCapabilities";

export { createMockNativeKeymap, createTestKeymap } from "./testing";
export type { TestBinding } from "./testing";
