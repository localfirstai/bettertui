/**
 * JSX type declarations for @bettertui/solid.
 *
 * Declares all valid intrinsic element names and their prop shapes. These are
 * augmented onto `solid-js`'s JSX namespace so editors provide autocomplete
 * inside `.tsx` files that set `jsxImportSource: "@bettertui/solid"`.
 *
 * Tag names follow Solid's convention of using underscores for multi-word
 * elements (e.g. `tab_select`, `progress_bar`) because JSX transpilers may
 * treat hyphens as subtraction operators in some contexts.
 */

import type { LayoutConstraints, Style } from "@bettertui/shared";
import type { JSX as SolidJSX } from "solid-js";

/** All valid JSX element names for @bettertui/solid. */
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
  | "progress_bar"
  | "spinner"
  | "badge"
  | "divider"
  | "slider"
  | "code"
  | "markdown"
  | "table"
  | "texttable"
  | "tab_select"
  | "timeline"
  | "diff"
  | "image";

/** Base props shared by all terminal UI elements. */
export interface BaseProps {
  // biome-ignore lint/suspicious/noExplicitAny: style must accept any to avoid conflict with HTMLElementTags/SVGElementTags which use CSSProperties
  style?: any;
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
  children?: SolidJSX.Element;
}

export interface InputProps extends BaseProps {
  value?: string;
  placeholder?: string;
  // biome-ignore lint/suspicious/noExplicitAny: onChange must accept any to avoid conflict with HTMLInputElement's onChange event type
  onChange?: any;
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
  /** Scroll thumb position (0–1). Distinct from the layout `position` prop. */
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
  columns?: Array<{
    header: string;
    key?: string;
    width?: number;
    align?: "left" | "center" | "right";
  }>;
  rows?: string[][];
  showBorder?: boolean;
  borderStyle?: "single" | "double" | "rounded" | "bold" | "none";
  showHeader?: boolean;
  selectedRow?: number;
  striped?: boolean;
  // biome-ignore lint/suspicious/noExplicitAny: onSelect must accept any to avoid conflict with HTMLTableElement's onSelect event type
  onSelect?: any;
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

// Augment solid-js's JSX namespace with BetterTUI intrinsic elements.
declare module "solid-js" {
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
      progress_bar: BaseProps & {
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
      // biome-ignore lint/suspicious/noExplicitAny: intentional override of conflicting HTML style type
      code: Omit<BaseProps, "style"> & { style?: any; language?: string; content?: string };
      markdown: MarkdownProps;
      table: TableProps;
      texttable: BaseProps & { headers?: string[]; rows?: string[][]; showHeader?: boolean };
      tab_select: TabSelectProps;
      timeline: BaseProps & { duration?: number; looping?: boolean; autoPlay?: boolean };
      diff: DiffProps;
      image: Omit<BaseProps, "style"> & {
        // biome-ignore lint/suspicious/noExplicitAny: intentional override of conflicting HTML style type
        style?: any;
        data?: Buffer;
        width?: number;
        height?: number;
        protocol?: string;
      };
    }
  }
}
