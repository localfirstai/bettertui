// Component catalogue pattern matching OpenTUI's approach.
// Each component maps a string name to its React component function.
// The reconciler uses this catalogue to resolve JSX intrinsic elements.

import { ChatView, PromptComposer, StatusBar, ThinkingIndicator } from "./chat";
import { Calendar, Chart, Pane, Viewport } from "./container";
import { CodeBlock, Diff, Markdown } from "./content";
import { DataTable, List, Table, Tree } from "./data-display";
import { Badge, Progress, Spinner } from "./feedback";
import {
  Button,
  Checkbox,
  Combobox,
  Input,
  Radio,
  Select,
  Slider,
  Switch,
  Textarea,
} from "./interactive";
import { Box, Flex, Grid, Separator, Spacer, Stack } from "./layout";
import { NerdFont, Slot } from "./native";
import { Accordion, Tabs } from "./navigation";
import { ContextMenu, Dropdown, Modal, Popover, Tooltip } from "./overlay";
import { ScrollArea } from "./scroll";
import { StatusLine, Toast } from "./status";
import { Terminal, TerminalProcess, TerminalViewport } from "./terminal";
import { Blockquote, Code, Heading, Label, Text } from "./typography";

export const componentCatalogue = {
  Box,
  Flex,
  Grid,
  Stack,
  Spacer,
  Separator,
  Text,
  Heading,
  Label,
  Code,
  Blockquote,
  Button,
  Input,
  Textarea,
  Checkbox,
  Radio,
  Switch,
  Slider,
  Select,
  Combobox,
  Tabs,
  Accordion,
  Badge,
  Progress,
  Spinner,
  List,
  Tree,
  Table,
  DataTable,
  Tooltip,
  Modal,
  Popover,
  Dropdown,
  ContextMenu,
  Toast,
  StatusLine,
  Pane,
  Viewport,
  Calendar,
  Chart,
  ScrollArea,
  Markdown,
  CodeBlock,
  Diff,
  PromptComposer,
  ChatView,
  StatusBar,
  ThinkingIndicator,
  Terminal,
  TerminalViewport,
  TerminalProcess,
  Slot,
  NerdFont,
} as const;

export type ComponentCatalogue = typeof componentCatalogue;

// Re-exports for backward compatibility
export * from "./layout";
export * from "./typography";
export * from "./interactive";
export * from "./navigation";
export * from "./feedback";
export * from "./data-display";
export * from "./overlay";
export * from "./status";
export * from "./container";
export * from "./scroll";
export * from "./content";
export * from "./chat";
export * from "./terminal";
export * from "./native";
