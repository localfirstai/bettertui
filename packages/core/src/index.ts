export type {
  NodeId,
  Point,
  Size,
  Rect,
  Direction,
  Alignment,
  Overflow,
  LayoutConstraints,
  LayoutResult,
  RenderCommand,
  EventType,
  Event,
  KeyEvent,
  MouseButton,
  MouseEvent,
  ResizeEvent,
  PasteEvent,
  ColorValue,
  Color,
  Style,
  BorderStyle,
  Theme,
  ThemeColors,
  ThemeSpacing,
  Frame,
  FrameCell,
  RenderNode,
} from "@bettertui/shared";

export type NodeType = "text" | "box" | "flex" | "input" | "list" | "custom";

export interface NodeOptions {
  id?: string;
  type: NodeType;
  props?: Record<string, unknown>;
  children?: NodeOptions[];
  style?: Partial<import("@bettertui/shared").Style>;
}

export interface TreeDiff {
  added: string[];
  removed: string[];
  updated: string[];
}

// Framework-agnostic command protocol and tree manipulation
export type {
  HostContext,
  Instance,
  TextInstance,
  HostConfig,
  Command,
  CommandBufferConsumer,
} from "./command-buffer";
export {
  CommandBuffer,
  generateId,
  createInstance,
  createTextInstance,
  appendChild,
  removeChild,
  insertBefore,
  prepareUpdate,
  commitUpdate,
  commitTextUpdate,
  finalizeInitialChildren,
  resetAfterCommit,
} from "./command-buffer";

// Framework-agnostic reconciler (wraps tree ops with command emission)
export { createReconciler } from "./reconciler";

// Framework-agnostic runtime
export { Runtime } from "./runtime";
