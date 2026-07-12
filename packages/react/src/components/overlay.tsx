import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface TooltipProps {
  children?: ReactNode;
  content: string;
  delay?: number;
  position?: "top" | "bottom" | "left" | "right";
  style?: Record<string, unknown> | undefined;
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
  style?: Record<string, unknown> | undefined;
}

export function Modal(props: ModalProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Modal", { style: userStyle, ...rest }, children);
}

export interface PopoverProps {
  children?: ReactNode;
  content?: ReactNode;
  position?: "top" | "bottom" | "left" | "right";
  style?: Record<string, unknown> | undefined;
}

export function Popover(props: PopoverProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Popover", { style: userStyle, ...rest }, children);
}

export interface DropdownProps {
  children?: ReactNode;
  items?: Array<{ label: string; value: string; disabled?: boolean }>;
  onSelect?: (value: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function Dropdown(props: DropdownProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Dropdown", { style: userStyle, ...rest }, children);
}

export interface ContextMenuProps {
  children?: ReactNode;
  items?: Array<{ label: string; value: string; disabled?: boolean; separator?: boolean }>;
  onSelect?: (value: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function ContextMenu(props: ContextMenuProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("ContextMenu", { style: userStyle, ...rest }, children);
}
