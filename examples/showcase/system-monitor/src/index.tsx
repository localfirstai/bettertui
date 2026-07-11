import { CommandBuffer, createReconciler } from "@bettertui/core";
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
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface MonitorState {
  tick: number;
  cpu: { user: number; system: number; idle: number; total: number }[];
  memory: { used: number; total: number; cached: number };
  disk: { used: number; total: number; read: number; write: number };
  network: { in: number; out: number; connections: number };
  processes: ProcessInfo[];
  selectedProcess: number;
  sortColumn: "pid" | "name" | "cpu" | "mem";
  sortAsc: boolean;
  showHelp: boolean;
}

interface ProcessInfo {
  pid: number;
  name: string;
  cpu: number;
  memory: number;
  status: "running" | "sleeping" | "stopped";
  user: string;
}

const initialProcesses: ProcessInfo[] = [
  { pid: 1, name: "systemd", cpu: 0.1, memory: 0.3, status: "running", user: "root" },
  { pid: 245, name: "node", cpu: 2.3, memory: 1.2, status: "running", user: "user" },
  { pid: 1023, name: "bash", cpu: 0.0, memory: 0.1, status: "running", user: "user" },
  { pid: 2048, name: "code", cpu: 5.2, memory: 3.4, status: "running", user: "user" },
  { pid: 3072, name: "python", cpu: 1.1, memory: 0.8, status: "sleeping", user: "user" },
  { pid: 4096, name: "rustc", cpu: 12.5, memory: 4.2, status: "running", user: "user" },
  { pid: 5120, name: "cargo", cpu: 3.8, memory: 2.1, status: "running", user: "user" },
  { pid: 6144, name: "tsup", cpu: 8.4, memory: 1.8, status: "running", user: "user" },
  { pid: 7168, name: "pnpm", cpu: 0.2, memory: 0.4, status: "sleeping", user: "user" },
  { pid: 8192, name: "vim", cpu: 0.0, memory: 0.2, status: "sleeping", user: "user" },
];

const state: MonitorState = {
  tick: 0,
  cpu: [
    { user: 12, system: 5, idle: 83, total: 100 },
    { user: 8, system: 3, idle: 89, total: 100 },
    { user: 25, system: 8, idle: 67, total: 100 },
    { user: 5, system: 2, idle: 93, total: 100 },
  ],
  memory: { used: 8.2, total: 16.0, cached: 2.1 },
  disk: { used: 256, total: 512, read: 45, write: 12 },
  network: { in: 1.2, out: 0.8, connections: 24 },
  processes: [...initialProcesses],
  selectedProcess: 0,
  sortColumn: "pid",
  sortAsc: true,
  showHelp: false,
};

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

function fluctuate(value: number, delta: number, min: number, max: number): number {
  const change = (Math.random() - 0.5) * delta * 2;
  return clamp(value + change, min, max);
}

function updateSimulatedData(): void {
  state.tick++;

  for (let i = 0; i < state.cpu.length; i++) {
    const core = state.cpu[i];
    if (!core) continue;
    const newUser = fluctuate(core.user, 5, 0, 90);
    const newSystem = fluctuate(core.system, 3, 0, 30);
    const newIdle = clamp(100 - newUser - newSystem, 5, 99);
    state.cpu[i] = { user: newUser, system: newSystem, idle: newIdle, total: 100 };
  }

  state.memory.used = fluctuate(state.memory.used, 0.3, 4, 14);
  state.memory.cached = fluctuate(state.memory.cached, 0.2, 1, 4);

  state.disk.read = fluctuate(state.disk.read, 10, 0, 200);
  state.disk.write = fluctuate(state.disk.write, 8, 0, 150);

  state.network.in = fluctuate(state.network.in, 0.5, 0.1, 5);
  state.network.out = fluctuate(state.network.out, 0.3, 0.1, 3);
  state.network.connections = Math.round(fluctuate(state.network.connections, 3, 5, 50));

  for (const proc of state.processes) {
    proc.cpu = fluctuate(proc.cpu, 1, 0, 30);
    proc.memory = fluctuate(proc.memory, 0.2, 0, 8);
    if (Math.random() < 0.05) {
      proc.status = proc.status === "running" ? "sleeping" : "running";
    }
  }
}

