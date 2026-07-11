import { CommandBuffer, createReconciler } from "@bettertui/core";
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
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface AppProps {
  count: number;
  min: number;
  max: number;
  direction: string;
}

function App({ count, min, max, direction }: AppProps) {
  const variant = count > 0 ? "success" : count < 0 ? "danger" : "default";

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

function renderApp(count: number, min: number, max: number, direction: string) {
  const element = <App count={count} min={min} max={max} direction={direction} />;
  reconciler.createInstance("Provider", { children: element });
}

let count = 0;
let min = 0;
let max = 0;
let direction = "none";

console.log("BetterTUI Counter Demo");
console.log("Press +/- to increment/decrement, r to reset, q to quit");

renderApp(count, min, max, direction);

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "+") {
    count++;
    direction = "up";
    if (count > max) max = count;
    renderApp(count, min, max, direction);
  } else if (key === "-") {
    count--;
    direction = "down";
    if (count < min) min = count;
    renderApp(count, min, max, direction);
  } else if (key === "r") {
    count = 0;
    direction = "none";
    renderApp(count, min, max, direction);
  } else if (key === "q") {
    process.exit(0);
  }
});
