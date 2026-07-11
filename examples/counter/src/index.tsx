import { Box, Text, Flex, Provider } from "@bettertui/react";
import { CommandBuffer } from "@bettertui/reconciler";
import { createReconciler } from "@bettertui/reconciler";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface AppProps {
  count: number;
}

function App({ count }: AppProps) {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Box padding={1}>
          <Text bold>BetterTUI Counter</Text>
        </Box>
        <Box padding={1}>
          <Text>Count: {count}</Text>
        </Box>
        <Flex flexDirection="row" gap={1}>
          <Box padding={1}>
            <Text>Press + to increment</Text>
          </Box>
          <Box padding={1}>
            <Text>Press - to decrement</Text>
          </Box>
          <Box padding={1}>
            <Text>Press q to quit</Text>
          </Box>
        </Flex>
      </Flex>
    </Provider>
  );
}

function renderApp(count: number): void {
  const element = <App count={count} />;
  reconciler.createInstance("Provider", { children: element });
}

let count = 0;

console.log("BetterTUI Counter Demo");
console.log("Press +/- to increment/decrement, q to quit");

renderApp(count);

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "+") {
    count++;
    renderApp(count);
    console.log(`Count: ${count}`);
  } else if (key === "-") {
    count--;
    renderApp(count);
    console.log(`Count: ${count}`);
  } else if (key === "q") {
    process.exit(0);
  }
});