function sortProcesses(): void {
  const { sortColumn, sortAsc } = state;
  state.processes.sort((a, b) => {
    let cmp: number;
    switch (sortColumn) {
      case "pid":
        cmp = a.pid - b.pid;
        break;
      case "name":
        cmp = a.name.localeCompare(b.name);
        break;
      case "cpu":
        cmp = a.cpu - b.cpu;
        break;
      case "mem":
        cmp = a.memory - b.memory;
        break;
      default:
        cmp = 0;
    }
    return sortAsc ? cmp : -cmp;
  });
}

function formatUptime(tick: number): string {
  const totalSeconds = tick;
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  return `${hours}h ${minutes}m ${seconds}s`;
}

function HelpPanel() {
  return (
    <Box padding={1} borderStyle="rounded">
      <Flex flexDirection="column" gap={0}>
        <Heading level={3}>Keyboard Shortcuts</Heading>
        <Separator />
        <Text> j/k Move selection up/down</Text>
        <Text> 1/2/3/4 Sort by PID/Name/CPU/Memory</Text>
        <Text> Space Toggle process status</Text>
        <Text> h Toggle this help panel</Text>
        <Text> q Quit</Text>
      </Flex>
    </Box>
  );
}

function CpuSection() {
  const coreLabels = ["Core 0", "Core 1", "Core 2", "Core 3"];
  return (
    <Flex flexDirection="column" gap={0}>
      <Flex flexDirection="row" alignItems="center">
        <Heading level={3}>CPU</Heading>
        <Spacer />
        <Badge variant="info">4 Cores</Badge>
      </Flex>
      <Grid columns={2} gap={1}>
        {state.cpu.map((core, i) => {
          const usage = Math.round(core.user + core.system);
          const variant = usage > 80 ? "error" : usage > 50 ? "warning" : "success";
          return (
            <Flex key={coreLabels[i]} flexDirection="column" gap={0}>
              <Flex flexDirection="row" alignItems="center">
                <Text dimColor>{coreLabels[i]}</Text>
                <Spacer />
                <Badge variant={variant}>{usage}%</Badge>
              </Flex>
              <Progress value={usage} />
              <Flex flexDirection="row" gap={1}>
                <Text dimColor>
                  u:{Math.round(core.user)}% s:{Math.round(core.system)}%
                </Text>
              </Flex>
            </Flex>
          );
        })}
      </Grid>
    </Flex>
  );
}

function MemorySection() {
  const { used, total, cached } = state.memory;
  const usagePercent = Math.round((used / total) * 100);
  const variant = usagePercent > 80 ? "error" : usagePercent > 60 ? "warning" : "success";
  return (
    <Flex flexDirection="column" gap={0}>
      <Flex flexDirection="row" alignItems="center">
        <Heading level={3}>Memory</Heading>
        <Spacer />
        <Badge variant={variant}>{usagePercent}%</Badge>
      </Flex>
      <Progress value={usagePercent} />
      <Flex flexDirection="row" gap={1}>
        <Text dimColor>
          Used: {used.toFixed(1)} GB / {total.toFixed(1)} GB
        </Text>
        <Spacer />
        <Text dimColor>Cached: {cached.toFixed(1)} GB</Text>
      </Flex>
    </Flex>
  );
}

function DiskSection() {
  const { used, total, read, write } = state.disk;
  const usagePercent = Math.round((used / total) * 100);
  const variant = usagePercent > 80 ? "error" : usagePercent > 60 ? "warning" : "success";
  return (
    <Flex flexDirection="column" gap={0}>
      <Flex flexDirection="row" alignItems="center">
        <Heading level={3}>Disk</Heading>
        <Spacer />
        <Badge variant={variant}>{usagePercent}%</Badge>
      </Flex>
      <Progress value={usagePercent} />
      <Flex flexDirection="row" gap={1}>
        <Text dimColor>
          Used: {used} GB / {total} GB
        </Text>
        <Spacer />
        <Text dimColor>
          R: {read.toFixed(1)} MB/s W: {write.toFixed(1)} MB/s
        </Text>
      </Flex>
    </Flex>
  );
}

function NetworkSection() {
  const { in: inRate, out: outRate, connections } = state.network;
  return (
    <Flex flexDirection="column" gap={0}>
      <Flex flexDirection="row" alignItems="center">
        <Heading level={3}>Network</Heading>
        <Spacer />
        <Badge variant="info">{connections} conn</Badge>
      </Flex>
      <Flex flexDirection="row" gap={1}>
        <Text dimColor>In: {inRate.toFixed(2)} MB/s</Text>
        <Spacer />
        <Text dimColor>Out: {outRate.toFixed(2)} MB/s</Text>
      </Flex>
    </Flex>
  );
}

