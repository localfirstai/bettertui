import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface TabItem {
  label: string;
  id?: string;
}

export interface TabsProps {
  tabs: TabItem[];
  activeIndex?: number;
  disabled?: boolean;
  onChange?: (index: number) => void;
  style?: Record<string, unknown> | undefined;
}

export function Tabs(props: TabsProps): JSX.Element {
  return createElement("Tabs", props);
}

export interface AccordionProps {
  children?: ReactNode;
  title?: string;
  expanded?: boolean;
  onToggle?: () => void;
  style?: Record<string, unknown> | undefined;
}

export function Accordion(props: AccordionProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Accordion", { style: userStyle, ...rest }, children);
}
