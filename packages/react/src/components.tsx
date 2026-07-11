import type { JSX, ReactNode } from "react";

// ─── Layout Components ───────────────────────────────────

export interface BoxProps {
  children?: ReactNode;
  flexDirection?: "row" | "column";
  justifyContent?:
    | "flex-start"
    | "center"
    | "flex-end"
    | "space-between"
    | "space-around"
    | "space-evenly";
  alignItems?: "flex-start" | "center" | "flex-end" | "stretch";
  padding?: number;
  margin?: number;
  width?: number | string;
  height?: number | string;
  style?: Record<string, unknown>;
}

export function Box(props: BoxProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface FlexProps {
  children?: ReactNode;
  flexDirection?: "row" | "column";
  gap?: number;
  justifyContent?:
    | "flex-start"
    | "center"
    | "flex-end"
    | "space-between"
    | "space-around"
    | "space-evenly";
  alignItems?: "flex-start" | "center" | "flex-end" | "stretch";
  style?: Record<string, unknown>;
}

export function Flex(props: FlexProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface GridProps {
  children?: ReactNode;
  columns?: number;
  rows?: number;
  gap?: number;
  style?: Record<string, unknown>;
}

export function Grid(props: GridProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface StackProps {
  children?: ReactNode;
  style?: Record<string, unknown>;
}

export function Stack(props: StackProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface SpacerProps {
  size?: number;
}

export function Spacer(_props: SpacerProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface SeparatorProps {
  orientation?: "horizontal" | "vertical";
  style?: Record<string, unknown>;
}

export function Separator(_props: SeparatorProps): JSX.Element {
  return null as unknown as JSX.Element;
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
  return props.children as unknown as JSX.Element;
}

export interface HeadingProps {
  children?: ReactNode;
  level?: 1 | 2 | 3 | 4 | 5 | 6;
  style?: Record<string, unknown>;
}

export function Heading(props: HeadingProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface LabelProps {
  children?: ReactNode;
  htmlFor?: string;
  style?: Record<string, unknown>;
}

export function Label(props: LabelProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface CodeProps {
  children?: ReactNode;
  inline?: boolean;
  language?: string;
  style?: Record<string, unknown>;
}

export function Code(props: CodeProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface BlockquoteProps {
  children?: ReactNode;
  style?: Record<string, unknown>;
}

export function Blockquote(props: BlockquoteProps): JSX.Element {
  return props.children as unknown as JSX.Element;
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
  return props.children as unknown as JSX.Element;
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

export function Input(_props: InputProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface TextareaProps {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  rows?: number;
  onChange?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function Textarea(_props: TextareaProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface CheckboxProps {
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (checked: boolean) => void;
  style?: Record<string, unknown>;
}

export function Checkbox(_props: CheckboxProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function Radio(_props: RadioProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface SwitchProps {
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (checked: boolean) => void;
  style?: Record<string, unknown>;
}

export function Switch(_props: SwitchProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function Slider(_props: SliderProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface SelectProps {
  children?: ReactNode;
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  onChange?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function Select(_props: SelectProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function Combobox(_props: ComboboxProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function Tabs(_props: TabsProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface AccordionProps {
  children?: ReactNode;
  title?: string;
  expanded?: boolean;
  onToggle?: () => void;
  style?: Record<string, unknown>;
}

export function Accordion(_props: AccordionProps): JSX.Element {
  return null as unknown as JSX.Element;
}

// ─── Feedback Components ─────────────────────────────────

export interface BadgeProps {
  children?: ReactNode;
  variant?: "default" | "primary" | "success" | "warning" | "danger" | "info";
  style?: Record<string, unknown>;
}

export function Badge(props: BadgeProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface ProgressProps {
  value?: number;
  max?: number;
  style?: Record<string, unknown>;
}

export function Progress(_props: ProgressProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface SpinnerProps {
  label?: string;
  type?: "dots" | "line" | "braille" | "arc";
  style?: Record<string, unknown>;
}

export function Spinner(_props: SpinnerProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function List(_props: ListProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function Tree(_props: TreeProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function Table(_props: TableProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function DataTable(_props: DataTableProps): JSX.Element {
  return null as unknown as JSX.Element;
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
  return props.children as unknown as JSX.Element;
}

export interface ModalProps {
  children?: ReactNode;
  title?: string;
  closable?: boolean;
  onClose?: () => void;
  style?: Record<string, unknown>;
}

export function Modal(props: ModalProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface PopoverProps {
  children?: ReactNode;
  content?: ReactNode;
  position?: "top" | "bottom" | "left" | "right";
  style?: Record<string, unknown>;
}

export function Popover(props: PopoverProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface DropdownProps {
  children?: ReactNode;
  items?: Array<{ label: string; value: string; disabled?: boolean }>;
  onSelect?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function Dropdown(_props: DropdownProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface ContextMenuProps {
  children?: ReactNode;
  items?: Array<{ label: string; value: string; disabled?: boolean; separator?: boolean }>;
  onSelect?: (value: string) => void;
  style?: Record<string, unknown>;
}

export function ContextMenu(_props: ContextMenuProps): JSX.Element {
  return null as unknown as JSX.Element;
}

// ─── Status Components ───────────────────────────────────

export interface ToastProps {
  message: string;
  variant?: "default" | "success" | "warning" | "error" | "info";
  duration?: number;
  onDismiss?: () => void;
  style?: Record<string, unknown>;
}

export function Toast(_props: ToastProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface StatusLineProps {
  children?: ReactNode;
  items?: Array<{ label: string; value?: string; separator?: boolean }>;
  style?: Record<string, unknown>;
}

export function StatusLine(_props: StatusLineProps): JSX.Element {
  return null as unknown as JSX.Element;
}

// ─── Container Components ────────────────────────────────

export interface PaneProps {
  children?: ReactNode;
  title?: string;
  border?: boolean;
  scrollable?: boolean;
  style?: Record<string, unknown>;
}

export function Pane(_props: PaneProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface ViewportProps {
  children?: ReactNode;
  width?: number;
  height?: number;
  scrollX?: number;
  scrollY?: number;
  style?: Record<string, unknown>;
}

export function Viewport(_props: ViewportProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface CalendarProps {
  value?: Date;
  min?: Date;
  max?: Date;
  onSelect?: (date: Date) => void;
  style?: Record<string, unknown>;
}

export function Calendar(_props: CalendarProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface ChartProps {
  data?: Array<{ label: string; value: number }>;
  type?: "bar" | "line" | "sparkline";
  width?: number;
  height?: number;
  style?: Record<string, unknown>;
}

export function Chart(_props: ChartProps): JSX.Element {
  return null as unknown as JSX.Element;
}
