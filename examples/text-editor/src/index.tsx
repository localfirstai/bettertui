import { CommandBuffer, createReconciler } from "@bettertui/core";
import {
  Badge,
  Flex,
  Heading,
  Input,
  Provider,
  Separator,
  Stack,
  StatusLine,
  Text,
  Textarea,
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface EditorState {
  title: string;
  content: string;
  savedContent: string;
  cursorLine: number;
  cursorCol: number;
}

function TextEditor({ state }: { state: EditorState }) {
  const isDirty = state.content !== state.savedContent;
  const charCount = state.content.length;
  const lineCount = state.content.split("\n").length;

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Text Editor</Heading>
          <Spacer />
          {isDirty ? (
            <Badge variant="warning">Modified</Badge>
          ) : (
            <Badge variant="success">Saved</Badge>
          )}
        </Flex>

        <Separator />

        <Flex flexDirection="column" gap={0}>
          <Text dimColor>Title:</Text>
          <Input value={state.title} placeholder="Untitled" width={40} />
        </Flex>

        <Flex flexDirection="column" gap={0}>
          <Text dimColor>Content:</Text>
          <Textarea value={state.content} placeholder="Start typing..." width={60} height={10} />
        </Flex>

        <Separator />

        <Flex flexDirection="row" gap={2}>
          <Text dimColor>Lines: {lineCount}</Text>
          <Text dimColor>Chars: {charCount}</Text>
          <Text dimColor>
            Ln {state.cursorLine}, Col {state.cursorCol}
          </Text>
        </Flex>

        <Separator />

        <Stack gap={0}>
          <Text dimColor>s=save r=reset c=clear q=quit</Text>
        </Stack>

        <StatusLine
          items={[
            { label: "Title", value: state.title || "Untitled" },
            { label: "Length", value: `${charCount}` },
            { label: "Status", value: isDirty ? "Modified" : "Saved" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

const state: EditorState = {
  title: "My Document",
  content:
    "Hello, this is a text editor example.\nYou can edit this content.\nPress s to save, r to reset.",
  savedContent: "",
  cursorLine: 1,
  cursorCol: 1,
};

state.savedContent = state.content;

function renderApp() {
  const element = <TextEditor state={state} />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Text Editor Demo");
console.log("s=save r=reset c=clear q=quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "s") {
    state.savedContent = state.content;
    renderApp();
  } else if (key === "r") {
    state.content = state.savedContent;
    renderApp();
  } else if (key === "c") {
    state.content = "";
    renderApp();
  } else if (key === "q") {
    process.exit(0);
  }
});
