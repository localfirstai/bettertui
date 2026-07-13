// Performance stress test — frame-loop throughput under load.
//
// Demonstrates: driving re-renders on a timer while displaying FPS, frame count,
// and render-time metrics. Uses the React render() loop (not the raw
// createReconciler path) so it matches every other example. Builds on live-metrics.
// Next: live-metrics, data-table-basics.

import {
  Badge,
  DataTable,
  Flex,
  Heading,
  Progress,
  Provider,
  Separator,
  StatusLine,
  Text,
  Tree,
  render,
} from "@bettertui/react";
import type { TreeNode } from "@bettertui/react";
import { KeyInput, isMainModule } from "~/lib/keyboard";
import { KeyInputProvider, useExampleKey } from "~/lib/keyboard-context";
import type { ExampleMeta } from "~/lib/meta";

export const meta: ExampleMeta = {
  slug: "performance-stress-test",
  title: "Performance Stress Test",
  description: "Measure FPS and render time under large-table / large-tree workloads.",
  category: "performance",
  level: 5,
  tags: ["performance", "setInterval", "DataTable", "Tree", "metrics"],
  next: ["live-metrics", "data-table-basics"],
};

let storedKeyInput: KeyInput | null = null;

const TESTS = ["Idle", "Large Table", "Large Tree", "Rapid Updates"];

interface Metrics {
  active: number;
  running: boolean;
  fps: number;
  frames: number;
  renderTime: number;
  tableSize: number;
  treeSize: number;
}

const m: Metrics = {
  active: 0,
  running: false,
  fps: 0,
  frames: 0,
  renderTime: 0,
  tableSize: 200,
  treeSize: 200,
};

let fpsTimer: ReturnType<typeof setInterval> | null = null;
let testTimer: ReturnType<typeof setInterval> | null = null;
let framesThisSecond = 0;

function genTable(n: number) {
  return Array.from({ length: n }, (_, i) => ({
    id: String(i + 1),
    name: `Item-${i + 1}`,
    value: `${(Math.random() * 100).toFixed(2)}`,
  }));
}

function genTree(n: number): TreeNode[] {
  return Array.from({ length: n }, (_, i) => ({
    id: `n${i + 1}`,
    label: `Node-${i + 1}`,
    expanded: false,
  }));
}

function stop() {
  if (testTimer) clearInterval(testTimer);
  if (fpsTimer) clearInterval(fpsTimer);
  testTimer = null;
  fpsTimer = null;
  m.running = false;
  renderApp();
}

function start() {
  if (m.running) {
    stop();
    return;
  }
  m.running = true;
  m.frames = 0;
  framesThisSecond = 0;
  fpsTimer = setInterval(() => {
    m.fps = framesThisSecond;
    framesThisSecond = 0;
    renderApp();
  }, 1000);

  const step = () => {
    const t0 = Date.now();
    if (m.active === 1) m.tableSize = 100 + (m.frames % 401);
    if (m.active === 2) m.treeSize = 100 + (m.frames % 401);
    m.frames++;
    framesThisSecond++;
    m.renderTime = Date.now() - t0;
    renderApp();
  };
  testTimer = setInterval(step, 16);
}

function Stress() {
  useExampleKey((event) => {
    if (event.key >= "1" && event.key <= "4") {
      m.active = Number(event.key) - 1;
      if (m.running) start();
      renderApp();
    } else if (event.key === " ") {
      start();
    } else if (event.key === "c") {
      stop();
      m.fps = 0;
      m.frames = 0;
      m.renderTime = 0;
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
          <Heading level={2}>Performance Stress Test</Heading>
          <Badge variant={m.running ? "success" : "info"}>
            {m.running ? "Running" : "Stopped"}
          </Badge>
        </Flex>
        <Separator />
        <Flex flexDirection="column" gap={0}>
          {TESTS.map((name, i) => (
            <Flex key={name} flexDirection="row" gap={1}>
              <Badge variant={m.active === i ? "success" : "info"}>{i + 1}</Badge>
              <Text bold={m.active === i}>{name}</Text>
            </Flex>
          ))}
        </Flex>
        <Separator />
        <Flex flexDirection="row" gap={2}>
          <Flex flexDirection="column" gap={0}>
            <Text dim>FPS</Text>
            <Text bold color="green">
              {m.fps}
            </Text>
          </Flex>
          <Flex flexDirection="column" gap={0}>
            <Text dim>Frames</Text>
            <Text bold color="blue">
              {m.frames}
            </Text>
          </Flex>
          <Flex flexDirection="column" gap={0}>
            <Text dim>Render</Text>
            <Text bold color="yellow">
              {m.renderTime}ms
            </Text>
          </Flex>
        </Flex>
        <Progress value={Math.min(m.fps, 60)} />
        <Separator />
        {m.active === 1 && (
          <DataTable
            columns={[
              { key: "id", header: "ID", width: 8 },
              { key: "name", header: "Name", width: 14 },
              { key: "value", header: "Value", width: 10 },
            ]}
            data={genTable(Math.min(m.tableSize, 60)) as unknown as Record<string, unknown>[]}
          />
        )}
        {m.active === 2 && <Tree nodes={genTree(Math.min(m.treeSize, 20))} />}
        <Separator />
        <StatusLine
          items={[
            { label: "Test", value: TESTS[m.active] ?? "Unknown" },
            { label: "FPS", value: String(m.fps) },
            { label: "space", value: "start/stop" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  if (!storedKeyInput) return;
  render(
    <KeyInputProvider keyInput={storedKeyInput}>
      <Stress />
    </KeyInputProvider>,
  );
}

export function run(keyInput: KeyInput): void {
  storedKeyInput = keyInput;
  renderApp();
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
  stop();
}

export const Example = Stress;

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
