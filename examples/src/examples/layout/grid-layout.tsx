// Grid layout — fixed-column grids for dashboards and tabular content.
//
// Demonstrates: Grid with columns and gap. Pairs well with Box/Text cells.
// Next: flex-layout, list-view.

import { Box, Flex, Grid, Heading, Provider, Separator, Text } from "@bettertui/react";
import type { KeyInput } from "../../lib/keyboard";
import { useExampleKey } from "../../lib/keyboard-context";
import type { ExampleMeta } from "../../lib/meta";
import { mountExample } from "../../lib/standalone";

export const meta: ExampleMeta = {
  slug: "grid-layout",
  title: "Grid Layout",
  description: "Fixed-column grids for dashboards and tabular content.",
  category: "layout",
  level: 2,
  tags: ["Grid", "columns", "gap"],
  next: ["flex-layout", "list-view"],
};

const tiles = Array.from({ length: 12 }, (_, i) => ({
  id: i,
  label: `Tile ${i + 1}`,
}));

function GridLayout() {
  useExampleKey((event) => {
    if (event.key === "q" || event.key === "Escape") return true;
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1} padding={1}>
        <Heading level={1}>Grid Layout</Heading>
        <Separator />
        <Text dim>4 columns, gap 1</Text>
        <Grid columns={4} gap={1}>
          {tiles.map((t) => (
            <Box key={t.id} padding={1} style={{ border: { fg: "#50c8a0" } }}>
              <Text>{t.label}</Text>
            </Box>
          ))}
        </Grid>
        <Separator />
        <Text dim>Press q to quit</Text>
      </Flex>
    </Provider>
  );
}

export function run(keyInput: KeyInput): void {
  mountExample(GridLayout, keyInput);
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = GridLayout;
