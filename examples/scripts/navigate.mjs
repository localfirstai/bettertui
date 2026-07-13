// Example navigator. OpenTUI gives every example package a dev script; BetterTUI
// centralises discovery here. Usage:
//
//   node scripts/navigate.mjs            # list all
//   node scripts/navigate.mjs <slug>     # show run command
//   node scripts/navigate.mjs --search table
//
// Reads the compiled registry. Build first with `pnpm --filter @bettertui/examples build`.

import { CATEGORY_LABELS, CATEGORY_ORDER } from "../dist/index.mjs";
import { exampleBySlug, examples } from "../dist/index.mjs";

function list() {
  for (const cat of CATEGORY_ORDER) {
    const inCat = examples.filter((e) => e.category === cat);
    if (inCat.length === 0) continue;
    console.log(`\n${CATEGORY_LABELS[cat]}`);
    for (const e of inCat) {
      console.log(`  ${e.slug.padEnd(26)} ${e.title}  [L${e.level}]`);
    }
  }
}

function search(term) {
  const t = term.toLowerCase();
  const hits = examples.filter(
    (e) =>
      e.slug.includes(t) ||
      e.title.toLowerCase().includes(t) ||
      e.description.toLowerCase().includes(t) ||
      e.tags.some((tag) => tag.toLowerCase().includes(t)),
  );
  console.log(`\nMatches for "${term}" (${hits.length}):`);
  for (const e of hits) {
    console.log(`  ${e.slug.padEnd(26)} ${e.title}`);
  }
}

const [cmd, value] = process.argv.slice(2);
if (cmd === "--search" && value) {
  search(value);
} else if (cmd && exampleBySlug[cmd]) {
  console.log(`Run: node dist/index.mjs ${cmd}`);
} else {
  console.log(`BetterTUI Examples (${examples.length})`);
  list();
}
