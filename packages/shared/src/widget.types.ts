// ─── Layout ─────────────────────────────────────────────────────────────────

export interface BoxOptions {
  flexDirection?: "row" | "column" | "row-reverse" | "column-reverse";
  justifyContent?:
    | "flex-start"
    | "flex-end"
    | "center"
    | "space-between"
    | "space-around"
    | "space-evenly";
  alignItems?: "flex-start" | "flex-end" | "center" | "stretch" | "baseline";
  flexGrow?: number;
  flexShrink?: number;
  flexBasis?: number | "auto";
  gap?: number;
  padding?: number;
  paddingTop?: number;
  paddingRight?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  margin?: number;
  marginTop?: number;
  marginRight?: number;
  marginBottom?: number;
  marginLeft?: number;
  width?: number | "auto" | "100%";
  height?: number | "auto" | "100%";
  minWidth?: number;
  maxWidth?: number;
  minHeight?: number;
  maxHeight?: number;
  border?: boolean;
  borderStyle?: "single" | "double" | "rounded" | "bold" | "none";
  title?: string;
  overflow?: "visible" | "hidden" | "scroll";
  position?: "relative" | "absolute";
  zIndex?: number;
  visible?: boolean;
}

// ─── Typography ──────────────────────────────────────────────────────────────

export interface TextOptions {
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  dim?: boolean;
  strikethrough?: boolean;
  color?: string;
  bgColor?: string;
}

export interface CodeOptions {
  language?: string;
}

// ─── Interactive ─────────────────────────────────────────────────────────────

export interface InputOptions {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  password?: boolean;
  onChange?: (value: string) => void;
  onSubmit?: (value: string) => void;
}

export interface TextareaOptions {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  rows?: number;
  onChange?: (value: string) => void;
}

export interface SelectOptions {
  options?: Array<{ label: string; value: string }>;
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  onChange?: (value: string) => void;
}

export interface SliderOptions {
  value?: number;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  onChange?: (value: number) => void;
}

// ─── Navigation ──────────────────────────────────────────────────────────────

export interface TabSelectOptions {
  tabs?: Array<{ label: string; value: string }>;
  activeIndex?: number;
  onChange?: (value: string) => void;
}

// ─── Content ─────────────────────────────────────────────────────────────────

export interface MarkdownOptions {
  content?: string;
}

export interface DiffOptions {
  content?: string;
  oldContent?: string;
  newContent?: string;
  language?: string;
}

// ─── Display ─────────────────────────────────────────────────────────────────

export interface TextTableOptions {
  headers?: string[];
  rows?: string[][];
  showHeader?: boolean;
}

// ─── Scroll ──────────────────────────────────────────────────────────────────

export interface ScrollBarOptions {
  orientation?: "vertical" | "horizontal";
  thumbSize?: number;
  trackSize?: number;
  position?: number;
}

export interface ScrollBoxOptions {
  width?: number | "auto" | "100%";
  height?: number | "auto" | "100%";
  scrollX?: boolean;
  scrollY?: boolean;
}
