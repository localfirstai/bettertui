// Scroll area — vertical scrolling for overflowing content.
//
// Demonstrates: ScrollArea with scrollTop and a long content region. Scrolls
// with j/k. Pairs naturally with List/Tree for large datasets.
// Next: list-view, tree-view.

import {
  Flex,
  Heading,
  Provider,
  ScrollArea,
  Separator,
  StatusLine,
  Text,
  render,
} from "@bettertui/react";
import type { KeyInput } from "../../lib/keyboard";
import { useExampleKey } from "../../lib/keyboard-context";
import type { ExampleMeta } from "../../lib/meta";

export const meta: ExampleMeta = {
  slug: "scroll-area-basics",
  title: "Scroll Area",
  description: "Scroll long content vertically with a visible scrollbar.",
  category: "containers",
  level: 2,
  tags: ["ScrollArea", "scrolling"],
  next: ["list-view", "tree-view"],
};

const lines = Array.from({ length: 40 }, (_, i) => `Line ${i + 1} — scrollable content`);

let scrollTop = 0;

function ScrollDemo() {
  useExampleKey((event) => {
    if (event.key === "j" || event.key === "ArrowDown") {
      scrollTop = Math.min(scrollTop + 1, lines.length - 1);
      renderApp();
    } else if (event.key === "k" || event.key === "ArrowUp") {
      scrollTop = Math.max(scrollTop - 1, 0);
      renderApp();
    } else if (event.key === "q" || event.key === "Escape") {
      return true;
    }
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Heading level={1}>Scroll Area</Heading>
        <Separator />
        <ScrollArea scrollTop={scrollTop} showScrollbar>
          <Flex flexDirection="column" gap={0}>
            {lines.map((l) => (
              <Text key={l}>{l}</Text>
            ))}
          </Flex>
        </ScrollArea>
        <Separator />
        <StatusLine
          items={[
            { label: "Scroll", value: `${scrollTop}/${lines.length - 1}` },
            { label: "j/k", value: "scroll" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  render(<ScrollDemo />);
}

export function run(keyInput: KeyInput): void {
  void keyInput;
  renderApp();
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = ScrollDemo;
