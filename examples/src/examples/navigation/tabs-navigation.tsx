// Tabs & accordion — navigation and disclosure widgets.
//
// Demonstrates: Tabs (activeIndex) and Accordion (expanded) for organizing
// content into switchable/disclosable regions.
// Next: list-view, data-table-basics.

import {
  Accordion,
  Box,
  Flex,
  Heading,
  Provider,
  Separator,
  Tabs,
  Text,
  render,
} from "@bettertui/react";
import type { KeyInput } from "../../lib/keyboard";
import { KeyInputProvider, useExampleKey } from "../../lib/keyboard-context";
import type { ExampleMeta } from "../../lib/meta";

export const meta: ExampleMeta = {
  slug: "tabs-navigation",
  title: "Tabs & Accordion",
  description: "Switchable tabs and expandable accordion sections for content organization.",
  category: "navigation",
  level: 2,
  tags: ["Tabs", "TabItem", "Accordion"],
  next: ["list-view", "data-table-basics"],
};

let storedKeyInput: KeyInput | null = null;
const tabs = [
  { label: "Overview", id: "overview" },
  { label: "Settings", id: "settings" },
  { label: "About", id: "about" },
];

let activeTab = 1;
let accordionOpen = false;

function TabsAndAccordion() {
  useExampleKey((event) => {
    if (event.key === "ArrowRight") {
      activeTab = (activeTab + 1) % tabs.length;
      renderApp();
    } else if (event.key === "ArrowLeft") {
      activeTab = (activeTab - 1 + tabs.length) % tabs.length;
      renderApp();
    } else if (event.key === " ") {
      accordionOpen = !accordionOpen;
      renderApp();
    } else if (event.key === "q" || event.key === "Escape") {
      return true;
    }
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Heading level={1}>Tabs & Accordion</Heading>
        <Separator />
        <Tabs tabs={tabs} activeIndex={activeTab} />
        <Box padding={1}>
          <Text>You are viewing the "{tabs[activeTab]?.label}" tab.</Text>
        </Box>
        <Separator />
        <Accordion title="Click to expand (or press Space)" expanded={accordionOpen}>
          <Text>This content is revealed when the accordion is expanded.</Text>
        </Accordion>
        <Separator />
        <Text dim>←/→ switch tabs · Space toggle accordion · q quit</Text>
      </Flex>
    </Provider>
  );
}

function renderApp() {
  if (!storedKeyInput) return;
  render(
    <KeyInputProvider keyInput={storedKeyInput}>
      <TabsAndAccordion />
    </KeyInputProvider>,
  );
}

export function run(keyInput: KeyInput): void {
  storedKeyInput = keyInput;
  renderApp();
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = TabsAndAccordion;
