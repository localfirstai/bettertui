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

interface TreeNode {
  id: string;
  label: string;
  children?: TreeNode[];
}

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
      {
        id: "utils",
        label: "utils",
        children: [
          { id: "format.ts", label: "format.ts" },
          { id: "validate.ts", label: "validate.ts" },
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
      { id: "table.test.tsx", label: "Table.test.tsx" },
    ],
  },
  {
    id: "docs",
    label: "docs",
    children: [
      { id: "architecture.md", label: "ARCHITECTURE.md" },
      { id: "contributing.md", label: "CONTRIBUTING.md" },
    ],
  },
  { id: "package.json", label: "package.json" },
  { id: "tsconfig.json", label: "tsconfig.json" },
  { id: "readme.md", label: "README.md" },
];

function countFilesAndFolders(nodes: TreeNode[]): { files: number; folders: number } {
  let files = 0;
  let folders = 0;
  for (const node of nodes) {
    if (node.children) {
      folders++;
      const sub = countFilesAndFolders(node.children);
      files += sub.files;
      folders += sub.folders;
    } else {
      files++;
    }
  }
  return { files, folders };
}

function getVisibleNodes(nodes: TreeNode[], expandedIds: Set<string>, parentPath = ""): string[] {
  const result: string[] = [];
  for (const node of nodes) {
    const path = parentPath ? `${parentPath}/${node.id}` : node.id;
    result.push(path);
    if (node.children && expandedIds.has(node.id)) {
      result.push(...getVisibleNodes(node.children, expandedIds, path));
    }
  }
  return result;
}

function findParentPath(nodes: TreeNode[], targetId: string, parentPath = ""): string | null {
  for (const node of nodes) {
    const path = parentPath ? `${parentPath}/${node.id}` : node.id;
    if (node.id === targetId) return parentPath || "(root)";
    if (node.children) {
      const found = findParentPath(node.children, targetId, path);
      if (found !== null) return found;
    }
  }
  return null;
}

function TreeDemo({
  selectedId,
  expandedIds,
}: {
  selectedId: string;
  expandedIds: Set<string>;
}) {
  const runtime = useRuntime();
  const { files, folders } = countFilesAndFolders(projectTree);
  const nodesWithExpanded = projectTree.map(function attachExpanded(
    node: TreeNode,
  ): import("@bettertui/react").TreeNode {
    return {
      ...node,
      expanded: expandedIds.has(node.id),
      children: node.children?.map(attachExpanded),
    };
  });
  const parentPath = findParentPath(projectTree, selectedId);
  const visible = getVisibleNodes(projectTree, expandedIds);
  const currentIdx = visible.indexOf(selectedId);

  useKeyboard((key) => {
    if (key.key === "j") {
      if (currentIdx < visible.length - 1) {
        selectedId = visible[currentIdx + 1];
      }
      renderApp();
    } else if (key.key === "k") {
      if (currentIdx > 0) {
        selectedId = visible[currentIdx - 1];
      }
      renderApp();
    } else if (key.key === "Enter" || key.key === " ") {
      if (expandedIds.has(selectedId)) {
        expandedIds.delete(selectedId);
      } else {
        expandedIds.add(selectedId);
      }
      renderApp();
    } else if (key.key === "q") {
      runtime?.dispose();
      process.exit(0);
    }
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
          <Tree nodes={nodesWithExpanded} selectedId={selectedId} />
        </Box>

        <Separator />

        <Flex flexDirection="column" gap={0} padding={1}>
          <Flex flexDirection="row" gap={1}>
            <Text bold>Selected:</Text>
            <Text>{selectedId}</Text>
          </Flex>
          {parentPath && (
            <Flex flexDirection="row" gap={1}>
              <Text bold>Parent:</Text>
              <Text dim>{parentPath}</Text>
            </Flex>
          )}
        </Flex>

        <Separator />

        <StatusLine
          items={[
            { label: "j/k", value: "navigate" },
            { label: "Enter", value: "expand" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

const selectedId = "src";
const expandedIds = new Set(["src", "components", "hooks"]);

function renderApp() {
  render(<TreeDemo selectedId={selectedId} expandedIds={expandedIds} />);
}

console.log("BetterTUI Tree View Demo");
console.log("j/k=navigate Enter=toggle q=quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();

process.on("SIGINT", () => {
  process.exit(0);
});
