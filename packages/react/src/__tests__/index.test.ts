import { describe, expect, it } from "vitest";
import {
  Box,
  CapabilitiesProvider,
  Code,
  Diff,
  FocusProvider,
  Input,
  Markdown,
  Provider,
  RuntimeProvider,
  ScrollBar,
  ScrollBox,
  Select,
  SelectionProvider,
  Slider,
  TabSelect,
  TerminalProvider,
  Text,
  TextTable,
  Textarea,
  easings,
  render,
  renderToStringAsync,
  useAnimation,
  useCapabilities,
  useClipboard,
  useFocus,
  useFrame,
  useKeyboard,
  useMouse,
  useResize,
  useRuntime,
  useSelection,
  useTerminal,
  useTheme,
  useTimeline,
} from "../index";

describe("public API exports", () => {
  it("exports hooks as functions", () => {
    expect(typeof useTheme).toBe("function");
    expect(typeof useFocus).toBe("function");
    expect(typeof useKeyboard).toBe("function");
    expect(typeof useTerminal).toBe("function");
    expect(typeof useResize).toBe("function");
    expect(typeof useFrame).toBe("function");
    expect(typeof useClipboard).toBe("function");
    expect(typeof useAnimation).toBe("function");
    expect(typeof useMouse).toBe("function");
    expect(typeof useSelection).toBe("function");
    expect(typeof useCapabilities).toBe("function");
    expect(typeof useTimeline).toBe("function");
  });

  it("exports providers as components", () => {
    expect(Provider).toBeDefined();
    expect(FocusProvider).toBeDefined();
    expect(TerminalProvider).toBeDefined();
    expect(SelectionProvider).toBeDefined();
    expect(CapabilitiesProvider).toBeDefined();
    expect(RuntimeProvider).toBeDefined();
  });

  it("exports render function", () => {
    expect(typeof render).toBe("function");
  });

  it("exports renderToStringAsync function", () => {
    expect(typeof renderToStringAsync).toBe("function");
  });

  it("exports useRuntime", () => {
    expect(typeof useRuntime).toBe("function");
  });

  it("exports easings object", () => {
    expect(easings).toBeDefined();
    expect(typeof easings).toBe("object");
  });

  it("exports Box component", () => {
    expect(Box).toBeDefined();
    expect(typeof Box).toBe("function");
  });

  it("exports Text component", () => {
    expect(Text).toBeDefined();
    expect(typeof Text).toBe("function");
  });

  it("exports Code component", () => {
    expect(Code).toBeDefined();
    expect(typeof Code).toBe("function");
  });

  it("exports interactive components", () => {
    expect(Input).toBeDefined();
    expect(Textarea).toBeDefined();
    expect(Select).toBeDefined();
    expect(Slider).toBeDefined();
  });

  it("exports TabSelect component", () => {
    expect(TabSelect).toBeDefined();
  });

  it("exports content components", () => {
    expect(Markdown).toBeDefined();
    expect(Diff).toBeDefined();
  });

  it("exports display components", () => {
    expect(TextTable).toBeDefined();
  });

  it("exports scroll components", () => {
    expect(ScrollBar).toBeDefined();
    expect(ScrollBox).toBeDefined();
  });
});
