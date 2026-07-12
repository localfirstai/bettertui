import { Box, Flex, Provider, Text, render, useKeyboard, useRuntime } from "@bettertui/react";

function App() {
  const runtime = useRuntime();

  useKeyboard((key) => {
    if (key.key === "q") {
      runtime?.dispose();
      process.exit(0);
    }
  });

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
          <Text dim>A high-performance terminal UI framework powered by Rust and TypeScript</Text>
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

const { dispose } = render(<App />);

process.stdin.setRawMode?.(true);
process.stdin.resume();

// Cleanup on exit
process.on("exit", dispose);
process.on("SIGINT", () => {
  dispose();
  process.exit(0);
});
