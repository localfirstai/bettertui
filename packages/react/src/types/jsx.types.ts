import type { LayoutConstraints, Style } from "@bettertui/shared";
import type React from "react";

/**
 * All valid JSX element names for @bettertui/react.
 * These correspond to node kinds understood by the Rust engine.
 */
export type BetterTUIElementType =
  | "box"
  | "text"
  | "input"
  | "textarea"
  | "scrollbox"
  | "scrollbar"
  | "list"
  | "tree"
  | "dialog"
  | "progressbar"
  | "spinner"
  | "badge"
  | "divider"
  | "slider"
  | "code"
  | "markdown"
  | "table"
  | "texttable"
  | "tabselect"
  | "timeline"
  | "diff"
  | "image";

/** Base props shared by all terminal UI elements. */
export interface BaseProps {
  /** Visual style overrides. */
  style?: Style;
  /** Layout / flex properties. */
  layout?: LayoutConstraints;
  flexDirection?: LayoutConstraints["flexDirection"];
  width?: LayoutConstraints["width"];
  height?: LayoutConstraints["height"];
  minWidth?: LayoutConstraints["minWidth"];
  minHeight?: LayoutConstraints["minHeight"];
  maxWidth?: LayoutConstraints["maxWidth"];
  maxHeight?: LayoutConstraints["maxHeight"];
  flexGrow?: LayoutConstraints["flexGrow"];
  flexShrink?: LayoutConstraints["flexShrink"];
  justifyContent?: LayoutConstraints["justifyContent"];
  alignItems?: LayoutConstraints["alignItems"];
  alignSelf?: LayoutConstraints["alignSelf"];
  padding?: LayoutConstraints["padding"];
  margin?: LayoutConstraints["margin"];
  gap?: LayoutConstraints["gap"];
  overflow?: LayoutConstraints["overflow"];
  position?: LayoutConstraints["position"];
  zIndex?: LayoutConstraints["zIndex"];
  fg?: Style["fg"];
  bg?: Style["bg"];
  bold?: Style["bold"];
  italic?: Style["italic"];
  underline?: Style["underline"];
  dim?: Style["dim"];
  strikethrough?: Style["strikethrough"];
  inverse?: Style["inverse"];
  children?: React.ReactNode;
}

export interface InputProps extends BaseProps {
  value?: string;
  placeholder?: string;
  onChange?: (value: string) => void;
  onSubmit?: (value: string) => void;
}

export type TextProps = BaseProps;
export type BoxProps = BaseProps;

export interface ScrollBoxProps extends BaseProps {
  onScroll?: (x: number, y: number) => void;
}

export interface ScrollBarProps extends BaseProps {
  orientation?: "vertical" | "horizontal";
  thumbSize?: number;
  trackSize?: number;
  /** Scroll thumb position (0–1 or pixel offset). Distinct from the layout `position` prop. */
  scrollPosition?: number;
  onChange?: (position: number) => void;
}

export interface SliderProps extends BaseProps {
  value?: number;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  onChange?: (value: number) => void;
}

export interface TableProps extends BaseProps {
  /** Column definitions. */
  columns?: Array<{
    header: string;
    key?: string;
    width?: number;
    align?: "left" | "center" | "right";
  }>;
  /** Row data as 2D string array. */
  rows?: string[][];
  showBorder?: boolean;
  borderStyle?: "single" | "double" | "rounded" | "bold" | "none";
  showHeader?: boolean;
  selectedRow?: number;
  striped?: boolean;
  onSelect?: (row: string[], index: number) => void;
}

export interface TabSelectProps extends BaseProps {
  tabs?: Array<{ label: string; value: string }>;
  activeIndex?: number;
  onChange?: (value: string) => void;
}

export interface MarkdownProps extends BaseProps {
  content?: string;
}

export interface DiffProps extends BaseProps {
  content?: string;
  oldContent?: string;
  newContent?: string;
  language?: string;
}

declare global {
  namespace JSX {
    interface IntrinsicElements {
      box: BoxProps;
      text: TextProps;
      input: InputProps;
      textarea: InputProps;
      scrollbox: ScrollBoxProps;
      scrollbar: ScrollBarProps;
      list: BaseProps & {
        items?: unknown[];
        onSelect?: (item: unknown) => void;
        height?: number;
        searchable?: boolean;
      };
      tree: BaseProps;
      dialog: BaseProps & { open?: boolean; title?: string; onClose?: () => void };
      progressbar: BaseProps & {
        value?: number;
        min?: number;
        max?: number;
        showPercent?: boolean;
        label?: string;
      };
      spinner: BaseProps & { variant?: string; speed?: number; label?: string };
      badge: BaseProps & { label?: string; variant?: string };
      divider: BaseProps & {
        label?: string;
        char?: string;
        orientation?: "horizontal" | "vertical";
      };
      slider: SliderProps;
      code: BaseProps & { language?: string; content?: string };
      markdown: MarkdownProps;
      table: TableProps;
      texttable: BaseProps & { headers?: string[]; rows?: string[][]; showHeader?: boolean };
      tabselect: TabSelectProps;
      timeline: BaseProps & { duration?: number; looping?: boolean; autoPlay?: boolean };
      diff: DiffProps;
      image: BaseProps & { data?: Buffer; width?: number; height?: number; protocol?: string };
    }
  }
}
