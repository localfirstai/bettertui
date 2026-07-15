import type { CodeBlockOptions, DiffOptions, MarkdownOptions } from "@bettertui/core";
import { highlightCode } from "@bettertui/core";
import type { JSX } from "react";
import { Box, Flex } from "./layout";
import { Text } from "./typography";

export interface MarkdownProps extends MarkdownOptions {
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

export interface CodeBlockProps extends CodeBlockOptions {
  style?: Record<string, unknown> | undefined;
}

export function CodeBlock({ code, language, showLineNumbers, style }: CodeBlockProps): JSX.Element {
  const content = code || "";
  const lines = language ? highlightCode(content, language) : [];

  if (lines.length > 0) {
    return (
      <Flex flexDirection="column" style={style}>
        {lines.map((segments, lineIdx) => (
          // biome-ignore lint/suspicious/noArrayIndexKey: parser output is static, lines may repeat
          <Flex key={`l-${lineIdx}`} flexDirection="row">
            {showLineNumbers && (
              <Box width={4}>
                <Text dim>{String(lineIdx + 1).padStart(3, " ")} </Text>
              </Box>
            )}
            {segments.map((seg, segIdx) => (
              <Text
                // biome-ignore lint/suspicious/noArrayIndexKey: segments are positional tokens
                key={`s-${segIdx}`}
                color={seg.fg ?? ""}
                bold={seg.bold ?? false}
                italic={seg.italic ?? false}
                dim={seg.dim ?? false}
                underline={seg.underline ?? false}
              >
                {seg.text}
              </Text>
            ))}
          </Flex>
        ))}
      </Flex>
    );
  }

  const plainLines = content.split("\n");
  return (
    <Flex flexDirection="column" style={style}>
      {plainLines.map((line, i) => (
        <Flex key={`${i}-${line}`} flexDirection="row">
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

export interface DiffProps extends DiffOptions {
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
