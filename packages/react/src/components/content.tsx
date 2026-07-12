import type { JSX } from "react";
import { Box, Flex } from "./layout";
import { Text } from "./typography";

export interface MarkdownProps {
  content?: string;
  indent?: number;
  headingStyle?: Record<string, unknown>;
  boldStyle?: Record<string, unknown>;
  italicStyle?: Record<string, unknown>;
  codeStyle?: Record<string, unknown>;
  codeBlockStyle?: Record<string, unknown>;
  linkStyle?: Record<string, unknown>;
  quoteStyle?: Record<string, unknown>;
  ruleStyle?: Record<string, unknown>;
  style?: Record<string, unknown> | undefined;
}

export function Markdown({ content = "", indent = 0, style }: MarkdownProps): JSX.Element {
  const blocks = content.split("\n\n");
  return (
    <Flex flexDirection="column" style={style}>
      {blocks.map((b) => {
        const isHeader = b.startsWith("#");
        const clean = b.replace(/^#+\s/, "");
        return (
          <Box key={b} paddingLeft={indent} marginBottom={1}>
            <Text bold={isHeader}>{clean}</Text>
          </Box>
        );
      })}
    </Flex>
  );
}

export interface CodeBlockProps {
  code?: string;
  language?: string;
  showLineNumbers?: boolean;
  style?: Record<string, unknown> | undefined;
}

export function CodeBlock({ code, showLineNumbers, style }: CodeBlockProps): JSX.Element {
  const lines = (code || "").split("\n");
  return (
    <Flex flexDirection="column" style={style}>
      {lines.map((line) => (
        <Flex key={line} flexDirection="row">
          {showLineNumbers && (
            <Box width={4}>
              <Text dim>{String(i + 1).padStart(3, " ")} </Text>
            </Box>
          )}
          <Text>{line}</Text>
        </Flex>
      ))}
    </Flex>
  );
}

export interface DiffProps {
  oldText?: string;
  newText?: string;
  unified?: boolean;
  style?: Record<string, unknown> | undefined;
}

export function Diff({ oldText = "", newText = "", style }: DiffProps): JSX.Element {
  const oldLines = oldText.split("\n");
  const newLines = newText.split("\n");
  return (
    <Flex flexDirection="column" style={style}>
      {oldLines.map((line) => (
        <Text key={`o-${line}`} style={{ fg: "red" }}>
          - {line}
        </Text>
      ))}
      {newLines.map((line) => (
        <Text key={`n-${line}`} style={{ fg: "green" }}>
          + {line}
        </Text>
      ))}
    </Flex>
  );
}
