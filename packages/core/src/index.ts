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
  ColorValue,
  Color,
  Style,
  BorderStyle,
  Theme,
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
