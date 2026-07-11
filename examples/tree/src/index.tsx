import { CommandBuffer, createReconciler } from "@bettertui/core";
import {
  Badge,
  Flex,
  Heading,
  Provider,
  Separator,
  Spacer,
  StatusLine,
  Text,
  Tree,
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface TreeNode {
  id: string;
  label: string;
  children?: TreeNode[];
}

const fileTree: TreeNode[] = [
  {
    id: "src",
    label: "src",
    children: [
      {
        id: "components",
        label: "components",
        children: [
          { id: "button.tsx", label: "Button.tsx" },
          { id: "input.tsx", label: "Input.tsx" },
          { id: "modal.tsx", label: "Modal.tsx" },
        ],
      },
      {
        id: "hooks",
        label: "hooks",
        children: [
          { id: "use-theme.ts", label: "useTheme.ts" },
          { id: "use-focus.ts", label: "useFocus.ts" },
        ],
      },
      { id: "index.tsx", label: "index.tsx" },
      { id: "app.tsx", label: "App.tsx" },
    ],
  },
  {
    id: "tests",
    label: "tests",
    children: [
      { id: "button.test.tsx", label: "Button.test.tsx" },
      { id: "input.test.tsx", label: "Input.test.tsx" },
    ],
  },
  { id: "package.json", label: "package.json" },
  { id: "tsconfig.json", label: "tsconfig.json" },
  { id: "readme.md", label: "README.md" },
];

function countNodes(nodes: TreeNode[]): number {
  let count = 0;
  for (const node of nodes) {
    count++;
    if (node.children) {
      count += countNodes(node.children);
    }
  }
  return count;
}

function TreeDemo({ selectedId }: { selectedId: string }) {
  const totalFiles = countNodes(fileTree);

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Tree View Demo</Heading>
          <Spacer />
          <Badge variant="info">{totalFiles} items</Badge>
        </Flex>

        <Separator />

        <Heading level={3}>File Explorer</Heading>
        <Tree nodes={fileTree} selectedId={selectedId} />

        <Separator />

        <Text dimColor>Use j/k to navigate, Enter to expand/collapse, q to quit</Text>

        <StatusLine
          items={[
            { label: "Selected", value: selectedId || "none" },
            { label: "Files", value: `${totalFiles}` },
          ]}
        />
      </Flex>
    </Provider>
  );
}

let selectedId = "src";

function renderApp() {
  const element = <TreeDemo selectedId={selectedId} />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Tree View Demo");
console.log("Navigate with j/k, q to quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "j") {
    selectedId = "components";
    renderApp();
  } else if (key === "k") {
    selectedId = "src";
    renderApp();
  } else if (key === "q") {
    process.exit(0);
  }
});
