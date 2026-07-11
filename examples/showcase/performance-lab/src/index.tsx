import { CommandBuffer, createReconciler } from "@bettertui/core";
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
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface TestResult {
  name: string;
  fps: number;
  renderTime: number;
  frames: number;
  duration: number;
}

interface LabState {
  activeTest: number;
  running: boolean;
  frameCount: number;
  lastFps: number;
  renderTime: number;
  startTime: number;
  memoryUsage: number;
  tableSize: number;
  treeDepth: number;
  animationSpeed: number;
  testHistory: TestResult[];
  tick: number;
}

const state: LabState = {
  activeTest: 0,
  running: false,
  frameCount: 0,
  lastFps: 0,
  renderTime: 0,
  startTime: 0,
  memoryUsage: 12.4,
  tableSize: 200,
  treeDepth: 200,
  animationSpeed: 1,
  testHistory: [],
  tick: 0,
};

let testInterval: ReturnType<typeof setInterval> | null = null;
let fpsInterval: ReturnType<typeof setInterval> | null = null;
let framesThisSecond = 0;

const TEST_NAMES = [
  "Idle Baseline",
  "Large Table",
  "Large Tree",
  "Rapid Updates",
  "Mixed Workload",
];

function generateTableRows(
  count: number,
): { id: string; name: string; value: string; status: string }[] {
  const rows: { id: string; name: string; value: string; status: string }[] = [];
  for (let i = 0; i < count; i++) {
    rows.push({
      id: String(i + 1),
      name: `Item-${i + 1}`,
      value: `${(Math.random() * 100).toFixed(2)}`,
      status: i % 3 === 0 ? "active" : i % 3 === 1 ? "pending" : "done",
    });
  }
  return rows;
}

function generateTreeNodes(
  count: number,
): { id: string; label: string; children?: { id: string; label: string }[] }[] {
  const nodes: { id: string; label: string; children?: { id: string; label: string }[] }[] = [];
  for (let i = 0; i < count; i++) {
    const childCount = Math.floor(Math.random() * 3) + 1;
    const children: { id: string; label: string }[] = [];
    for (let j = 0; j < childCount; j++) {
      children.push({ id: `n${i + 1}.${j + 1}`, label: `Node-${i + 1}.${j + 1}` });
    }
    nodes.push({ id: `n${i + 1}`, label: `Node-${i + 1}`, children });
  }
  return nodes;
}

function formatDuration(ms: number): string {
  const seconds = Math.floor(ms / 1000);
  const minutes = Math.floor(seconds / 60);
  return `${minutes}m ${seconds % 60}s`;
}

function recordTestResult(name: string, frames: number, duration: number): void {
  state.testHistory.push({
    name,
    fps: state.lastFps,
    renderTime: state.renderTime,
    frames,
    duration,
  });
  if (state.testHistory.length > 5) {
    state.testHistory.shift();
  }
}

function stopTest(): void {
  if (testInterval) {
    clearInterval(testInterval);
    testInterval = null;
  }
  if (fpsInterval) {
    clearInterval(fpsInterval);
    fpsInterval = null;
  }

  if (state.running && state.startTime > 0) {
    const duration = Date.now() - state.startTime;
    recordTestResult(TEST_NAMES[state.activeTest] ?? "Unknown", state.frameCount, duration);
  }

  state.running = false;
  state.startTime = 0;
  renderApp();
}

