/**
 * Example registry
 */

export { helloWorldExample } from "./hello-world";
export { colorsExample } from "./colors";
export { keyboardExample } from "./keyboard";
export { capabilitiesExample } from "./capabilities";
export { flexLayoutExample } from "./flex-layout";
export { inputDemoExample } from "./input-demo";
export { selectDemoExample } from "./select-demo";
export { performanceExample } from "./performance";

import type { Example, ExampleCategory, ExampleSection } from "../lib/types";
import { CATEGORY_LABELS, CATEGORY_ORDER } from "../lib/types";
import { capabilitiesExample } from "./capabilities";
import { colorsExample } from "./colors";
import { flexLayoutExample } from "./flex-layout";
import { helloWorldExample } from "./hello-world";
import { inputDemoExample } from "./input-demo";
import { keyboardExample } from "./keyboard";
import { performanceExample } from "./performance";
import { selectDemoExample } from "./select-demo";

const ALL_EXAMPLES: Example[] = [
  helloWorldExample,
  colorsExample,
  keyboardExample,
  capabilitiesExample,
  flexLayoutExample,
  inputDemoExample,
  selectDemoExample,
  performanceExample,
];

export function getExamples(): Example[] {
  return ALL_EXAMPLES;
}

export function getExamplesByCategory(): Map<ExampleCategory, Example[]> {
  const map = new Map<ExampleCategory, Example[]>();
  for (const ex of ALL_EXAMPLES) {
    const list = map.get(ex.category) ?? [];
    list.push(ex);
    map.set(ex.category, list);
  }
  return map;
}

export function getExampleSections(): ExampleSection[] {
  const sections: ExampleSection[] = [];
  const byCategory = getExamplesByCategory();

  for (const category of CATEGORY_ORDER) {
    const examples = byCategory.get(category);
    if (examples && examples.length > 0) {
      sections.push({ category, examples });
    }
  }

  return sections;
}

export function findExample(slug: string): Example | undefined {
  return ALL_EXAMPLES.find((ex) => ex.slug === slug);
}

export { CATEGORY_ORDER, CATEGORY_LABELS };
