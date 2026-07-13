// Live metrics — a simulated real-time dashboard with auto-updating data.
//
// Demonstrates: an auto-ticking state (setInterval) driving CPU/memory/disk/
// network panels and a sortable process DataTable. Builds on data-table-basics.
// Next: data-table-basics, performance-stress-test.

import {
  Badge,
  Box,
  DataTable,
  Flex,
  Grid,
  Heading,
  Progress,
  Provider,
  Separator,
  Spacer,
  StatusLine,
  Text,
  render,
} from "@bettertui/react";
import type { KeyInput } from "../../lib/keyboard";
import { useExampleKey } from "../../lib/keyboard-context";
import type { ExampleMeta } from "../../lib/meta";

export const meta: ExampleMeta = {
  slug: "live-metrics",
  title: "Live Metrics",
  description: "A simulated real-time system dashboard with auto-updating metrics.",
  category: "performance",
  level: 4,
  tags: ["setInterval", "DataTable", "Progress", "live data"],
  next: ["data-table-basics", "performance-stress-test"],
};

const state = {
  tick: 0,
  cpu: 35,
  mem: 4.2,
  disk: 64,
  net: 1.2,
  processes: [
    { pid: 1, name: "systemd", cpu: 0.1, mem: 0.3, status: "running" },
    { pid: 245, name: "node", cpu: 2.3, mem: 1.2, status: "running" },
    { pid: 1023, name: "bash", cpu: 0.0, mem: 0.1, status: "sleeping" },
    { pid: 2048, name: "code", cpu: 5.2, mem: 3.4, status: "running" },
  ],
  selected: 0,
};

const processColumns = [
  { key: "pid", header: "PID", width: 8 },
  { key: "name", header: "Name", width: 14 },
  { key: "cpu", header: "CPU%", width: 8 },
  { key: "mem", header: "MEM%", width: 8 },
  { key: "status", header: "Status", width: 10 },
];

function clamp(v: number, lo: number, hi: number) {
  return Math.max(lo, Math.min(hi, v));
}

function tick() {
  state.tick++;
  state.cpu = clamp(state.cpu + (Math.random() - 0.5) * 8, 5, 95);
  state.mem = clamp(state.mem + (Math.random() - 0.5) * 0.4, 4, 14);
  state.disk = clamp(state.disk + (Math.random() - 0.5) * 2, 50, 90);
  state.net = clamp(state.net + (Math.random() - 0.5) * 0.6, 0.1, 5);
  for (const p of state.processes) {
    p.cpu = clamp(p.cpu + (Math.random() - 0.5) * 2, 0, 30);
    p.mem = clamp(p.mem + (Math.random() - 0.5) * 0.2, 0, 8);
  }
  renderApp();
}

function LiveMetrics() {
  useExampleKey((event) => {
    if (event.key === "j") {
      state.selected = Math.min(state.selected + 1, state.processes.length - 1);
      renderApp();
    } else if (event.key === "k") {
      state.selected = Math.max(state.selected - 1, 0);
      renderApp();
    } else if (event.key === "q" || event.key === "Escape") {
      return true;
    }
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Live Metrics</Heading>
          <Spacer />
          <Badge variant="success">Live</Badge>
          <Badge variant="info">tick {state.tick}</Badge>
        </Flex>
        <Separator />
        <Grid columns={2} gap={1}>
          <Box padding={1}>
            <Text bold>CPU {Math.round(state.cpu)}%</Text>
            <Progress value={state.cpu} />
          </Box>
          <Box padding={1}>
            <Text bold>Memory {state.mem.toFixed(1)} GB</Text>
            <Progress value={(state.mem / 16) * 100} />
          </Box>
          <Box padding={1}>
            <Text bold>Disk {state.disk}%</Text>
            <Progress value={state.disk} />
          </Box>
          <Box padding={1}>
            <Text bold>Net {state.net.toFixed(2)} MB/s</Text>
            <Progress value={state.net * 10} />
          </Box>
        </Grid>
        <Separator />
        <DataTable
          columns={processColumns}
          data={
            state.processes.map((p) => ({
              pid: String(p.pid),
              name: p.name,
              cpu: p.cpu.toFixed(1),
              mem: p.mem.toFixed(1),
              status: p.status,
            })) as unknown as Record<string, unknown>[]
          }
          selectedIndex={state.selected}
        />
        <Separator />
        <StatusLine
          items={[
            { label: "j/k", value: "nav" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  render(<LiveMetrics />);
}

let timer: ReturnType<typeof setInterval> | null = null;

export function run(keyInput: KeyInput): void {
  void keyInput;
  timer = setInterval(tick, 1000);
  renderApp();
}

export function destroy(keyInput: KeyInput): void {
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
  void keyInput;
}

export const Example = LiveMetrics;