function ProcessTable() {
  const columns = [
    { key: "pid", header: "PID", width: 8 },
    { key: "name", header: "Name", width: 14 },
    { key: "cpu", header: "CPU%", width: 8 },
    { key: "memory", header: "MEM%", width: 8 },
    { key: "status", header: "Status", width: 10 },
    { key: "user", header: "User", width: 10 },
  ];

  const data = state.processes.map((p) => ({
    pid: String(p.pid),
    name: p.name,
    cpu: p.cpu.toFixed(1),
    memory: p.memory.toFixed(1),
    status: p.status,
    user: p.user,
  }));

  return (
    <Flex flexDirection="column" gap={0}>
      <Flex flexDirection="row" alignItems="center">
        <Heading level={3}>Processes</Heading>
        <Spacer />
        <Badge variant="info">{state.processes.length} total</Badge>
        <Spacer />
        <Text dimColor>
          Sort: {state.sortColumn.toUpperCase()} {state.sortAsc ? "asc" : "desc"}
        </Text>
      </Flex>
      <DataTable
        columns={columns}
        data={data}
        selectedId={String(state.processes[state.selectedProcess]?.pid)}
      />
    </Flex>
  );
}

function StatusFooter() {
  const avgCpu = Math.round(
    state.cpu.reduce((sum, c) => sum + c.user + c.system, 0) / state.cpu.length,
  );
  const memPercent = Math.round((state.memory.used / state.memory.total) * 100);
  return (
    <StatusLine
      items={[
        { label: "Uptime", value: formatUptime(state.tick) },
        { label: "Processes", value: String(state.processes.length) },
        { label: "Memory", value: `${memPercent}%` },
        { label: "CPU", value: `${avgCpu}%` },
        { label: "Controls", value: "j/k nav 1-4 sort h help q quit" },
      ]}
    />
  );
}

function SystemMonitor() {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>System Monitor</Heading>
          <Spacer />
          <Text dimColor>Uptime: {formatUptime(state.tick)}</Text>
          <Spacer />
          <Badge variant="success">Live</Badge>
        </Flex>

        <Separator />

        <CpuSection />

        <Flex flexDirection="row" gap={1}>
          <Flex flexDirection="column" flex={1} gap={0}>
            <MemorySection />
          </Flex>
          <Flex flexDirection="column" flex={1} gap={0}>
            <DiskSection />
          </Flex>
          <Flex flexDirection="column" flex={1} gap={0}>
            <NetworkSection />
          </Flex>
        </Flex>

        <Separator />

        <ProcessTable />

        <Separator />

        {state.showHelp && <HelpPanel />}

        <StatusFooter />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  sortProcesses();
  const element = <SystemMonitor />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI System Monitor");
console.log("Press h for help, q to quit");

renderApp();

setInterval(() => {
  updateSimulatedData();
  renderApp();
}, 1000);

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "j") {
    state.selectedProcess = Math.min(state.selectedProcess + 1, state.processes.length - 1);
    renderApp();
  } else if (key === "k") {
    state.selectedProcess = Math.max(state.selectedProcess - 1, 0);
    renderApp();
  } else if (key === "1") {
    if (state.sortColumn === "pid") {
      state.sortAsc = !state.sortAsc;
    } else {
      state.sortColumn = "pid";
      state.sortAsc = true;
    }
    renderApp();
  } else if (key === "2") {
    if (state.sortColumn === "name") {
      state.sortAsc = !state.sortAsc;
    } else {
      state.sortColumn = "name";
      state.sortAsc = true;
    }
    renderApp();
  } else if (key === "3") {
    if (state.sortColumn === "cpu") {
      state.sortAsc = !state.sortAsc;
    } else {
      state.sortColumn = "cpu";
      state.sortAsc = false;
    }
    renderApp();
  } else if (key === "4") {
    if (state.sortColumn === "mem") {
      state.sortAsc = !state.sortAsc;
    } else {
      state.sortColumn = "mem";
      state.sortAsc = false;
    }
    renderApp();
  } else if (key === " ") {
    const proc = state.processes[state.selectedProcess];
    if (proc) {
      proc.status = proc.status === "running" ? "stopped" : "running";
    }
    renderApp();
  } else if (key === "h") {
    state.showHelp = !state.showHelp;
    renderApp();
  } else if (key === "q") {
    process.exit(0);
  }
});
