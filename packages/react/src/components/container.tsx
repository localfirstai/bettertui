import { createElement } from "react";
import type { JSX, ReactNode } from "react";
import { Box, Flex } from "./layout";
import { Text } from "./typography";

export interface PaneProps {
  children?: ReactNode;
  title?: string;
  border?: boolean;
  scrollable?: boolean;
  style?: Record<string, unknown> | undefined;
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
  style?: Record<string, unknown> | undefined;
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
  style?: Record<string, unknown> | undefined;
}

export function Calendar(props: CalendarProps): JSX.Element {
  return createElement("Calendar", props);
}

export interface ChartProps {
  data?: Array<{ label: string; value: number }>;
  type?: "bar" | "line" | "sparkline";
  width?: number;
  height?: number;
  style?: Record<string, unknown> | undefined;
}

export function Chart({ data = [], width = 40, style }: ChartProps): JSX.Element {
  const max = Math.max(1, ...data.map((d) => d.value));
  return (
    <Flex flexDirection="column" style={style}>
      {data.map((d) => {
        const barLen = Math.floor((d.value / max) * (width - 15));
        return (
          <Flex key={d.label} flexDirection="row">
            <Box width={10}>
              <Text>{d.label}</Text>
            </Box>
            <Text>{"█".repeat(Math.max(0, barLen))}</Text>
            <Box width={5}>
              <Text dim> {d.value}</Text>
            </Box>
          </Flex>
        );
      })}
    </Flex>
  );
}
