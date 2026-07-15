import type {
  ChatViewOptions,
  ChatMessage as CoreChatMessage,
  PromptComposerOptions,
  StatusBarOptions,
  ThinkingIndicatorOptions,
} from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";
import { Flex } from "./layout";
import { Text } from "./typography";

export type ChatMessage = CoreChatMessage;

export interface PromptComposerProps extends PromptComposerOptions {
  style?: Record<string, unknown> | undefined;
}

export function PromptComposer(props: PromptComposerProps): JSX.Element {
  return createElement("PromptComposer", props);
}

export interface ChatViewProps extends ChatViewOptions {
  style?: Record<string, unknown> | undefined;
}

export function ChatView(props: ChatViewProps): JSX.Element {
  return createElement("ChatView", props);
}

export interface StatusBarProps extends StatusBarOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function StatusBar({ items, children, style }: StatusBarProps): JSX.Element {
  return (
    <Flex flexDirection="row" style={{ bg: "bright_black", fg: "white", ...style }}>
      {children}
      {items?.map((item) => (
        <Flex key={item.label} flexDirection="row" paddingLeft={1} paddingRight={1}>
          <Text bold>{item.label}</Text>
          {item.value && <Text>: {item.value}</Text>}
        </Flex>
      ))}
    </Flex>
  );
}

export interface ThinkingIndicatorProps extends ThinkingIndicatorOptions {
  style?: Record<string, unknown> | undefined;
}

export function ThinkingIndicator(props: ThinkingIndicatorProps): JSX.Element {
  return createElement("ThinkingIndicator", props);
}
