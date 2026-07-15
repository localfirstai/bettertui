import type { JSX, ReactNode } from "react";
import { Box } from "./layout";
import { Text } from "./typography";

export interface MarkdownProps {
  content?: string;
  style?: Record<string, unknown> | undefined;
}

export function Markdown({ content = "", style }: MarkdownProps): JSX.Element {
  const blocks = content.split("\n\n");
  return (
    <Box flexDirection="column" style={style}>
      {blocks.map((b) => {
        const isHeader = b.startsWith("#");
        const clean = b.replace(/^#+\s/, "");
        return (
          <Box key={b} marginBottom={1}>
            <Text bold={isHeader}>{clean}</Text>
          </Box>
        );
      })}
    </Box>
  );
}

export interface DiffProps {
  oldText?: string;
  newText?: string;
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Diff({ oldText = "", newText = "", style }: DiffProps): JSX.Element {
  const oldLines = oldText.split("\n");
  const newLines = newText.split("\n");
  return (
    <Box flexDirection="column" style={style}>
      {oldLines.map((line, i) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: diff line index
        <Text key={`o-${i}`} dim>
          - {line}
        </Text>
      ))}
      {newLines.map((line, i) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: diff line index
        <Text key={`n-${i}`} dim>
          + {line}
        </Text>
      ))}
    </Box>
  );
}
