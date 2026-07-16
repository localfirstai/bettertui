import type { JSX } from "react";
import { Box } from "./layout";
import { Text } from "./typography";

export interface TextTableProps {
  headers?: string[];
  rows?: string[][];
  showHeader?: boolean;
  style?: Record<string, unknown> | undefined;
}

export function TextTable({ headers, rows, showHeader, style }: TextTableProps): JSX.Element {
  const data = rows ?? [];
  return (
    <Box flexDirection="column" style={style}>
      {headers && showHeader !== false && (
        <Box flexDirection="row">
          {headers.map((h) => (
            <Box key={h}>
              <Text bold>{h}</Text>
            </Box>
          ))}
        </Box>
      )}
      {data.map((row, rIdx) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: static data table rows
        <Box key={rIdx} flexDirection="row">
          {row.map((cell, cIdx) => (
            // biome-ignore lint/suspicious/noArrayIndexKey: static data table cells
            <Box key={cIdx}>
              <Text>{String(cell)}</Text>
            </Box>
          ))}
        </Box>
      ))}
    </Box>
  );
}
