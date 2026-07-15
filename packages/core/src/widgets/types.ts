import type {
  AlignItems,
  AlignSelf,
  FlexDirection,
  Inset,
  JustifyContent,
  Margin,
  Padding,
  Position,
  Sizing,
} from "@bettertui/shared";

// ─── Layout ─────────────────────────────────────────────────────────────────

export interface BoxOptions {
  flexDirection?: FlexDirection;
  justifyContent?: JustifyContent;
  alignItems?: AlignItems;
  alignSelf?: AlignSelf;
  flexWrap?: "nowrap" | "wrap";
  flexGrow?: number;
  flexShrink?: number;
  flexBasis?: Sizing;
  gap?: number | { row?: number; column?: number };
  padding?: number | Padding;
  paddingTop?: number;
  paddingRight?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  margin?: number | Margin;
  marginTop?: number;
  marginRight?: number;
  marginBottom?: number;
  marginLeft?: number;
  width?: Sizing | undefined;
  height?: Sizing | undefined;
  minWidth?: Sizing;
  maxWidth?: Sizing;
  minHeight?: Sizing;
  maxHeight?: Sizing;
  position?: Position;
  inset?: Inset;
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
  zIndex?: number;
  visible?: boolean;
}

export interface FlexOptions extends BoxOptions {
  flexDirection: FlexDirection;
}

export interface GridOptions {
  columns?: number;
  rows?: number;
  gap?: number | { row?: number; column?: number };
  columnGap?: number;
  rowGap?: number;
  padding?: number | Padding;
  margin?: number | Margin;
  width?: Sizing;
  height?: Sizing;
  minWidth?: Sizing;
  maxWidth?: Sizing;
  minHeight?: Sizing;
  maxHeight?: Sizing;
  position?: Position;
  zIndex?: number;
  visible?: boolean;
}

export interface StackOptions {
  width?: Sizing;
  height?: Sizing;
  padding?: number | Padding;
  margin?: number | Margin;
  position?: Position;
  zIndex?: number;
  visible?: boolean;
}

export interface SpacerOptions {
  size?: number;
}

