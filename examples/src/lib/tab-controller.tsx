// Reusable multi-tab example shell. Maps OpenTUI's tab-controller.ts renderable
// into a BetterTUI React component: a tab bar plus one visible panel at a time.
// Internal to the examples package.

import { Box, Flex, Heading, Separator, Text } from "@bettertui/react";
import { useState } from "react";
import type { KeyInput } from "./keyboard";

export interface TabDef {
  id: string;
  title: string;
  render: () => React.ReactNode;
}

export interface TabControllerProps {
  tabs: TabDef[];
  keyInput: KeyInput;
  title?: string;
}

export function TabController({ tabs, keyInput, title }: TabControllerProps) {
  const [active, setActive] = useState(0);

  keyInput.on((event) => {
    if (event.key === "ArrowRight") {
      setActive((i) => (i + 1) % tabs.length);
    } else if (event.key === "ArrowLeft") {
      setActive((i) => (i - 1 + tabs.length) % tabs.length);
    }
    return false;
  });

  const current = tabs[active];
  if (!current) return null;

  return (
    <Flex flexDirection="column" gap={1}>
      {title ? <Heading level={1}>{title}</Heading> : null}
      <Flex flexDirection="row" gap={1}>
        {tabs.map((tab, i) => (
          <Box
            key={tab.id}
            padding={{ top: 0, right: 1, bottom: 0, left: 1 }}
            style={{ inverse: i === active }}
          >
            <Text bold={i === active}>{tab.title}</Text>
          </Box>
        ))}
      </Flex>
      <Separator />
      <Box padding={1}>{current.render()}</Box>
    </Flex>
  );
}
