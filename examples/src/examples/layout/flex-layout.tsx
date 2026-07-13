// Flex layout — row/column flexbox, alignment, and gaps.
//
// Demonstrates: Flex with flexDirection, justifyContent, alignItems, and gap.
// The workhorse for responsive composition. Next: grid-layout, scroll-area-basics.

import { Box, Flex, Heading, Provider, Separator, Text } from "@bettertui/react";
import type { KeyInput } from "../../lib/keyboard";
import { useExampleKey } from "../../lib/keyboard-context";
import type { ExampleMeta } from "../../lib/meta";
import { mountExample } from "../../lib/standalone";

export const meta: ExampleMeta = {
  slug: "flex-layout",
  title: "Flex Layout",
  description: "Row/column flexbox, alignment, and gaps for responsive composition.",
  category: "layout",
  level: 2,
  tags: ["Flex", "alignItems", "justifyContent", "gap"],
  next: ["grid-layout", "scroll-area-basics"],
};

const cells = ["Alpha", "Beta", "Gamma", "Delta"];

function FlexLayout() {
  useExampleKey((event) => {
    if (event.key === "q" || event.key === "Escape") return true;
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1} padding={1}>
        <Heading level={1}>Flex Layout</Heading>
        <Separator />
        <Text bold>Row, space-between, center</Text>
        <Flex flexDirection="row" justifyContent="space-between" alignItems="center">
          {cells.map((c) => (
            <Box key={c} padding={1} style={{ border: { fg: "#648cdc" } }}>
              <Text>{c}</Text>
            </Box>
          ))}
        </Flex>
        <Separator />
        <Text bold>Column, gap, start</Text>
        <Flex flexDirection="column" gap={1} alignItems="flex-start">
          {cells.map((c) => (
            <Box key={c} padding={{ top: 0, right: 2, bottom: 0, left: 2 }}>
              <Text dim>{c}</Text>
            </Box>
          ))}
        </Flex>
        <Separator />
        <Text dim>Press q to quit</Text>
      </Flex>
    </Provider>
  );
}

export function run(keyInput: KeyInput): void {
  mountExample(FlexLayout, keyInput);
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = FlexLayout;
