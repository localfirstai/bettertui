import { describe, expect, it } from "vitest";
import {
  Accordion,
  Badge,
  Blockquote,
  Box,
  Button,
  Calendar,
  CapabilitiesProvider,
  Chart,
  ChatView,
  Checkbox,
  Code,
  CodeBlock,
  Combobox,
  ContextMenu,
  DataTable,
  Diff,
  Dropdown,
  Flex,
  FocusProvider,
  Heading,
  Input,
  Label,
  List,
  Markdown,
  Modal,
  NerdFont,
  Pane,
  Popover,
  Progress,
  PromptComposer,
  Provider,
  Radio,
  RuntimeProvider,
  ScrollArea,
  Select,
  SelectionProvider,
  Separator,
  Slider,
  Slot,
  Spacer,
  Spinner,
  StatusBar,
  StatusLine,
  Switch,
  Table,
  Tabs,
  Terminal,
  TerminalProcess,
  TerminalProvider,
  TerminalViewport,
  Text,
  Textarea,
  ThinkingIndicator,
  Toast,
  Tooltip,
  Tree,
  Viewport,
  easings,
  render,
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

  it("exports container components", () => {
    expect(Flex).toBeDefined();
    expect(Text).toBeDefined();
    expect(Spacer).toBeDefined();
    expect(Separator).toBeDefined();
    expect(Viewport).toBeDefined();
    expect(ScrollArea).toBeDefined();
    expect(Pane).toBeDefined();
  });

  it("exports form components", () => {
    expect(Button).toBeDefined();
    expect(Input).toBeDefined();
    expect(Textarea).toBeDefined();
    expect(Checkbox).toBeDefined();
    expect(Radio).toBeDefined();
    expect(Switch).toBeDefined();
    expect(Slider).toBeDefined();
    expect(Select).toBeDefined();
    expect(Combobox).toBeDefined();
  });

  it("exports display components", () => {
    expect(Badge).toBeDefined();
    expect(Progress).toBeDefined();
    expect(Spinner).toBeDefined();
    expect(Table).toBeDefined();
    expect(DataTable).toBeDefined();
    expect(Tree).toBeDefined();
    expect(List).toBeDefined();
    expect(Calendar).toBeDefined();
    expect(Chart).toBeDefined();
  });

  it("exports overlay components", () => {
    expect(Tooltip).toBeDefined();
    expect(Modal).toBeDefined();
    expect(Popover).toBeDefined();
    expect(Dropdown).toBeDefined();
    expect(ContextMenu).toBeDefined();
    expect(Toast).toBeDefined();
    expect(Tabs).toBeDefined();
    expect(Accordion).toBeDefined();
  });

  it("exports text components", () => {
    expect(Heading).toBeDefined();
    expect(Label).toBeDefined();
    expect(Code).toBeDefined();
    expect(Blockquote).toBeDefined();
  });

  it("exports markdown components", () => {
    expect(Markdown).toBeDefined();
    expect(CodeBlock).toBeDefined();
    expect(Diff).toBeDefined();
  });

  it("exports specialized components", () => {
    expect(PromptComposer).toBeDefined();
    expect(ChatView).toBeDefined();
    expect(StatusBar).toBeDefined();
    expect(StatusLine).toBeDefined();
    expect(ThinkingIndicator).toBeDefined();
    expect(Terminal).toBeDefined();
    expect(TerminalViewport).toBeDefined();
    expect(TerminalProcess).toBeDefined();
    expect(Slot).toBeDefined();
    expect(NerdFont).toBeDefined();
  });
});
