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
  render,
  useKeyboard,
  useRuntime,
} from "@bettertui/react";

interface AppProps {
  count: number;
  min: number;
  max: number;
  direction: string;
}

function App({ count, min, max, direction }: AppProps) {
  const runtime = useRuntime();
  const variant = count > 0 ? "success" : count < 0 ? "danger" : "default";

  useKeyboard((key) => {
    if (key.key === "+") {
      count++;
      direction = "up";
      if (count > max) max = count;
      renderApp(count, min, max, direction);
    } else if (key.key === "-") {
      count--;
      direction = "down";
      if (count < min) min = count;
      renderApp(count, min, max, direction);
    } else if (key.key === "r") {
      count = 0;
      direction = "none";
      renderApp(count, min, max, direction);
    } else if (key.key === "q") {
      runtime?.dispose();
      process.exit(0);
    }
    return true;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Box padding={1}>
          <Heading level={1}>Counter</Heading>
        </Box>

        <Separator />

        <Flex flexDirection="column" gap={1} padding={1}>
          <Flex alignItems="center" gap={1}>
            <Text bold>Count:</Text>
            <Badge variant={variant}>{count}</Badge>
            <Spacer size={1} />
            <Text dim>{direction !== "none" ? `(${direction})` : ""}</Text>
          </Flex>
        </Flex>

        <Separator />

        <Flex flexDirection="row" gap={1} padding={1}>
          <Text>[-] Decrement</Text>
          <Text>[+] Increment</Text>
          <Text>[r] Reset</Text>
          <Text>[q] Quit</Text>
        </Flex>

        <Separator />

        <StatusLine
          items={[
            { label: "Count", value: String(count) },
            { label: "Min", value: String(min) },
            { label: "Max", value: String(max) },
            { separator: true },
            { label: "+/-", value: "adjust" },
            { label: "r", value: "reset" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

let count = 0;
let min = 0;
let max = 0;
let direction = "none";

function renderApp(newCount: number, newMin: number, newMax: number, newDirection: string) {
  count = newCount;
  min = newMin;
  max = newMax;
  direction = newDirection;
  render(<App count={count} min={min} max={max} direction={direction} />);
}

console.log("BetterTUI Counter Demo");
console.log("Press +/- to increment/decrement, r to reset, q to quit");

renderApp(count, min, max, direction);

process.stdin.setRawMode?.(true);
process.stdin.resume();

process.on("SIGINT", () => {
  process.exit(0);
});
