import { highlightCode } from "@bettertui/core";
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
                color={seg.fg || undefined}
                bold={seg.bold ?? undefined}
                italic={seg.italic ?? undefined}
                dim={seg.dim ?? undefined}
                underline={seg.underline ?? undefined}
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
