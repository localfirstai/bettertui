import { CommandBuffer, createReconciler } from "@bettertui/core";
import { Box, Flex, Provider, Text } from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

function App() {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Box padding={1}>
          <Text bold>Hello, BetterTUI!</Text>
        </Box>
        <Flex flexDirection="row" gap={1}>
          <Box padding={1}>
            <Text>Welcome to BetterTUI</Text>
          </Box>
        </Flex>
        <Box padding={1}>
          <Text dimColor>
            A high-performance terminal UI framework powered by Rust and TypeScript
          </Text>
        </Box>
        <Flex flexDirection="row" gap={1}>
          <Box padding={1}>
            <Text color="green">Press q to quit</Text>
          </Box>
        </Flex>
      </Flex>
    </Provider>
  );
}

const element = <App />;
reconciler.createInstance("Provider", { children: element });

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  if (data.toString() === "q") {
    process.exit(0);
  }
});
