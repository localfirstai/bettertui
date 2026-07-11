import { CommandBuffer, createReconciler } from "@bettertui/core";
import {
  Badge,
  Box,
  Flex,
  Grid,
  Heading,
  Progress,
  Provider,
  Separator,
  Spacer,
  Stack,
  StatusLine,
  Text,
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface StatCardProps {
  label: string;
  value: string;
  color: string;
}

function StatCard({ label, value, color }: StatCardProps) {
  return (
    <Box padding={1}>
      <Flex flexDirection="column" gap={0}>
        <Text dimColor>{label}</Text>
        <Text bold color={color}>
          {value}
        </Text>
      </Flex>
    </Box>
  );
}

interface ActivityItemProps {
  time: string;
  message: string;
  level: "info" | "warn" | "error";
}

function ActivityItem({ time, message, level }: ActivityItemProps) {
  return (
    <Flex flexDirection="row" gap={1}>
      <Text dimColor>{time}</Text>
      <Badge variant={level === "error" ? "error" : level === "warn" ? "warning" : "info"}>
        {level}
      </Badge>
      <Text>{message}</Text>
    </Flex>
  );
}

function Dashboard({ tick }: { tick: number }) {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>System Dashboard</Heading>
          <Spacer />
          <Badge variant="success">Online</Badge>
        </Flex>

        <Separator />

        <Grid columns={3} gap={1}>
          <StatCard label="CPU" value={`${35 + (tick % 20)}%`} color="green" />
          <StatCard label="Memory" value="4.2 GB" color="yellow" />
          <StatCard label="Disk" value="128 GB" color="blue" />
        </Grid>

        <Grid columns={3} gap={1}>
          <StatCard label="Network" value="1.2 Gb/s" color="cyan" />
          <StatCard label="Uptime" value={`${tick}s`} color="magenta" />
          <StatCard label="Errors" value={`${tick % 5}`} color="red" />
        </Grid>

        <Separator />

        <Heading level={3}>Recent Activity</Heading>
        <Stack gap={0}>
          <ActivityItem time="12:01" message="Deployment completed" level="info" />
          <ActivityItem time="11:45" message="High memory usage detected" level="warn" />
          <ActivityItem time="11:30" message="Connection timeout to db-02" level="error" />
          <ActivityItem time="11:15" message="Service restarted" level="info" />
        </Stack>

        <Separator />

        <Flex flexDirection="column" gap={0}>
          <Text dimColor>Disk Usage</Text>
          <Progress value={64} width={40} />
        </Flex>

        <Separator />

        <StatusLine
          items={[
            { label: "Dashboard", value: "v1.0" },
            { label: "Tick", value: `${tick}` },
            { label: "Controls", value: "r=refresh q=quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

let tick = 0;

function renderApp() {
  const element = <Dashboard tick={tick} />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Dashboard Showcase");
console.log("r=refresh q=quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "r") {
    tick++;
    renderApp();
  } else if (key === "q") {
    process.exit(0);
  }
});
