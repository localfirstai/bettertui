import { CommandBuffer, createReconciler } from "@bettertui/core";
import {
  Badge,
  Box,
  Flex,
  Heading,
  Provider,
  Separator,
  Spacer,
  StatusLine,
  Text,
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

const unicodeSets = [
  {
    label: "Box Drawing",
    chars: "┌───┐\n│   │\n└───┘",
  },
  {
    label: "Arrows",
    chars: "← → ↑ ↓  ↔ ⇄ ⇒ ⇔",
  },
  {
    label: "Mathematical",
    chars: "∑ ∏ ∫ √ ∞ ± × ÷ ≠ ≤ ≥",
  },
  {
    label: "Currency",
    chars: "$ € £ ¥ ₹ ₿ ¢ © ® ™",
  },
];

const emojiSets = [
  {
    label: "General",
    chars: "🎉 🚀 💻 🐛 ✅ ❌ ⚡ 🔥",
  },
  {
    label: "Objects",
    chars: "🖥️ ⌨️ 🖱️ 📟 💾 📀 🔌 💡",
  },
  {
    label: "Symbols",
    chars: "⛔ ⚠️ 🔒 🔓 🔑 🗝️ 📍 📌",
  },
];

const styleSets = [
  {
    label: "Basic Styles",
    render: () => (
      <Flex flexDirection="column" gap={0}>
        <Text bold>Bold text</Text>
        <Text dim>Dim text</Text>
        <Text underline>Underlined text</Text>
        <Text strikethrough>Strikethrough text</Text>
      </Flex>
    ),
  },
  {
    label: "Colors",
    render: () => (
      <Flex flexDirection="column" gap={0}>
        <Text color="red">Red text</Text>
        <Text color="green">Green text</Text>
        <Text color="yellow">Yellow text</Text>
        <Text color="blue">Blue text</Text>
        <Text color="magenta">Magenta text</Text>
        <Text color="cyan">Cyan text</Text>
      </Flex>
    ),
  },
  {
    label: "Combined Styles",
    render: () => (
      <Flex flexDirection="column" gap={0}>
        <Text bold color="red">
          Bold red
        </Text>
        <Text dim color="green">
          Dim green
        </Text>
        <Text bold color="yellow">
          Bold yellow
        </Text>
        <Text underline color="blue">
          Underlined blue
        </Text>
      </Flex>
    ),
  },
];

const processConcepts = [
  { name: "shell", pid: 1234, status: "running" as const },
  { name: "editor", pid: 5678, status: "running" as const },
  { name: "build", pid: 9012, status: "exited" as const },
  { name: "test-runner", pid: 3456, status: "error" as const },
];

const statusVariant = (status: "running" | "exited" | "error") => {
  if (status === "running") return "success";
  if (status === "error") return "danger";
  return "default";
};

interface AppProps {
  unicodeIdx: number;
  emojiIdx: number;
  styleIdx: number;
}

function App({ unicodeIdx, emojiIdx, styleIdx }: AppProps) {
  const unicode = unicodeSets[unicodeIdx];
  const emoji = emojiSets[emojiIdx];
  const style = styleSets[styleIdx];

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Box padding={1}>
          <Heading level={1}>Terminal Concepts</Heading>
        </Box>

        <Separator />

        <Flex flexDirection="column" gap={1} padding={1}>
          <Text bold>Unicode Support</Text>
          <Text dim>{unicode.label}</Text>
          <Box padding={1}>
            <Text>{unicode.chars}</Text>
          </Box>
        </Flex>

        <Separator />

        <Flex flexDirection="column" gap={1} padding={1}>
          <Text bold>Emoji Display</Text>
          <Text dim>{emoji.label}</Text>
          <Box padding={1}>
            <Text>{emoji.chars}</Text>
          </Box>
        </Flex>

        <Separator />

        <Flex flexDirection="column" gap={1} padding={1}>
          <Text bold>Terminal Styling</Text>
          <Text dim>{style.label}</Text>
          <Box padding={1}>{style.render()}</Box>
        </Flex>

        <Separator />

        <Flex flexDirection="column" gap={1} padding={1}>
          <Text bold>Process Concepts</Text>
          <Text dim>Simulated process statuses</Text>
          <Flex flexDirection="column" gap={0} padding={1}>
            {processConcepts.map((proc) => (
              <Flex key={proc.name} alignItems="center" gap={1}>
                <Badge variant={statusVariant(proc.status)}>{proc.status}</Badge>
                <Text>{proc.name}</Text>
                <Spacer size={1} />
                <Text dim>pid {proc.pid}</Text>
              </Flex>
            ))}
          </Flex>
        </Flex>

        <Separator />

        <StatusLine
          items={[
            { label: "u", value: "unicode" },
            { label: "e", value: "emoji" },
            { label: "s", value: "style" },
            { separator: true },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp(unicodeIdx: number, emojiIdx: number, styleIdx: number) {
  const element = <App unicodeIdx={unicodeIdx} emojiIdx={emojiIdx} styleIdx={styleIdx} />;
  reconciler.createInstance("Provider", { children: element });
}

let unicodeIdx = 0;
let emojiIdx = 0;
let styleIdx = 0;

console.log("BetterTUI Terminal Demo");
console.log("Press u/e/c to cycle sections, q to quit");

renderApp(unicodeIdx, emojiIdx, styleIdx);

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "u") {
    unicodeIdx = (unicodeIdx + 1) % unicodeSets.length;
    renderApp(unicodeIdx, emojiIdx, styleIdx);
  } else if (key === "e") {
    emojiIdx = (emojiIdx + 1) % emojiSets.length;
    renderApp(unicodeIdx, emojiIdx, styleIdx);
  } else if (key === "s") {
    styleIdx = (styleIdx + 1) % styleSets.length;
    renderApp(unicodeIdx, emojiIdx, styleIdx);
  } else if (key === "q") {
    process.exit(0);
  }
});
