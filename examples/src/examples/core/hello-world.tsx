// Hello World — the smallest possible BetterTUI screen.
//
// Demonstrates: the render() entry point, Provider, Box, Text. Every other
// example builds on this. Next: rendering-engine, flex-layout.

import { Box, Flex, Heading, Provider, Separator, Text } from "@bettertui/react";
import type { KeyInput } from "../../lib/keyboard";
import { useExampleKey } from "../../lib/keyboard-context";
import type { ExampleMeta } from "../../lib/meta";
import { mountExample } from "../../lib/standalone";

export const meta: ExampleMeta = {
  slug: "hello-world",
  title: "Hello World",
  description: "The smallest possible BetterTUI screen rendered through React.",
  category: "core",
  level: 1,
  tags: ["render", "Box", "Text", "Provider"],
  next: ["rendering-engine", "flex-layout"],
};

function HelloWorld() {
  useExampleKey((event) => {
    if (event.key === "q" || event.key === "Escape") {
      return true;
    }
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1} padding={1}>
        <Heading level={1}>Hello, BetterTUI</Heading>
        <Separator />
        <Box padding={1}>
          <Text>This is a terminal UI rendered with React components.</Text>
        </Box>
        <Separator />
        <Text dim>Press q to quit</Text>
      </Flex>
    </Provider>
  );
}

export function run(keyInput: KeyInput): void {
  mountExample(HelloWorld, keyInput);
}

export function destroy(keyInput: KeyInput): void {
  keyInput.off(() => {});
  keyInput.stop();
}

export const Example = HelloWorld;
