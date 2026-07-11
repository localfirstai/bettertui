import { CommandBuffer, createReconciler } from "@bettertui/core";
import { Box, Flex, Provider, Text } from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

function App() {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Box padding={1}>
          <Text bold>Template Example</Text>
        </Box>
        <Box padding={1}>
          <Text>Edit this template to create a new example.</Text>
        </Box>
      </Flex>
    </Provider>
  );
}

function renderApp() {
  const element = <App />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Template");
console.log("Press q to quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();
  if (key === "q") {
    process.exit(0);
  }
});
