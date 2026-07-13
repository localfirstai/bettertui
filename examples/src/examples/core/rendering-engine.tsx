// Rendering & engine — how BetterTUI draws to the terminal.
//
// Demonstrates: the CommandBuffer + Runtime surface from @bettertui/core that the
// React reconciler sits on top of. This is the "look under the hood" example;
// most apps should use the React render() API instead.
// Next: hello-world, flex-layout.

import { CommandBuffer, createReconciler } from "@bettertui/core";
import { Box, Flex, Heading, Provider, Separator, Text, render } from "@bettertui/react";
import type { KeyInput } from "../../lib/keyboard";
import { useExampleKey } from "../../lib/keyboard-context";
import type { ExampleMeta } from "../../lib/meta";

export const meta: ExampleMeta = {
  slug: "rendering-engine",
  title: "Rendering & Engine",
  description: "The CommandBuffer + reconciler layer the React API builds on.",
  category: "core",
  level: 4,
  tags: ["CommandBuffer", "createReconciler", "Runtime", "engine"],
  next: ["hello-world", "flex-layout"],
};

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

function EngineView() {
  useExampleKey((event) => {
    if (event.key === "q" || event.key === "Escape") return true;
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Heading level={1}>Rendering & Engine</Heading>
        <Separator />
        <Box padding={1}>
          <Flex flexDirection="column" gap={0}>
            <Text bold>Two layers</Text>
            <Text dim>1. React components → reconciler → CommandBuffer</Text>
            <Text dim>2. CommandBuffer → Rust engine → terminal cells</Text>
            <Text>This example drives the reconciler directly via createReconciler.</Text>
          </Flex>
        </Box>
        <Separator />
        <Text dim>Press q to quit</Text>
      </Flex>
    </Provider>
  );
}

function renderApp() {
  const element = <EngineView />;
  reconciler.createInstance("Provider", { children: element });
  render(element);
}

export function run(keyInput: KeyInput): void {
  void keyInput;
  renderApp();
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = EngineView;
