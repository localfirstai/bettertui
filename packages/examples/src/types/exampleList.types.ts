import type { CliRenderer, SelectOption } from "@bettertui/core";

// Re-export theme types
export type {
  AppTheme,
  ComponentTheme,
  ThemeMode,
  ThemeTokens,
} from "../constants/theme.js";

export type ExampleCategory =
  | "Layout & Composition"
  | "Input & Editing"
  | "Scroll & Navigation"
  | "Text & Documents"
  | "Rendering & Effects"
  | "Runtime & Tooling"
  | "Terminal & Native";

export interface ExampleDefinition {
  name: string;
  description: string;
  run?: (renderer: CliRenderer) => void | Promise<void>;
  destroy?: (renderer: CliRenderer) => void;
  unavailableMessage?: string;
}

export interface Example extends ExampleDefinition {
  category: ExampleCategory;
}

export interface ExampleSection {
  category: ExampleCategory;
  examples: readonly ExampleDefinition[];
}

export interface CategoryMenuValue {
  kind: "category";
  category: ExampleCategory;
}

export interface SpacerMenuValue {
  kind: "spacer";
}

export interface MessageMenuValue {
  kind: "message";
}

export interface ExampleMenuValue {
  kind: "example";
  example: Example;
}

export type MenuOptionValue =
  | CategoryMenuValue
  | SpacerMenuValue
  | MessageMenuValue
  | ExampleMenuValue;

export type MenuOption = Omit<SelectOption, "value"> & {
  value: MenuOptionValue;
};

export type MenuFocusArea = "filter" | "list";

// ExampleTheme is now just ComponentTheme for backward compatibility
export type ExampleTheme = import("../constants/theme.js").ComponentTheme;