function startTest(): void {
  if (state.running) {
    stopTest();
    return;
  }

  state.running = true;
  state.startTime = Date.now();
  state.frameCount = 0;
  state.lastFps = 0;
  state.renderTime = 0;
  framesThisSecond = 0;

  fpsInterval = setInterval(() => {
    state.lastFps = framesThisSecond;
    framesThisSecond = 0;
    renderApp();
  }, 1000);

  switch (state.activeTest) {
    case 0: {
      testInterval = setInterval(() => {
        const renderStart = Date.now();
        state.frameCount++;
        framesThisSecond++;
        state.renderTime = Date.now() - renderStart;
        state.memoryUsage = 12.4 + Math.sin(state.frameCount * 0.01) * 0.3;
        renderApp();
      }, 16);
      break;
    }
    case 1: {
      testInterval = setInterval(() => {
        const renderStart = Date.now();
        state.frameCount++;
        framesThisSecond++;
        state.tableSize = 100 + (state.frameCount % 401);
        state.renderTime = Date.now() - renderStart;
        state.memoryUsage = 12.4 + (state.tableSize / 500) * 2;
        renderApp();
      }, 16);
      break;
    }
    case 2: {
      testInterval = setInterval(() => {
        const renderStart = Date.now();
        state.frameCount++;
        framesThisSecond++;
        state.treeDepth = 100 + (state.frameCount % 401);
        state.renderTime = Date.now() - renderStart;
        state.memoryUsage = 12.4 + (state.treeDepth / 500) * 2;
        renderApp();
      }, 16);
      break;
    }
    case 3: {
      testInterval = setInterval(() => {
        const renderStart = Date.now();
        state.frameCount++;
        framesThisSecond++;
        state.renderTime = Date.now() - renderStart;
        state.memoryUsage = 12.4 + Math.random() * 0.5;
        renderApp();
      }, 0);
      break;
    }
    case 4: {
      testInterval = setInterval(() => {
        const renderStart = Date.now();
        state.frameCount++;
        framesThisSecond++;
        state.tableSize = 100 + (state.frameCount % 401);
        state.treeDepth = 100 + (state.frameCount % 401);
        state.animationSpeed = 1 + Math.sin(state.frameCount * 0.05) * 0.5;
        state.renderTime = Date.now() - renderStart;
        state.memoryUsage = 12.4 + (state.tableSize / 500) * 1 + Math.random() * 0.3;
        renderApp();
      }, 16);
      break;
    }
  }

  renderApp();
}

function TestSelector() {
  return (
    <Flex flexDirection="column" gap={0}>
      <Heading level={3}>Test Scenarios</Heading>
      {TEST_NAMES.map((name, i) => {
        const isActive = state.activeTest === i;
        const variant = isActive ? "success" : "info";
        return (
          <Flex key={name} flexDirection="row" alignItems="center" gap={1}>
            <Badge variant={variant}>{i + 1}</Badge>
            <Text bold={isActive}>{name}</Text>
            {isActive && <Text dim> &lt;--</Text>}
          </Flex>
        );
      })}
    </Flex>
  );
}

function MetricsPanel() {
  const uptime = state.startTime > 0 ? formatDuration(Date.now() - state.startTime) : "0s";
  return (
    <Flex flexDirection="column" gap={0}>
      <Heading level={3}>Metrics</Heading>
      <Flex flexDirection="row" gap={2}>
        <Flex flexDirection="column" gap={0}>
          <Text dim>FPS</Text>
          <Text bold color="green">
            {state.lastFps}
          </Text>
        </Flex>
        <Flex flexDirection="column" gap={0}>
          <Text dim>Frames</Text>
          <Text bold color="blue">
            {state.frameCount}
          </Text>
        </Flex>
        <Flex flexDirection="column" gap={0}>
          <Text dim>Render Time</Text>
          <Text bold color="yellow">
            {state.renderTime}ms
          </Text>
        </Flex>
        <Flex flexDirection="column" gap={0}>
          <Text dim>Memory</Text>
          <Text bold color="magenta">
            {state.memoryUsage.toFixed(1)} MB
          </Text>
        </Flex>
        <Flex flexDirection="column" gap={0}>
          <Text dim>Uptime</Text>
          <Text bold>{uptime}</Text>
        </Flex>
      </Flex>
    </Flex>
  );
}

function MetricProgressBars() {
  const fpsPercent = Math.min(state.lastFps, 60);
  const memPercent = Math.min((state.memoryUsage / 20) * 100, 100);
  const renderPercent = Math.min(state.renderTime * 10, 100);

  return (
    <Flex flexDirection="column" gap={0}>
      <Flex flexDirection="column" gap={0}>
        <Text dim>FPS ({state.lastFps}/60)</Text>
        <Progress value={fpsPercent} />
      </Flex>
      <Flex flexDirection="column" gap={0}>
        <Text dim>Memory ({state.memoryUsage.toFixed(1)}/20 MB)</Text>
        <Progress value={memPercent} />
      </Flex>
      <Flex flexDirection="column" gap={0}>
        <Text dim>Render Time ({state.renderTime}ms)</Text>
        <Progress value={renderPercent} />
      </Flex>
    </Flex>
  );
}

