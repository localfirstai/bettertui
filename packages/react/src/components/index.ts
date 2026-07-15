// Component catalogue pattern matching OpenTUI's approach.
// Each component maps a string name to its React component function.
// The reconciler uses this catalogue to resolve JSX intrinsic elements.

import { Diff, Markdown } from "./content";
import { TextTable } from "./data-display";
import { Input, Select, Slider, Textarea } from "./interactive";
import { Box } from "./layout";
import { TabSelect } from "./navigation";
import { ScrollBar, ScrollBox } from "./scroll";
import { Code, Text } from "./typography";

export const componentCatalogue = {
  Box,
  Text,
  Code,
  Input,
  Textarea,
  Select,
  Slider,
  TabSelect,
  Markdown,
  Diff,
  TextTable,
  ScrollBar,
  ScrollBox,
} as const;

export type ComponentCatalogue = typeof componentCatalogue;

// Re-exports for backward compatibility
export * from "./layout";
export * from "./typography";
export * from "./interactive";
export * from "./navigation";
export * from "./data-display";
export * from "./scroll";
export * from "./content";
