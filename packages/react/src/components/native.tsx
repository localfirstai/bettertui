import type { SlotOptions } from "@bettertui/core";
import { createElement } from "react";
import type { JSX } from "react";
import { Text } from "./typography";

export interface SlotProps extends SlotOptions {
  style?: Record<string, unknown> | undefined;
}

export function Slot(props: SlotProps): JSX.Element {
  return createElement("Slot", props);
}

export interface NerdFontProps {
  icon: string;
  style?: Record<string, unknown> | undefined;
}

export function NerdFont({ icon, style }: NerdFontProps): JSX.Element {
  // A minimal bridge for nerdfont string pass-through
  return <Text style={style}>{icon}</Text>;
}
