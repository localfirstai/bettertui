// Tabs & accordion — navigation and disclosure widgets.
//
// Demonstrates: Tabs (activeIndex) and Accordion (expanded) for organizing
// content into switchable/disclosable regions.
// Next: list-view, dropdown-menu, tooltip-basics.

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
  useKeyboard,
  useRuntime,
} from "@bettertui/react";
import type { ExampleMeta } from "./lib/meta";

export const meta: ExampleMeta = {
  slug: "tabs-navigation",
  title: "Tabs & Accordion",
  description: "Switchable tabs and expandable accordion sections for content organization.",
  category: "navigation",
  level: 2,
  tags: ["Tabs", "TabItem", "Accordion"],
  next: ["list-view", "dropdown-menu", "tooltip-basics"],
};

const tabs = [
  { label: "Overview", id: "overview" },
  { label: "Settings", id: "settings" },
  { label: "About", id: "about" },
];

let activeTab = 1;
let accordionOpen = false;

function TabsAndAccordion() {
  const runtime = useRuntime();

  useKeyboard((key) => {
    if (key.key === "ArrowRight") {
      activeTab = (activeTab + 1) % tabs.length;
      renderApp();
    } else if (key.key === "ArrowLeft") {
      activeTab = (activeTab - 1 + tabs.length) % tabs.length;
      renderApp();
    } else if (key.key === " ") {
      accordionOpen = !accordionOpen;
      renderApp();
    } else if (key.key === "q") {
      runtime?.runtime.dispose();
      process.exit(0);
    }
    return true;
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
  render(<TabsAndAccordion />);
}

export function run() {
  renderApp();
}

if (import.meta.main) {
  run();
}
