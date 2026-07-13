// Generates per-example README.md files from each example's `meta` export.
// Run after building: node scripts/generate-docs.mjs
// Keeps documentation next to code and always matching the committed source.

import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { CATEGORY_LABELS, exampleBySlug, examples } from "../dist/index.mjs";

const here = dirname(fileURLToPath(import.meta.url));
const docsDir = resolve(here, "../docs");

mkdirSync(docsDir, { recursive: true });

function related(slug) {
  const e = exampleBySlug[slug];
  if (!e || !e.next || e.next.length === 0) return "_None specified._";
  return e.next
    .map((n) => {
      const r = exampleBySlug[n];
      return `- \`${n}\`${r ? ` — ${r.title}` : ""}`;
    })
    .join("\n");
}

for (const e of examples) {
  const category = CATEGORY_LABELS[e.category] ?? e.category;
  const caps = e.requires?.length ? e.requires.map((c) => `\`${c}\``).join(", ") : "_None._";
  const md = `# ${e.title}

> ${e.description}

- **Category:** ${category}
- **Level:** ${e.level} / 5
- **Demonstrates:** ${e.tags.join(", ")}
- **Requires:** ${caps}

## What it shows

This example focuses on **${e.tags[0] ?? e.title}**. Read the source in
\`src/examples/${e.category}/${e.slug}.tsx\` — each example is small, self-contained, and easy to read.

## Run it

\`\`\`bash
pnpm --filter @bettertui/examples build
node dist/index.mjs ${e.slug}
\`\`\`

Or from the interactive browser:

\`\`\`bash
pnpm --filter @bettertui/examples dev
\`\`\`

## Key APIs

${e.tags.map((t) => `- \`${t}\``).join("\n")}

## Next examples

${related(e.slug)}
`;
  writeFileSync(resolve(docsDir, `${e.slug}.md`), md);
}

console.log(`Generated ${examples.length} example READMEs in docs/`);
