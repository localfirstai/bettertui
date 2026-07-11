import { CommandBuffer, createReconciler } from "@bettertui/core";
import {
  Badge,
  DataTable,
  Flex,
  Heading,
  Provider,
  Separator,
  StatusLine,
  Table,
  Text,
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

const users = [
  { name: "Alice", role: "Admin", status: "active", commits: 142 },
  { name: "Bob", role: "Editor", status: "active", commits: 87 },
  { name: "Charlie", role: "Viewer", status: "inactive", commits: 12 },
  { name: "Diana", role: "Editor", status: "active", commits: 203 },
  { name: "Eve", role: "Admin", status: "active", commits: 56 },
];

const columns = [
  { key: "name", header: "Name", width: 12 },
  { key: "role", header: "Role", width: 10 },
  { key: "status", header: "Status", width: 10 },
  { key: "commits", header: "Commits", width: 10, align: "right" as const },
];

interface TableDemoProps {
  selectedIndex: number;
}

function TableDemo({ selectedIndex }: TableDemoProps) {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Heading level={2}>Table Example</Heading>

        <Separator />

        <Heading level={3}>Basic Table</Heading>
        <Table
          columns={["Name", "Role", "Status"]}
          rows={[
            ["Alice", "Admin", "active"],
            ["Bob", "Editor", "active"],
            ["Charlie", "Viewer", "inactive"],
          ]}
        />

        <Separator />

        <Heading level={3}>DataTable with Selection</Heading>
        <DataTable columns={columns} rows={users} selectedIndex={selectedIndex} />

        <Separator />

        <Flex flexDirection="row" gap={1}>
          {users.map((u, i) => (
            <Badge key={u.name} variant={i === selectedIndex ? "success" : "info"}>
              {u.name}
            </Badge>
          ))}
        </Flex>

        <Separator />

        <Text dimColor>Use j/k or arrow keys to navigate, q to quit</Text>

        <StatusLine
          items={[
            { label: "Selected", value: users[selectedIndex]?.name ?? "-" },
            { label: "Row", value: `${selectedIndex + 1}/${users.length}` },
          ]}
        />
      </Flex>
    </Provider>
  );
}

let selectedIndex = 0;

function renderApp() {
  const element = <TableDemo selectedIndex={selectedIndex} />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Table Demo");
console.log("Navigate with j/k or arrow keys, q to quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "j" || key === "\x1b[B") {
    selectedIndex = Math.min(selectedIndex + 1, users.length - 1);
    renderApp();
  } else if (key === "k" || key === "\x1b[A") {
    selectedIndex = Math.max(selectedIndex - 1, 0);
    renderApp();
  } else if (key === "q") {
    process.exit(0);
  }
});
