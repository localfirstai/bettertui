import { createElement } from "react";
import type { JSX, ReactNode } from "react";

// ─── Layout Components ───────────────────────────────────

export type FlexDirection = "row" | "column" | "row-reverse" | "column-reverse";

export type JustifyContent =
  | "flex-start"
  | "center"
  | "flex-end"
  | "space-between"
  | "space-around"
  | "space-evenly";

export type AlignItems = "flex-start" | "center" | "flex-end" | "stretch" | "baseline";

export type AlignSelf = "flex-start" | "center" | "flex-end" | "stretch" | "baseline";

export type Position = "relative" | "absolute";

export type Sizing = number | string;

export interface Padding {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

export interface Margin {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

export interface Inset {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

export interface BoxProps {
  children?: ReactNode;
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
  width?: Sizing;
  height?: Sizing;
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
  style?: Record<string, unknown>;
}

export function Box(props: BoxProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Box", { style: userStyle, ...rest }, children);
}

export interface FlexProps {
  children?: ReactNode;
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
  width?: Sizing;
  height?: Sizing;
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
  style?: Record<string, unknown>;
}

export function Flex(props: FlexProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Flex", { style: userStyle, ...rest }, children);
}

export interface GridProps {
  children?: ReactNode;
  columns?: number;
  rows?: number;
  gap?: number | { row?: number; column?: number };
  columnGap?: number;
  rowGap?: number;
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
  width?: Sizing;
  height?: Sizing;
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
  style?: Record<string, unknown>;
}

export function Grid(props: GridProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Grid", { style: userStyle, ...rest }, children);
}

export interface StackProps {
  children?: ReactNode;
  width?: Sizing;
  height?: Sizing;
  padding?: number | Padding;
  margin?: number | Margin;
  position?: Position;
  zIndex?: number;
  visible?: boolean;
  style?: Record<string, unknown>;
}

export interface StackChildProps {
  zIndex?: number;
  offsetX?: number;
  offsetY?: number;
}

export function Stack(props: StackProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Stack", { style: userStyle, ...rest }, children);
}

export interface SpacerProps {
  size?: number;
}

export function Spacer(props: SpacerProps): JSX.Element {
  return createElement("Spacer", { size: props.size });
}

export interface SeparatorProps {
  orientation?: "horizontal" | "vertical";
  style?: Record<string, unknown>;
}

export function Separator(props: SeparatorProps): JSX.Element {
  return createElement("Separator", { orientation: props.orientation, style: props.style });
}

// ─── Typography Components ───────────────────────────────

export interface TextProps {
  children?: ReactNode;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  dim?: boolean;
  strikethrough?: boolean;
  color?: string;
  bgColor?: string;
  style?: Record<string, unknown>;
}

export function Text(props: TextProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Text", { style: userStyle, ...rest }, children);
}

export interface HeadingProps {
  children?: ReactNode;
  level?: 1 | 2 | 3 | 4 | 5 | 6;
  style?: Record<string, unknown>;
}

export function Heading(props: HeadingProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Heading", { style: userStyle, ...rest }, children);
}

export interface LabelProps {
  children?: ReactNode;
  htmlFor?: string;
  style?: Record<string, unknown>;
}

export function Label(props: LabelProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Label", { style: userStyle, ...rest }, children);
}

export interface CodeProps {
  children?: ReactNode;
  inline?: boolean;
  language?: string;
  style?: Record<string, unknown>;
}

export function Code(props: CodeProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Code", { style: userStyle, ...rest }, children);
}

export interface BlockquoteProps {
  children?: ReactNode;
  style?: Record<string, unknown>;
}

export function Blockquote(props: BlockquoteProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Blockquote", { style: userStyle, ...rest }, children);
}

// ─── Interactive Components ──────────────────────────────

export interface ButtonProps {
  children?: ReactNode;
  variant?: "default" | "primary" | "secondary" | "danger" | "ghost" | "link";
  disabled?: boolean;
  onPress?: () => void;
  style?: Record<string, unknown>;
}

export function Button(props: ButtonProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Button", { style: userStyle, ...rest }, children);
}

export interface InputProps {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  password?: boolean;
  onChange?: (value: string) => void;
  onSubmit?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function Input(props: InputProps): JSX.Element {
  return createElement("Input", props);
}

export interface TextareaProps {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  rows?: number;
  onChange?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function Textarea(props: TextareaProps): JSX.Element {
  return createElement("Textarea", props);
}

export interface CheckboxProps {
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (checked: boolean) => void;
  style?: Record<string, unknown>;
}

export function Checkbox(props: CheckboxProps): JSX.Element {
  return createElement("Checkbox", props);
}

export interface RadioProps {
  name?: string;
  value?: string;
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function Radio(props: RadioProps): JSX.Element {
  return createElement("Radio", props);
}

export interface SwitchProps {
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (checked: boolean) => void;
  style?: Record<string, unknown>;
}

export function Switch(props: SwitchProps): JSX.Element {
  return createElement("Switch", props);
}

export interface SliderProps {
  value?: number;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  onChange?: (value: number) => void;
  style?: Record<string, unknown>;
}

export function Slider(props: SliderProps): JSX.Element {
  return createElement("Slider", props);
}

export interface SelectProps {
  children?: ReactNode;
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  onChange?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function Select(props: SelectProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Select", { style: userStyle, ...rest }, children);
}

export interface ComboboxProps {
  children?: ReactNode;
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  options?: Array<{ label: string; value: string }>;
  onChange?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function Combobox(props: ComboboxProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Combobox", { style: userStyle, ...rest }, children);
}

// ─── Navigation Components ───────────────────────────────

export interface TabItem {
  label: string;
  id?: string;
}

export interface TabsProps {
  tabs: TabItem[];
  activeIndex?: number;
  disabled?: boolean;
  onChange?: (index: number) => void;
  style?: Record<string, unknown>;
}

export function Tabs(props: TabsProps): JSX.Element {
  return createElement("Tabs", props);
}

export interface AccordionProps {
  children?: ReactNode;
  title?: string;
  expanded?: boolean;
  onToggle?: () => void;
  style?: Record<string, unknown>;
}

export function Accordion(props: AccordionProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Accordion", { style: userStyle, ...rest }, children);
}

// ─── Feedback Components ─────────────────────────────────

export interface BadgeProps {
  children?: ReactNode;
  variant?: "default" | "primary" | "success" | "warning" | "danger" | "info";
  style?: Record<string, unknown>;
}

export function Badge(props: BadgeProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Badge", { style: userStyle, ...rest }, children);
}

export interface ProgressProps {
  value?: number;
  max?: number;
  style?: Record<string, unknown>;
}

export function Progress(props: ProgressProps): JSX.Element {
  return createElement("Progress", props);
}

export interface SpinnerProps {
  label?: string;
  type?: "dots" | "line" | "braille" | "arc";
  style?: Record<string, unknown>;
}

export function Spinner(props: SpinnerProps): JSX.Element {
  return createElement("Spinner", props);
}

// ─── Data Display Components ─────────────────────────────

export interface ListItem {
  id: string;
  label: string;
  disabled?: boolean;
}

export interface ListProps {
  items: ListItem[];
  selectedId?: string;
  onSelect?: (id: string) => void;
  style?: Record<string, unknown>;
}

export function List(props: ListProps): JSX.Element {
  return createElement("List", props);
}

export interface TreeNode {
  id: string;
  label: string;
  children?: TreeNode[];
  expanded?: boolean;
}

export interface TreeProps {
  nodes: TreeNode[];
  selectedId?: string;
  onSelect?: (id: string) => void;
  onToggle?: (id: string) => void;
  style?: Record<string, unknown>;
}

export function Tree(props: TreeProps): JSX.Element {
  return createElement("Tree", props);
}

export interface TableColumn<T = Record<string, unknown>> {
  key: string;
  header: string;
  width?: number;
  render?: (value: unknown, row: T) => ReactNode;
}

export interface TableProps<T = Record<string, unknown>> {
  columns: TableColumn<T>[];
  data: T[];
  selectedId?: string;
  onSelect?: (id: string) => void;
  style?: Record<string, unknown>;
}

export function Table(props: TableProps): JSX.Element {
  return createElement("Table", props);
}

export interface DataTableProps<T = Record<string, unknown>> {
  columns: TableColumn<T>[];
  data: T[];
  sortable?: boolean;
  filterable?: boolean;
  selectedId?: string;
  onSelect?: (id: string) => void;
  style?: Record<string, unknown>;
}

export function DataTable(props: DataTableProps): JSX.Element {
  return createElement("DataTable", props);
}

// ─── Overlay Components ──────────────────────────────────

export interface TooltipProps {
  children?: ReactNode;
  content: string;
  delay?: number;
  position?: "top" | "bottom" | "left" | "right";
  style?: Record<string, unknown>;
}

export function Tooltip(props: TooltipProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Tooltip", { style: userStyle, ...rest }, children);
}

export interface ModalProps {
  children?: ReactNode;
  title?: string;
  closable?: boolean;
  onClose?: () => void;
  style?: Record<string, unknown>;
}

export function Modal(props: ModalProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Modal", { style: userStyle, ...rest }, children);
}

export interface PopoverProps {
  children?: ReactNode;
  content?: ReactNode;
  position?: "top" | "bottom" | "left" | "right";
  style?: Record<string, unknown>;
}

export function Popover(props: PopoverProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Popover", { style: userStyle, ...rest }, children);
}

export interface DropdownProps {
  children?: ReactNode;
  items?: Array<{ label: string; value: string; disabled?: boolean }>;
  onSelect?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function Dropdown(props: DropdownProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Dropdown", { style: userStyle, ...rest }, children);
}

export interface ContextMenuProps {
  children?: ReactNode;
  items?: Array<{ label: string; value: string; disabled?: boolean; separator?: boolean }>;
  onSelect?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function ContextMenu(props: ContextMenuProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("ContextMenu", { style: userStyle, ...rest }, children);
}

// ─── Status Components ───────────────────────────────────

export interface ToastProps {
  message: string;
  variant?: "default" | "success" | "warning" | "error" | "info";
  duration?: number;
  onDismiss?: () => void;
  style?: Record<string, unknown>;
}

export function Toast(props: ToastProps): JSX.Element {
  return createElement("Toast", props);
}

export interface StatusLineProps {
  children?: ReactNode;
  items?: Array<{ label: string; value?: string; separator?: boolean }>;
  style?: Record<string, unknown>;
}

export function StatusLine(props: StatusLineProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("StatusLine", { style: userStyle, ...rest }, children);
}

// ─── Container Components ────────────────────────────────

export interface PaneProps {
  children?: ReactNode;
  title?: string;
  border?: boolean;
  scrollable?: boolean;
  style?: Record<string, unknown>;
}

export function Pane(props: PaneProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Pane", { style: userStyle, ...rest }, children);
}

export interface ViewportProps {
  children?: ReactNode;
  width?: number;
  height?: number;
  scrollX?: number;
  scrollY?: number;
  style?: Record<string, unknown>;
}

export function Viewport(props: ViewportProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Viewport", { style: userStyle, ...rest }, children);
}

export interface CalendarProps {
  value?: Date;
  min?: Date;
  max?: Date;
  onSelect?: (date: Date) => void;
  style?: Record<string, unknown>;
}

export function Calendar(props: CalendarProps): JSX.Element {
  return createElement("Calendar", props);
}

export interface ChartProps {
  data?: Array<{ label: string; value: number }>;
  type?: "bar" | "line" | "sparkline";
  width?: number;
  height?: number;
  style?: Record<string, unknown>;
}

export function Chart(props: ChartProps): JSX.Element {
  return createElement("Chart", props);
}

// ─── Scroll Components ────────────────────────────────────

export interface ScrollAreaProps {
  children?: ReactNode;
  scrollTop?: number;
  scrollLeft?: number;
  showScrollbar?: boolean;
  onScroll?: (scrollTop: number, scrollLeft: number) => void;
  style?: Record<string, unknown>;
}

export function ScrollArea(props: ScrollAreaProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("ScrollArea", { style: userStyle, ...rest }, children);
}

// ─── Content Components ───────────────────────────────────

export interface MarkdownProps {
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
  style?: Record<string, unknown>;
}

export function Markdown(props: MarkdownProps): JSX.Element {
  return createElement("Markdown", props);
}

export interface CodeBlockProps {
  code?: string;
  language?: string;
  showLineNumbers?: boolean;
  style?: Record<string, unknown>;
}

export function CodeBlock(props: CodeBlockProps): JSX.Element {
  return createElement("CodeBlock", props);
}

export interface DiffProps {
  oldText?: string;
  newText?: string;
  unified?: boolean;
  style?: Record<string, unknown>;
}

export function Diff(props: DiffProps): JSX.Element {
  return createElement("Diff", props);
}

// ─── Chat/AI Components ───────────────────────────────────

export interface PromptComposerProps {
  placeholder?: string;
  value?: string;
  cursorStyle?: "line" | "block" | "underline";
  maxLines?: number;
  history?: string[];
  disabled?: boolean;
  onSubmit?: (value: string) => void;
  onChange?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function PromptComposer(props: PromptComposerProps): JSX.Element {
  return createElement("PromptComposer", props);
}

export interface ChatMessage {
  role: "user" | "assistant" | "system";
  content: string;
}

export interface ChatViewProps {
  messages?: ChatMessage[];
  messageStyle?: Record<string, unknown>;
  userStyle?: Record<string, unknown>;
  assistantStyle?: Record<string, unknown>;
  systemStyle?: Record<string, unknown>;
  separatorStyle?: Record<string, unknown>;
  style?: Record<string, unknown>;
}

export function ChatView(props: ChatViewProps): JSX.Element {
  return createElement("ChatView", props);
}

export interface StatusBarProps {
  children?: ReactNode;
  items?: Array<{ label: string; value?: string; style?: Record<string, unknown> }>;
  style?: Record<string, unknown>;
}

export function StatusBar(props: StatusBarProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("StatusBar", { style: userStyle, ...rest }, children);
}

export interface ThinkingIndicatorProps {
  label?: string;
  style?: Record<string, unknown>;
}

export function ThinkingIndicator(props: ThinkingIndicatorProps): JSX.Element {
  return createElement("ThinkingIndicator", props);
}

// ─── Terminal Components ──────────────────────────────────

export interface TerminalProps {
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
  style?: Record<string, unknown>;
}

export function Terminal(props: TerminalProps): JSX.Element {
  return createElement("Terminal", props);
}

export interface TerminalViewportProps {
  children?: ReactNode;
  scrollOffset?: number;
  scrollMode?: "fixed" | "scrollable" | "infinite";
  style?: Record<string, unknown>;
}

export function TerminalViewport(props: TerminalViewportProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("TerminalViewport", { style: userStyle, ...rest }, children);
}

export interface TerminalProcessProps {
  program?: string;
  args?: string[];
  cwd?: string;
  env?: Record<string, string>;
  autoRestart?: boolean;
  restartDelay?: number;
  onSpawn?: (pid: number) => void;
  onExit?: (code: number) => void;
  onError?: (error: string) => void;
  style?: Record<string, unknown>;
}

export function TerminalProcess(props: TerminalProcessProps): JSX.Element {
  return createElement("TerminalProcess", props);
}
