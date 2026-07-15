// Central example metadata + category taxonomy for the BetterTUI example suite.
// Internal to the examples package; not part of the public framework API.

import type { KeyInput } from "./keyboard";

export type ExampleCategory =
  | "core"
  | "layout"
  | "containers"
  | "navigation"
  | "widgets"
  | "typography"
  | "theming"
  | "animation"
  | "performance"
  | "terminal";

// Display order. Scales to hundreds of examples without reorganising.
export const CATEGORY_ORDER: ExampleCategory[] = [
  "core",
  "layout",
  "containers",
  "navigation",
  "widgets",
  "typography",
  "theming",
  "animation",
  "performance",
  "terminal",
];

export const CATEGORY_LABELS: Record<ExampleCategory, string> = {
  core: "Core",
  layout: "Layout",
  containers: "Containers",
  navigation: "Navigation",
  widgets: "Widgets",
  typography: "Typography",
  theming: "Theming",
  animation: "Animation",
  performance: "Performance",
  terminal: "Terminal",
};

// The progressive learning path surfaced in the README.
export const LEARNING_PATH: ExampleCategory[] = [
  "core",
  "layout",
  "containers",
  "widgets",
  "typography",
  "navigation",
  "theming",
  "animation",
  "performance",
  "terminal",
];

export interface ExampleMeta {
  slug: string;
  title: string;
  description: string;
  category: ExampleCategory;
  level: 1 | 2 | 3 | 4 | 5;
  tags: string[];
  next?: string[];
  requires?: string[];
}

// Used by the launcher to render entries the framework cannot yet demo honestly.
export interface ExampleModule {
  meta: ExampleMeta;
  Example: React.FC;
  run: (keyInput: KeyInput) => void;
  destroy: (keyInput: KeyInput) => void;
}

export interface ExampleEntry {
  meta: ExampleMeta;
  load: () => Promise<ExampleModule>;
}
