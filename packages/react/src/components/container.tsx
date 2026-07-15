import type { CalendarOptions, ChartOptions, PaneOptions, ViewportOptions } from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";
import { Box, Flex } from "./layout";
import { Text } from "./typography";

export interface PaneProps extends PaneOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Pane(props: PaneProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Pane", { style: userStyle, ...rest }, children);
}

export interface ViewportProps extends ViewportOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Viewport(props: ViewportProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Viewport", { style: userStyle, ...rest }, children);
}

export interface CalendarProps extends CalendarOptions {
  style?: Record<string, unknown> | undefined;
}

export function Calendar(props: CalendarProps): JSX.Element {
  return createElement("Calendar", props);
}

export interface ChartProps extends ChartOptions {
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
