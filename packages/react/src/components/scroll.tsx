import type { ScrollAreaOptions } from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface ScrollAreaProps extends ScrollAreaOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function ScrollArea(props: ScrollAreaProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("ScrollArea", { style: userStyle, ...rest }, children);
}
