import type { AccordionOptions, TabItem as CoreTabItem, TabsOptions } from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export type TabItem = CoreTabItem;

export interface TabsProps extends TabsOptions {
  style?: Record<string, unknown> | undefined;
}

export function Tabs(props: TabsProps): JSX.Element {
  return createElement("Tabs", props);
}

export interface AccordionProps extends AccordionOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Accordion(props: AccordionProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Accordion", { style: userStyle, ...rest }, children);
}
