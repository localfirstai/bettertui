import { CommandBuffer, createReconciler } from "@bettertui/core";
import { Box, Flex, Grid, Heading, Provider, Separator, Spacer, Text } from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

let showPadding = true;
let gridMode = false;

function LayoutsDemo() {
  const pad = showPadding ? 1 : 0;

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Heading level={2}>Layout Primitives</Heading>
        <Separator />

        <Heading level={3}>Flex Column</Heading>
        <Box padding={pad}>
          <Flex flexDirection="column" gap={1}>
            <Text>Item A</Text>
            <Text>Item B</Text>
            <Text>Item C</Text>
          </Flex>
        </Box>

        <Separator />

        <Heading level={3}>Flex Row</Heading>
        <Box padding={pad}>
          <Flex flexDirection="row" gap={1}>
            <Box padding={1}>
              <Text>Left</Text>
            </Box>
            <Spacer />
            <Box padding={1}>
              <Text>Center</Text>
            </Box>
            <Spacer />
            <Box padding={1}>
              <Text>Right</Text>
            </Box>
          </Flex>
        </Box>

        <Separator />

        <Heading level={3}>{gridMode ? "Grid (3 columns)" : "Grid (2 columns)"}</Heading>
        <Box padding={pad}>
          <Grid columns={gridMode ? 3 : 2} gap={1}>
            <Box padding={1}>
              <Text>Cell 1</Text>
            </Box>
            <Box padding={1}>
              <Text>Cell 2</Text>
            </Box>
            <Box padding={1}>
              <Text>Cell 3</Text>
            </Box>
            <Box padding={1}>
              <Text>Cell 4</Text>
            </Box>
          </Grid>
        </Box>

        <Separator />

        <Heading level={3}>Nested Layout</Heading>
        <Box padding={pad}>
          <Flex flexDirection="column" gap={1}>
            <Flex flexDirection="row" gap={1}>
              <Box padding={1}>
                <Text bold>Header</Text>
              </Box>
              <Spacer />
              <Box padding={1}>
                <Text dimColor>v1.0</Text>
              </Box>
            </Flex>
            <Flex flexDirection="row" gap={1}>
              <Box padding={1}>
                <Text>Sidebar</Text>
              </Box>
              <Box padding={1}>
                <Text>Main Content</Text>
              </Box>
            </Flex>
          </Flex>
        </Box>

        <Separator />

        <Text dimColor>1=toggle padding 2=toggle grid q=quit</Text>
      </Flex>
    </Provider>
  );
}

function renderApp() {
  const element = <LayoutsDemo />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Layouts Demo");
console.log("1=toggle padding 2=toggle grid q=quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "1") {
    showPadding = !showPadding;
    renderApp();
  } else if (key === "2") {
    gridMode = !gridMode;
    renderApp();
  } else if (key === "q") {
    process.exit(0);
  }
});
