import {
  Badge,
  Box,
  Flex,
  Heading,
  Provider,
  Separator,
  StatusLine,
  Text,
  render,
  useKeyboard,
  useRuntime,
} from "@bettertui/react";

interface KeyEvent {
  key: string;
  raw: string;
  timestamp: number;
}

const keyHistory: KeyEvent[] = [];
const maxHistory = 10;

function formatKey(data: string): string {
  if (data === "\x1b[A") return "↑";
  if (data === "\x1b[B") return "↓";
  if (data === "\x1b[C") return "→";
  if (data === "\x1b[D") return "←";
  if (data === "\t") return "Tab";
  if (data === "\r") return "Enter";
  if (data === "\x03") return "Ctrl+C";
  return data;
}

function TerminalShowcase() {
  const runtime = useRuntime();

  useKeyboard((key) => {
    if (key.key === "q") {
      runtime?.dispose();
      process.exit(0);
    }

    keyHistory.unshift({
      key: formatKey(key.key),
      raw: JSON.stringify(key.key),
      timestamp: Date.now(),
    });

    if (keyHistory.length > maxHistory) {
      keyHistory.pop();
    }

    renderApp();
    return true;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Terminal Showcase</Heading>
          <Badge variant="info">{keyHistory.length} keys</Badge>
        </Flex>

        <Separator />

        <Heading level={3}>Key History</Heading>
        <Box padding={1}>
          {keyHistory.length === 0 ? (
            <Text dimColor>Press any key to see it displayed here.</Text>
          ) : (
            <Flex flexDirection="column" gap={0}>
              {keyHistory.map((event, i) => (
                <Flex key={`${event.key}-${i}`} flexDirection="row" gap={1}>
                  <Badge variant="info">{i + 1}</Badge>
                  <Text bold>{event.key}</Text>
                  <Text dimColor>({event.raw})</Text>
                </Flex>
              ))}
            </Flex>
          )}
        </Box>

        <Separator />

        <Heading level={3}>Supported Keys</Heading>
        <Flex flexDirection="column" gap={0}>
          <Text>↑ ↓ ← → — Arrow keys</Text>
          <Text>Tab — Tab key</Text>
          <Text>Any printable character</Text>
        </Flex>

        <Separator />

        <Text dimColor>Press any key, q=quit</Text>

        <StatusLine
          items={[
            { label: "History", value: `${keyHistory.length}/${maxHistory}` },
            { label: "Terminal", value: process.platform },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  render(<TerminalShowcase />);
}

console.log("BetterTUI Terminal Showcase");
console.log("Press any key to see it displayed, q=quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();

process.on("SIGINT", () => {
  process.exit(0);
});
