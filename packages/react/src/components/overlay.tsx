import type {
  ContextMenuOptions,
  DropdownOptions,
  ModalOptions,
  PopoverOptions,
  TooltipOptions,
} from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface TooltipProps extends TooltipOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Tooltip(props: TooltipProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Tooltip", { style: userStyle, ...rest }, children);
}

export interface ModalProps extends ModalOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Modal(props: ModalProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Modal", { style: userStyle, ...rest }, children);
}

export interface PopoverProps extends PopoverOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Popover(props: PopoverProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Popover", { style: userStyle, ...rest }, children);
}

export interface DropdownProps extends DropdownOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Dropdown(props: DropdownProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Dropdown", { style: userStyle, ...rest }, children);
}

export interface ContextMenuProps extends ContextMenuOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function ContextMenu(props: ContextMenuProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("ContextMenu", { style: userStyle, ...rest }, children);
}
