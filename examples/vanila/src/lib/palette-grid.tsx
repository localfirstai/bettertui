// Reusable terminal palette grid. Maps OpenTUI's PaletteGrid helper: renders the
// 16/256-colour terminal palette as a labelled grid of swatches. Internal to the
// examples package.

import { Grid, Text } from "@bettertui/react";

export interface PaletteCell {
  index: number;
  hex: string;
}

export function PaletteGrid({
  cells,
  columns = 8,
  color = "#dcdce6",
}: {
  cells: PaletteCell[];
  columns?: number;
  color?: string;
}) {
  return (
    <Grid columns={columns} gap={1}>
      {cells.map((cell) => (
        <Text key={cell.index} style={{ bg: cell.hex }}>
          {` ${String(cell.index).padStart(3, " ")} `}
        </Text>
      ))}
      {cells.length === 0 ? (
        <Text dim color={color}>
          No palette entries.
        </Text>
      ) : null}
    </Grid>
  );
}
