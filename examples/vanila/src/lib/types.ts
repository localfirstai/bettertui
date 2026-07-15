/**
 * Example types and definitions following OpenTUI's pattern
 */

import type { CliRenderer } from "@bettertui/core";

export type ExampleCategory = "Core" | "Input" | "Layout" | "Widgets" | "Animation" | "Performance";

export interface ExampleDefinition {
  name: string;
  description: string;
  run?: (renderer: CliRenderer) => void | Promise<void>;
  destroy?: (renderer: CliRenderer) => void;
}

export interface Example extends ExampleDefinition {
  category: ExampleCategory;
  slug: string;
}

export interface ExampleSection {
  category: ExampleCategory;
  examples: readonly Example[];
}

export const CATEGORY_LABELS: Record<ExampleCategory, string> = {
  Core: "Core",
  Input: "Input",
  Layout: "Layout",
  Widgets: "Widgets",
  Animation: "Animation",
  Performance: "Performance",
};

export const CATEGORY_ORDER: ExampleCategory[] = [
  "Core",
  "Input",
  "Layout",
  "Widgets",
  "Animation",
  "Performance",
];

export function defineExample(
  category: ExampleCategory,
  definition: ExampleDefinition & { slug: string },
): Example {
  return { ...definition, category };
}
