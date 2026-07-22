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

export interface MarkdownTheme {
  /** Color for h1 headings. Default: "bright_cyan". */
  h1Color?: string;
  /** Color for h2 headings. Default: "cyan". */
  h2Color?: string;
  /** Color for h3-h6 headings. Default: "blue". */
  h3Color?: string;
  /** Color for inline code and code fences. Default: "yellow". */
  codeColor?: string;
  /** Background for code blocks. Default: "bright_black". */
  codeBg?: string;
  /** Color for blockquote text. Default: "bright_black". */
  blockquoteColor?: string;
  /** Color for list bullet/number. Default: "bright_cyan". */
  bulletColor?: string;
  /** Color for links. Default: "blue". */
  linkColor?: string;
  /** Color for horizontal rules. Default: "bright_black". */
  hrColor?: string;
}

export interface MarkdownOptions {
  content?: string;
  /** Override default color theme for markdown elements. */
  theme?: MarkdownTheme;
  /** Maximum width for word wrapping (0 = no wrap). Default: 0. */
  maxWidth?: number;
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
  onChange?: (position: number) => void;
}

export interface ScrollBoxOptions {
  width?: number | "auto" | "100%";
  height?: number | "auto" | "100%";
  scrollX?: boolean;
  scrollY?: boolean;
  onScroll?: (offsetX: number, offsetY: number) => void;
}

// ─── Progress ────────────────────────────────────────────────────────────────

export interface ProgressBarOptions {
  value?: number;
  min?: number;
  max?: number;
  width?: number | "100%";
  showPercent?: boolean;
  color?: string;
  trackColor?: string;
  label?: string;
}

// ─── Spinner ─────────────────────────────────────────────────────────────────

export type SpinnerVariant =
  | "dots"
  | "line"
  | "arc"
  | "bounce"
  | "pipe"
  | "clock"
  | "earth"
  | "moon"
  | "pulse"
  | "star";

export interface SpinnerOptions {
  variant?: SpinnerVariant;
  color?: string;
  label?: string;
  speed?: number;
}

// ─── Badge ───────────────────────────────────────────────────────────────────

export type BadgeVariant = "default" | "primary" | "success" | "warning" | "error" | "info";

export interface BadgeOptions {
  label: string;
  variant?: BadgeVariant;
  color?: string;
  bgColor?: string;
}

// ─── Divider ─────────────────────────────────────────────────────────────────

export interface DividerOptions {
  orientation?: "horizontal" | "vertical";
  label?: string;
  char?: string;
  color?: string;
}

// ─── Tree ────────────────────────────────────────────────────────────────────

export interface TreeNode {
  id: string;
  label: string;
  children?: TreeNode[];
  expanded?: boolean;
  data?: unknown;
}

export interface TreeOptions {
  nodes?: TreeNode[];
  selectedId?: string;
  onSelect?: (node: TreeNode) => void;
  onToggle?: (node: TreeNode, expanded: boolean) => void;
  indentSize?: number;
}

// ─── List ────────────────────────────────────────────────────────────────────

export interface ListItem {
  id: string;
  label: string;
  description?: string;
  disabled?: boolean;
  data?: unknown;
}

export interface ListOptions {
  items?: ListItem[];
  selectedId?: string;
  height?: number;
  onSelect?: (item: ListItem) => void;
  onSelectMulti?: (items: ListItem[]) => void;
  onChange?: (item: ListItem) => void;
  searchable?: boolean;
  placeholder?: string;
  multiSelect?: boolean;
}

// ─── Dialog ──────────────────────────────────────────────────────────────────

export interface DialogOptions {
  title?: string;
  open?: boolean;
  width?: number | "auto";
  height?: number | "auto";
  onClose?: () => void;
  closeOnEsc?: boolean;
  closeOnClickOutside?: boolean;
}

// ─── Timeline ────────────────────────────────────────────────────────────────

export interface TimelineOptions {
  duration?: number;
  looping?: boolean;
  autoPlay?: boolean;
  onComplete?: () => void;
}

export interface TweenConfig {
  from: number;
  to: number;
  duration: number;
  startTime?: number;
  easing?: string;
}

// ─── Image ───────────────────────────────────────────────────────────────────

export type ImageFormat = "rgb" | "rgba" | "png";
export type ImageProtocol = "kitty" | "iterm2" | "sixel" | "auto";

export interface ImageOptions {
  /** Raw pixel data or PNG file bytes (Buffer). */
  data: Buffer;
  width: number;
  height: number;
  /** Pixel format. Default: "png". */
  format?: ImageFormat;
  /** Graphics protocol. Default: "auto" (uses graphicsQuery result). */
  protocol?: ImageProtocol;
  /** Kitty image id (required for Kitty protocol). */
  id?: number;
  /** Optional filename hint for iTerm2. */
  name?: string;
}

// ─── Table ───────────────────────────────────────────────────────────────────

export type TableColumnAlign = "left" | "center" | "right";
export type TableBorderStyle = "single" | "double" | "rounded" | "bold" | "none";

export interface TableColumn {
  header: string;
  key?: string;
  width?: number;
  align?: TableColumnAlign;
  minWidth?: number;
  maxWidth?: number;
}

export interface TableOptions {
  /** Column definitions (header label, optional fixed width, alignment). */
  columns?: TableColumn[];
  /** Row data as 2D string array — each inner array is one row. */
  rows?: string[][];
  /** Show table borders. Default: true. */
  showBorder?: boolean;
  /** Border drawing style. Default: "single". */
  borderStyle?: TableBorderStyle;
  /** Show header row. Default: true. */
  showHeader?: boolean;
  /** Index of the selected row (-1 = no selection). */
  selectedRow?: number;
  /** Zebra striping for alternating rows. Default: false. */
  striped?: boolean;
  /** Compact rendering (no padding around cells). Default: false. */
  compact?: boolean;
  /** Called when a row is selected. */
  onSelect?: (row: string[], index: number) => void;
}
