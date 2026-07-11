// Re-export all hooks and providers
export {
  Provider,
  useTheme,
  FocusProvider,
  useFocus,
  useKeyboard,
  TerminalProvider,
  useTerminal,
  useFrame,
  useClipboard,
  useAnimation,
} from "./hooks/index";

export type {
  Theme,
  ThemeColors,
  ThemeSpacing,
  ProviderProps,
  KeyEvent,
} from "./hooks/index";

// Runtime (render + RuntimeProvider + useRuntime)
export { render, RuntimeProvider, useRuntime } from "./runtime";

// Re-export core types that users need
export type {
  Command,
  CommandBuffer,
  Instance,
  Runtime,
} from "@bettertui/core";

// Box component
import type { JSX, ReactNode } from "react";

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

// Text component
export interface TextProps {
  children?: ReactNode;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  color?: string;
  bgColor?: string;
  style?: Record<string, unknown>;
}

export function Text(props: TextProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

// Flex component
export interface FlexProps {
  children?: ReactNode;
  flexDirection?: "row" | "column";
  gap?: number;
  style?: Record<string, unknown>;
}

export function Flex(props: FlexProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

// Spacer component
export interface SpacerProps {
  size?: number;
}

export function Spacer(_props: SpacerProps): JSX.Element {
  return null as unknown as JSX.Element;
}

// Button component
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

// Input component
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

// Textarea component
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

// Tabs component
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

// Modal component
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

// Badge component
export interface BadgeProps {
  children?: ReactNode;
  variant?: "default" | "primary" | "success" | "warning" | "danger" | "info";
  style?: Record<string, unknown>;
}

export function Badge(props: BadgeProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

// Progress component
export interface ProgressProps {
  value?: number;
  max?: number;
  style?: Record<string, unknown>;
}

export function Progress(_props: ProgressProps): JSX.Element {
  return null as unknown as JSX.Element;
}

// Spinner component
export interface SpinnerProps {
  label?: string;
  type?: "dots" | "line" | "braille" | "arc";
  style?: Record<string, unknown>;
}

export function Spinner(_props: SpinnerProps): JSX.Element {
  return null as unknown as JSX.Element;
}

// Tooltip component
export interface TooltipProps {
  children?: ReactNode;
  content: string;
  delay?: number;
  style?: Record<string, unknown>;
}

export function Tooltip(props: TooltipProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

// Separator component
export interface SeparatorProps {
  orientation?: "horizontal" | "vertical";
  style?: Record<string, unknown>;
}

export function Separator(_props: SeparatorProps): JSX.Element {
  return null as unknown as JSX.Element;
}

// Heading component
export interface HeadingProps {
  children?: ReactNode;
  level?: 1 | 2 | 3 | 4 | 5 | 6;
  style?: Record<string, unknown>;
}

export function Heading(props: HeadingProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

// Label component
export interface LabelProps {
  children?: ReactNode;
  htmlFor?: string;
  style?: Record<string, unknown>;
}

export function Label(props: LabelProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

// Code component
export interface CodeProps {
  children?: ReactNode;
  inline?: boolean;
  language?: string;
  style?: Record<string, unknown>;
}

export function Code(props: CodeProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

// Grid component
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

// Stack component
export interface StackProps {
  children?: ReactNode;
  style?: Record<string, unknown>;
}

export function Stack(props: StackProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}
