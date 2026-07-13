// List view — selectable, keyboard-navigable item lists.
//
// Demonstrates: List with selectedId and onSelect, plus j/k navigation.
// Next: scroll-area-basics, tree-view.

import { Box, Flex, Heading, List, Provider, Separator, StatusLine } from "@bettertui/react";
import type { KeyInput } from "../../lib/keyboard";
import { useExampleKey } from "../../lib/keyboard-context";
import type { ExampleMeta } from "../../lib/meta";
import { mountExample } from "../../lib/standalone";

export const meta: ExampleMeta = {
  slug: "list-view",
  title: "List View",
  description: "Selectable, keyboard-navigable item lists.",
  category: "containers",
  level: 2,
  tags: ["List", "selection", "navigation"],
  next: ["scroll-area-basics", "tree-view"],
};

const items = [
  { id: "rust", label: "Rust" },
  { id: "typescript", label: "TypeScript" },
  { id: "python", label: "Python" },
  { id: "go", label: "Go" },
  { id: "zig", label: "Zig" },
];

let selectedId = items[0]?.id ?? "";

function ListView() {
  useExampleKey((event) => {
    const idx = items.findIndex((i) => i.id === selectedId);
    if (event.key === "j" || event.key === "ArrowDown") {
      const next = items[idx + 1];
      if (next) selectedId = next.id;
      return true;
    }
    if (event.key === "k" || event.key === "ArrowUp") {
      const prev = items[idx - 1];
      if (prev) selectedId = prev.id;
      return true;
    }
    if (event.key === "q" || event.key === "Escape") return true;
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1} padding={1}>
        <Heading level={1}>List View</Heading>
        <Separator />
        <Box style={{ border: { fg: "#648cdc" } }} padding={1}>
          <List
            items={items}
            selectedId={selectedId}
            onSelect={(id: string) => {
              selectedId = id;
            }}
          />
        </Box>
        <Separator />
        <StatusLine
          items={[
            { label: "Selected", value: selectedId },
            { label: "j/k", value: "navigate" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

export function run(keyInput: KeyInput): void {
  mountExample(ListView, keyInput);
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = ListView;
