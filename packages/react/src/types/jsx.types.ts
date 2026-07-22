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
  | "list"
  | "tree"
  | "dialog"
  | "progressbar"
  | "spinner"
  | "badge"
  | "divider"
  | "code"
  | "markdown"
  | "table"
  | "tabselect"
  | "timeline"
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

declare global {
  namespace JSX {
    interface IntrinsicElements {
      box: BoxProps;
      text: TextProps;
      input: InputProps;
      textarea: InputProps;
      scrollbox: ScrollBoxProps;
      list: BaseProps & { items?: unknown[]; onSelect?: (item: unknown) => void };
      tree: BaseProps;
      dialog: BaseProps & { open?: boolean; title?: string; onClose?: () => void };
      progressbar: BaseProps & { value?: number; min?: number; max?: number };
      spinner: BaseProps & { variant?: string; interval?: number; label?: string };
      badge: BaseProps & { variant?: string };
      divider: BaseProps & { label?: string; char?: string };
      code: BaseProps & { language?: string };
      markdown: BaseProps;
      table: BaseProps;
      tabselect: BaseProps & { tabs?: string[]; onSelect?: (tab: string) => void };
      timeline: BaseProps;
      image: BaseProps & { src?: string };
    }
  }
}
