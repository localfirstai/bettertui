import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface ScrollAreaProps {
  children?: ReactNode;
  scrollTop?: number;
  scrollLeft?: number;
  showScrollbar?: boolean;
  onScroll?: (scrollTop: number, scrollLeft: number) => void;
  style?: Record<string, unknown> | undefined;
}

export function ScrollArea(props: ScrollAreaProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("ScrollArea", { style: userStyle, ...rest }, children);
}