export interface SeparatorOptions {
  orientation?: "horizontal" | "vertical";
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

export interface HeadingOptions {
  level?: 1 | 2 | 3 | 4 | 5 | 6;
}

export interface LabelOptions {
  htmlFor?: string;
}

export interface CodeOptions {
  inline?: boolean;
  language?: string;
}

export type BlockquoteOptions = Record<string, never>;

// ─── Interactive ─────────────────────────────────────────────────────────────

export interface ButtonOptions {
  variant?: "default" | "primary" | "secondary" | "danger" | "ghost" | "link";
  disabled?: boolean;
  onPress?: () => void;
}

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

export interface CheckboxOptions {
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (checked: boolean) => void;
}

export interface RadioOptions {
  name?: string;
  value?: string;
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (value: string) => void;
}

export interface SwitchOptions {
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (checked: boolean) => void;
}

export interface SliderOptions {
  value?: number;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  onChange?: (value: number) => void;
}

export interface SelectOptions {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  onChange?: (value: string) => void;
}

export interface ComboboxOptions {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  options?: Array<{ label: string; value: string }>;
  onChange?: (value: string) => void;
}

// ─── Navigation ──────────────────────────────────────────────────────────────

export interface TabItem {
  label: string;
  id?: string;
}

export interface TabsOptions {
  tabs: TabItem[];
  activeIndex?: number;
  disabled?: boolean;
  onChange?: (index: number) => void;
}

export interface AccordionOptions {
  title?: string;
  expanded?: boolean;
  onToggle?: () => void;
}

// ─── Feedback ────────────────────────────────────────────────────────────────

export interface BadgeOptions {
  variant?: "default" | "primary" | "success" | "warning" | "danger" | "info";
}

export interface ProgressOptions {
  value?: number;
  max?: number;
}

export interface SpinnerOptions {
  label?: string;
  type?: "dots" | "line" | "braille" | "arc";
}

// ─── Overlay ─────────────────────────────────────────────────────────────────

export interface TooltipOptions {
  content: string;
  delay?: number;
  position?: "top" | "bottom" | "left" | "right";
}

export interface ModalOptions {
  title?: string;
  closable?: boolean;
  onClose?: () => void;
}

export interface PopoverOptions {
  position?: "top" | "bottom" | "left" | "right";
}

export interface DropdownOptions {
  items?: Array<{ label: string; value: string; disabled?: boolean }>;
  onSelect?: (value: string) => void;
}

export interface ContextMenuOptions {
  items?: Array<{ label: string; value: string; disabled?: boolean; separator?: boolean }>;
  onSelect?: (value: string) => void;
}

// ─── Status ──────────────────────────────────────────────────────────────────

export interface ToastOptions {
  message: string;
  variant?: "default" | "success" | "warning" | "error" | "info";
  duration?: number;
  onDismiss?: () => void;
}

export interface StatusLineOptions {
  items?: Array<{ label: string; value?: string; separator?: boolean }>;
}

// ─── Container ───────────────────────────────────────────────────────────────

export interface PaneOptions {
  title?: string;
  border?: boolean;
  scrollable?: boolean;
}

export interface ViewportOptions {
  width?: number;
  height?: number;
  scrollX?: number;
  scrollY?: number;
}

export interface CalendarOptions {
  value?: Date;
  min?: Date;
  max?: Date;
  onSelect?: (date: Date) => void;
}

export interface ChartOptions {
  data?: Array<{ label: string; value: number }>;
  type?: "bar" | "line" | "sparkline";
  width?: number;
  height?: number;
}

// ─── Scroll ──────────────────────────────────────────────────────────────────

export interface ScrollAreaOptions {
  scrollTop?: number;
  scrollLeft?: number;
  showScrollbar?: boolean;
  onScroll?: (scrollTop: number, scrollLeft: number) => void;
}

// ─── Content ─────────────────────────────────────────────────────────────────

export interface MarkdownOptions {
  content?: string;
  indent?: number;
  headingStyle?: Record<string, unknown>;
  boldStyle?: Record<string, unknown>;
  italicStyle?: Record<string, unknown>;
  codeStyle?: Record<string, unknown>;
  codeBlockStyle?: Record<string, unknown>;
  linkStyle?: Record<string, unknown>;
  quoteStyle?: Record<string, unknown>;
  ruleStyle?: Record<string, unknown>;
}

export interface CodeBlockOptions {
  code?: string;
  language?: string;
  showLineNumbers?: boolean;
}

export interface DiffOptions {
  oldText?: string;
  newText?: string;
  unified?: boolean;
}

// ─── Data Display ────────────────────────────────────────────────────────────

export interface ListItem {
  id: string;
  label: string;
  disabled?: boolean;
}

export interface ListOptions {
  items: ListItem[];
  selectedId?: string;
  onSelect?: (id: string) => void;
}

export interface TreeNode {
  id: string;
  label: string;
  children?: TreeNode[];
  expanded?: boolean;
}

export interface TreeOptions {
  nodes: TreeNode[];
  selectedId?: string;
  onSelect?: (id: string) => void;
  onToggle?: (id: string) => void;
}

export interface TableColumn<T = Record<string, unknown>> {
  key: string;
  header: string;
  width?: number;
  align?: "left" | "center" | "right";
  render?: (value: unknown, row: T) => unknown;
}

export interface TableOptions<T = Record<string, unknown>> {
  columns: (string | TableColumn<T>)[];
  data?: T[];
  rows?: (string | number | boolean)[][];
  selectedId?: string;
  onSelect?: (id: string) => void;
}

export interface DataTableOptions<T = Record<string, unknown>> {
  columns: TableColumn<T>[];
  data?: T[];
  rows?: T[];
  sortable?: boolean;
  filterable?: boolean;
  selectedIndex?: number;
  onSelect?: (index: number) => void;
}

// ─── Chat ────────────────────────────────────────────────────────────────────

export interface ChatMessage {
  role: "user" | "assistant" | "system";
  content: string;
}

export interface ChatViewOptions {
  messages?: ChatMessage[];
  messageStyle?: Record<string, unknown>;
  userStyle?: Record<string, unknown>;
  assistantStyle?: Record<string, unknown>;
  systemStyle?: Record<string, unknown>;
  separatorStyle?: Record<string, unknown>;
}

export interface PromptComposerOptions {
  placeholder?: string;
  value?: string;
  cursorStyle?: "line" | "block" | "underline";
  maxLines?: number;
  history?: string[];
  disabled?: boolean;
  onSubmit?: (value: string) => void;
  onChange?: (value: string) => void;
}

export interface StatusBarOptions {
  items?: Array<{ label: string; value?: string }>;
}

export interface ThinkingIndicatorOptions {
  label?: string;
}

// ─── Native / Terminal ──────────────────────────────────────────────────────

export interface TerminalOptions {
  program?: string;
  args?: string[];
  cwd?: string;
  env?: Record<string, string>;
  cols?: number;
  rows?: number;
  autoFocus?: boolean;
  cursorStyle?: "block" | "underline" | "bar";
  cursorBlink?: boolean;
  mouseTracking?: boolean;
  onInput?: (data: string) => void;
  onResize?: (cols: number, rows: number) => void;
  onExit?: (code: number) => void;
}

export interface TerminalViewportOptions {
  scrollOffset?: number;
  scrollMode?: "fixed" | "scrollable" | "infinite";
}

export interface TerminalProcessOptions {
  command: string;
  args?: string[];
  cwd?: string;
  env?: Record<string, string>;
}

export interface SlotOptions {
  plugin: string;
  name: string;
}
