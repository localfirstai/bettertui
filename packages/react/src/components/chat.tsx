import { createElement } from "react";
import type { JSX, ReactNode } from "react";
import { Flex } from "./layout";
import { Text } from "./typography";

export interface PromptComposerProps {
  placeholder?: string;
  value?: string;
  cursorStyle?: "line" | "block" | "underline";
  maxLines?: number;
  history?: string[];
  disabled?: boolean;
  onSubmit?: (value: string) => void;
  onChange?: (value: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function PromptComposer(props: PromptComposerProps): JSX.Element {
  return createElement("PromptComposer", props);
}

export interface ChatMessage {
  role: "user" | "assistant" | "system";
  content: string;
}

export interface ChatViewProps {
  messages?: ChatMessage[];
  messageStyle?: Record<string, unknown>;
  userStyle?: Record<string, unknown>;
  assistantStyle?: Record<string, unknown>;
  systemStyle?: Record<string, unknown>;
  separatorStyle?: Record<string, unknown>;
  style?: Record<string, unknown> | undefined;
}

export function ChatView(props: ChatViewProps): JSX.Element {
  return createElement("ChatView", props);
}

export interface StatusBarProps {
  children?: ReactNode;
  items?: Array<{ label: string; value?: string; style?: Record<string, unknown> }>;
  style?: Record<string, unknown> | undefined;
}

export function StatusBar({ items, children, style }: StatusBarProps): JSX.Element {
  return (
    <Flex flexDirection="row" style={{ bg: "bright_black", fg: "white", ...style }}>
      {children}
      {items?.map((item) => (
        <Flex
          key={item.label}
          flexDirection="row"
          paddingLeft={1}
          paddingRight={1}
          style={item.style}
        >
          <Text bold>{item.label}</Text>
          {item.value && <Text>: {item.value}</Text>}
        </Flex>
      ))}
    </Flex>
  );
}

export interface ThinkingIndicatorProps {
  label?: string;
  style?: Record<string, unknown> | undefined;
}

export function ThinkingIndicator(props: ThinkingIndicatorProps): JSX.Element {
  return createElement("ThinkingIndicator", props);
}
