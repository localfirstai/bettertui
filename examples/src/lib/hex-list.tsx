// Reusable hex-code list renderer. Maps OpenTUI's HexList helper: a scrollable
// column of colour swatches + hex strings. Internal to the examples package.

import { Flex, Text } from "@bettertui/react";

export interface HexItem {
  hex: string;
  label?: string;
}

export function HexList({
  items,
  showLabel = true,
  color = "#dcdce6",
}: {
  items: HexItem[];
  showLabel?: boolean;
  color?: string;
}) {
  return (
    <Flex flexDirection="column" gap={0}>
      {items.map((item) => (
        <Flex key={item.hex} flexDirection="row" gap={1}>
          <Text
            style={{
              bg: item.hex,
              fg: item.hex,
            }}
          >
            {"  "}
          </Text>
          <Text style={{ fg: color }}>{item.hex}</Text>
          {showLabel && item.label ? <Text dim> · {item.label}</Text> : null}
        </Flex>
      ))}
    </Flex>
  );
}
