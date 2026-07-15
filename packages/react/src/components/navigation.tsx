import type { JSX } from "react";

export interface TabSelectProps {
  tabs?: Array<{ label: string; value: string }>;
  activeIndex?: number;
  onChange?: (value: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function TabSelect(_props: TabSelectProps): JSX.Element {
  return <div />;
}
