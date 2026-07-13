// Tree view — expand/collapse hierarchical data.
//
// Demonstrates: Tree with nested nodes, selectedId, and j/k navigation with
// Enter/Space to toggle expansion. Exercises recursive node attachment.
// Next: data-table-basics, list-view, advanced-data-table.

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
  Tree,
  render,
  useKeyboard,
  useRuntime,
} from "@bettertui/react";
import type { TreeNode } from "@bettertui/react";
import type { ExampleMeta } from "./lib/meta";

export const meta: ExampleMeta = {
  slug: "tree-view",
  title: "Tree View",
  description: "Expand/collapse a file-tree with keyboard navigation and selection.",
  category: "data-display",
  level: 2,
  tags: ["Tree", "TreeNode", "navigation"],
  next: ["data-table-basics", "list-view", "advanced-data-table"],
};

const projectTree: TreeNode[] = [
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
          { id: "table.tsx", label: "Table.tsx" },
        ],
      },
      {
        id: "hooks",
        label: "hooks",
        children: [
          { id: "use-theme.ts", label: "useTheme.ts" },
          { id: "use-focus.ts", label: "useFocus.ts" },
          { id: "use-keyboard.ts", label: "useKeyboard.ts" },
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
  { id: "readme.md", label: "README.md" },
];

function count(nodes: TreeNode[]): { files: number; folders: number } {
  let files = 0;
  let folders = 0;
  for (const n of nodes) {
    if (n.children) {
      folders++;
      const sub = count(n.children);
      files += sub.files;
      folders += sub.folders;
    } else {
      files++;
    }
  }
  return { files, folders };
}

function attachExpanded(node: TreeNode, expanded: Set<string>): TreeNode {
  const out: TreeNode = { ...node, expanded: expanded.has(node.id) };
  if (node.children) {
    out.children = node.children.map((c) => attachExpanded(c, expanded));
  }
  return out;
}

function visiblePaths(nodes: TreeNode[], expanded: Set<string>, parent = ""): string[] {
  const out: string[] = [];
  for (const n of nodes) {
    const path = parent ? `${parent}/${n.id}` : n.id;
    out.push(path);
    if (n.children && expanded.has(n.id)) out.push(...visiblePaths(n.children, expanded, path));
  }
  return out;
}

let selectedId = "src";
const expandedIds = new Set(["src", "components", "hooks"]);

function TreeView() {
  const runtime = useRuntime();
  const { files, folders } = count(projectTree);
  const nodes = projectTree.map((n) => attachExpanded(n, expandedIds));
  const visible = visiblePaths(projectTree, expandedIds);
  const currentIdx = visible.indexOf(selectedId);

  useKeyboard((key) => {
    if (key.key === "j") {
      if (currentIdx < visible.length - 1) selectedId = visible[currentIdx + 1];
    } else if (key.key === "k") {
      if (currentIdx > 0) selectedId = visible[currentIdx - 1];
    } else if (key.key === "Enter" || key.key === " ") {
      if (expandedIds.has(selectedId)) expandedIds.delete(selectedId);
      else expandedIds.add(selectedId);
    } else if (key.key === "q") {
      runtime?.runtime.dispose();
      process.exit(0);
    } else {
      return true;
    }
    renderApp();
    return true;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Tree View</Heading>
          <Spacer />
          <Badge variant="info">{expandedIds.size} expanded</Badge>
        </Flex>
        <Separator />
        <Flex flexDirection="row" gap={2} padding={1}>
          <Flex flexDirection="row" gap={1}>
            <Text bold>Folders:</Text>
            <Badge variant="success">{folders}</Badge>
          </Flex>
          <Flex flexDirection="row" gap={1}>
            <Text bold>Files:</Text>
            <Badge variant="primary">{files}</Badge>
          </Flex>
        </Flex>
        <Separator />
        <Box padding={1}>
          <Tree nodes={nodes} selectedId={selectedId} />
        </Box>
        <Separator />
        <StatusLine
          items={[
            { label: "Selected", value: selectedId },
            { label: "j/k", value: "navigate" },
            { label: "Enter", value: "expand" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  render(<TreeView />);
}

renderApp();