function TestHistory() {
  if (state.testHistory.length === 0) {
    return (
      <Flex flexDirection="column" gap={0}>
        <Heading level={3}>Test History</Heading>
        <Text dim>No completed tests yet.</Text>
      </Flex>
    );
  }

  return (
    <Flex flexDirection="column" gap={0}>
      <Heading level={3}>Test History (Last 5)</Heading>
      {state.testHistory.map((result, i) => (
        <Flex key={`${result.name}-${i}`} flexDirection="row" gap={1}>
          <Badge variant="info">{i + 1}</Badge>
          <Text>{result.name}</Text>
          <Text dim>
            - {result.fps} fps, {result.frames} frames, {result.renderTime}ms,{" "}
            {formatDuration(result.duration)}
          </Text>
        </Flex>
      ))}
    </Flex>
  );
}

function ActiveTestContent() {
  if (!state.running) {
    return null;
  }

  switch (state.activeTest) {
    case 1: {
      const columns = [
        { key: "id", header: "ID", width: 8 },
        { key: "name", header: "Name", width: 14 },
        { key: "value", header: "Value", width: 10 },
        { key: "status", header: "Status", width: 10 },
      ];
      const rows = generateTableRows(state.tableSize);
      return (
        <Flex flexDirection="column" gap={0}>
          <Flex flexDirection="row" alignItems="center">
            <Heading level={3}>Table ({state.tableSize} rows)</Heading>
          </Flex>
          <DataTable columns={columns} data={rows} />
        </Flex>
      );
    }
    case 2: {
      const treeNodes = generateTreeNodes(state.treeDepth);
      return (
        <Flex flexDirection="column" gap={0}>
          <Flex flexDirection="row" alignItems="center">
            <Heading level={3}>Tree ({state.treeDepth} nodes)</Heading>
          </Flex>
          <Tree nodes={treeNodes} />
        </Flex>
      );
    }
    case 3: {
      return (
        <Flex flexDirection="column" gap={0}>
          <Heading level={3}>Rapid Updates</Heading>
          <Text>Rendering at maximum speed (0ms interval)...</Text>
          <Text>Frame #{state.frameCount}</Text>
          <Progress value={state.frameCount % 100} />
        </Flex>
      );
    }
    case 4: {
      const columns = [
        { key: "id", header: "ID", width: 8 },
        { key: "name", header: "Name", width: 14 },
        { key: "value", header: "Value", width: 10 },
      ];
      const rows = generateTableRows(Math.min(state.tableSize, 50));
      const treeNodes = generateTreeNodes(Math.min(state.treeDepth, 30));
      return (
        <Flex flexDirection="column" gap={0}>
          <Heading level={3}>Mixed Workload</Heading>
          <Flex flexDirection="row" gap={1}>
            <Flex flexDirection="column" gap={0}>
              <Text bold>Table ({Math.min(state.tableSize, 50)} rows)</Text>
              <DataTable columns={columns} data={rows} />
            </Flex>
            <Flex flexDirection="column" gap={0}>
              <Text bold>Tree ({Math.min(state.treeDepth, 30)} nodes)</Text>
              <Tree nodes={treeNodes} />
            </Flex>
          </Flex>
          <Progress value={state.frameCount % 100} />
          <Badge variant={state.animationSpeed > 1.2 ? "success" : "warning"}>
            Speed: {state.animationSpeed.toFixed(2)}x
          </Badge>
        </Flex>
      );
    }
    default:
      return null;
  }
}

function PerformanceLab() {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Performance Lab</Heading>
          <Badge variant={state.running ? "success" : "info"}>
            {state.running ? "Running" : "Stopped"}
          </Badge>
        </Flex>

        <Separator />

        <TestSelector />

        <Separator />

        <MetricsPanel />

        <Separator />

        <MetricProgressBars />

        <Separator />

        <ActiveTestContent />

        <Separator />

        <TestHistory />

        <StatusLine
          items={[
            { label: "Test", value: TEST_NAMES[state.activeTest] ?? "Unknown" },
            { label: "FPS", value: String(state.lastFps) },
            { label: "Frames", value: String(state.frameCount) },
            { label: "Controls", value: "1-5 select space start/stop c clear q quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  const element = <PerformanceLab />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Performance Lab");
console.log("1-5=select test space=start/stop c=clear q=quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key >= "1" && key <= "5") {
    const testIndex = Number(key) - 1;
    if (state.activeTest !== testIndex) {
      if (state.running) {
        stopTest();
      }
      state.activeTest = testIndex;
      renderApp();
    }
  } else if (key === " ") {
    startTest();
  } else if (key === "c") {
    state.testHistory = [];
    state.frameCount = 0;
    state.lastFps = 0;
    state.renderTime = 0;
    state.memoryUsage = 12.4;
    renderApp();
  } else if (key === "q") {
    stopTest();
    process.exit(0);
  }
});
