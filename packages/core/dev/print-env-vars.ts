#!/usr/bin/env tsx

/**
 * Development script to print all registered BetterTUI environment variables.
 *
 * Usage:
 *   pnpm dev:env             # Colored output (default)
 *   pnpm dev:env --markdown # Markdown output
 *   pnpm dev:env --update   # Update docs/env-vars.md
 */

import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { generateEnvColored, generateEnvMarkdown } from "../src/index";

const args = process.argv.slice(2);
const useMarkdown = args.includes("--markdown");
const updateDocs = args.includes("--update");

const generateMarkdownContent = () => {
  return `# Environment Variables\n\n${generateEnvMarkdown()}---\n\n_generated via packages/core/dev/print-env-vars.ts_\n`;
};

if (updateDocs) {
  const docsPath = join(process.cwd(), "docs/env-vars.md");
  const content = generateMarkdownContent();
  writeFileSync(docsPath, content, "utf8");
  console.log(`✓ Updated ${docsPath}`);
} else if (useMarkdown) {
  console.log(`${generateEnvMarkdown()}\n---\n_generated via packages/core/dev/print-env-vars.ts_`);
} else {
  console.log(generateEnvColored());
}
