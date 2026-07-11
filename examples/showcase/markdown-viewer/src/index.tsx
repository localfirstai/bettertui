import { CommandBuffer, createReconciler } from "@bettertui/core";
import { Badge, Box, Flex, Heading, Provider, Separator, StatusLine, Text } from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface Section {
  id: string;
  title: string;
  content: string[];
  level: number;
}

const sections: Section[] = [
  {
    id: "intro",
    title: "Introduction",
    content: [
      "BetterTUI is a high-performance terminal UI framework.",
      "It combines Rust's speed with TypeScript's ergonomics.",
    ],
    level: 1,
  },
  {
    id: "getting-started",
    title: "Getting Started",
    content: [
      "Install the package and create your first component.",
      "Use the Provider wrapper for theme support.",
    ],
    level: 2,
  },
  {
    id: "installation",
    title: "Installation",
    content: [
      "Run: pnpm add @bettertui/core @bettertui/react",
      "Requires Node.js 20+ and Rust toolchain.",
    ],
    level: 3,
  },
  {
    id: "usage",
    title: "Basic Usage",
    content: [
      "Import CommandBuffer and createReconciler from core.",
      "Import components from @bettertui/react.",
      "Wrap your app in Provider and render.",
    ],
    level: 2,
  },
  {
    id: "code-example",
    title: "Code Example",
    content: [
      "const buffer = new CommandBuffer();",
      "const reconciler = createReconciler(buffer);",
      "reconciler.createInstance('Provider', { children: element });",
    ],
    level: 3,
  },
];

let selectedIndex = 0;

function MarkdownViewer() {
  const section = sections[selectedIndex];

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Markdown Viewer</Heading>
          <Badge variant="info">{sections.length} sections</Badge>
        </Flex>

        <Separator />

        <Flex flexDirection="row" gap={1}>
          {sections.map((s, i) => (
            <Badge key={s.id} variant={i === selectedIndex ? "success" : "info"}>
              {i + 1}
            </Badge>
          ))}
        </Flex>

        <Separator />

        <Heading level={section.level}>{section.title}</Heading>
        <Box padding={1}>
          <Flex flexDirection="column" gap={0}>
            {section.content.map((line, i) => (
              <Text key={`${section.id}-${i}`}>{line}</Text>
            ))}
          </Flex>
        </Box>

        <Separator />

        <Text dimColor>j=next k=prev q=quit</Text>

        <StatusLine
          items={[
            { label: "Section", value: `${selectedIndex + 1}/${sections.length}` },
            { label: "Title", value: section.title },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  const element = <MarkdownViewer />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Markdown Viewer");
console.log("j=next k=prev q=quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "j") {
    selectedIndex = Math.min(selectedIndex + 1, sections.length - 1);
    renderApp();
  } else if (key === "k") {
    selectedIndex = Math.max(selectedIndex - 1, 0);
    renderApp();
  } else if (key === "q") {
    process.exit(0);
  }
});
