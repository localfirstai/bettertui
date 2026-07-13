// Data table — tabular data with headers, columns, and a selected row.
//
// Demonstrates: DataTable with typed columns and selectedIndex navigation.
// Builds on the List/Tree selection pattern for rows.
// Next: tree-view, live-metrics.

import { Box, DataTable, Flex, Heading, Provider, Separator, StatusLine } from "@bettertui/react";
import { KeyInput, isMainModule } from "~/lib/keyboard";
import { useExampleKey } from "~/lib/keyboard-context";
import type { ExampleMeta } from "~/lib/meta";
import { mountExample } from "~/lib/standalone";

export const meta: ExampleMeta = {
  slug: "data-table-basics",
  title: "Data Table",
  description: "Tabular data with headers, columns, and a selected row.",
  category: "widgets",
  level: 3,
  tags: ["DataTable", "columns", "data"],
  next: ["tree-view", "live-metrics"],
};

const columns = [
  { key: "pid", header: "PID", width: 8 },
  { key: "name", header: "Name", width: 16 },
  { key: "cpu", header: "CPU%", width: 8 },
  { key: "mem", header: "MEM%", width: 8 },
  { key: "status", header: "Status", width: 10 },
];

const data = [
  { pid: "1", name: "systemd", cpu: "0.1", mem: "0.3", status: "running" },
  { pid: "245", name: "node", cpu: "2.3", mem: "1.2", status: "running" },
  { pid: "1023", name: "bash", cpu: "0.0", mem: "0.1", status: "sleeping" },
  { pid: "2048", name: "code", cpu: "5.2", mem: "3.4", status: "running" },
];

let selectedIndex = 0;

function DataTableExample() {
  useExampleKey((event) => {
    if (event.key === "j" || event.key === "ArrowDown") {
      selectedIndex = Math.min(selectedIndex + 1, data.length - 1);
      return true;
    }
    if (event.key === "k" || event.key === "ArrowUp") {
      selectedIndex = Math.max(selectedIndex - 1, 0);
      return true;
    }
    if (event.key === "q" || event.key === "Escape") return true;
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1} padding={1}>
        <Heading level={1}>Data Table</Heading>
        <Separator />
        <Box style={{ border: { fg: "#648cdc" } }} padding={1}>
          <DataTable
            columns={columns}
            data={data as unknown as Record<string, unknown>[]}
            selectedIndex={selectedIndex}
          />
        </Box>
        <Separator />
        <StatusLine
          items={[
            { label: "Selected", value: data[selectedIndex]?.name ?? "" },
            { label: "j/k", value: "navigate" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

export function run(keyInput: KeyInput): void {
  mountExample(DataTableExample, keyInput);
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = DataTableExample;

if (isMainModule()) {
  const ki = new KeyInput();
  ki.start();
  ki.on((event) => {
    if ((event.key === "q" || event.key === "Escape") && !event.ctrl) {
      destroy(ki);
      ki.stop();
      process.exit(0);
    }
  });
  run(ki);
}
