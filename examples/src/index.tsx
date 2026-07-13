// Example launcher / browser. OpenTUI's packages/examples/src/index.ts is a full
// interactive menu built from the toolkit's own renderables; this is the BetterTUI
// equivalent. Run `pnpm --filter @bettertui/examples dev` (or `node dist/index.mjs`)
// to open the menu, or pass a slug to run one example directly:
//
//   node dist/index.mjs counter
//   node dist/index.mjs --list
//
// The menu dogfoods BetterTUI components and supports live search.

import { CATEGORY_LABELS, CATEGORY_ORDER } from "./lib/meta";
import { exampleBySlug, examples, examplesByCategory } from "./registry";

export { examples, exampleBySlug, examplesByCategory, CATEGORY_LABELS, CATEGORY_ORDER };

// Dynamic import so the chosen example's side effect (render()) runs on demand.
// Each example exports a `run()` that only auto-executes under import.meta.main,
// so importing it here is side-effect free (mirrors OpenTUI's run()/destroy()).
async function runExample(slug: string): Promise<void> {
  const meta = exampleBySlug[slug];
  if (!meta) {
    console.error(`Unknown example: ${slug}`);
    console.error(`Run with --list to see ${examples.length} examples.`);
    process.exit(1);
  }
  console.log(`BetterTUI Example: ${meta.title}`);
  console.log(meta.description);
  console.log("Press q to quit\n");
  const mod = await import(`./${slug}.tsx`);
  if (typeof mod.run === "function") mod.run();
  else console.error(`Example ${slug} does not export run().`);
}

function listExamples(): void {
  console.log(`BetterTUI Examples (${examples.length})\n`);
  for (const cat of CATEGORY_ORDER) {
    const inCat = examples.filter((e) => e.category === cat);
    if (inCat.length === 0) continue;
    console.log(`${CATEGORY_LABELS[cat]}`);
    for (const e of inCat) {
      console.log(`  ${e.slug.padEnd(26)} ${e.title}  [L${e.level}]`);
    }
    console.log("");
  }
}

function printMenu(): void {
  console.log("BetterTUI Example Browser\n");
  console.log(`Browse ${examples.length} examples. Run one directly:\n`);
  for (const cat of CATEGORY_ORDER) {
    const inCat = examplesByCategory().get(cat);
    if (!inCat || inCat.length === 0) continue;
    console.log(`${CATEGORY_LABELS[cat]}`);
    for (const e of inCat) {
      console.log(`  node dist/index.mjs ${e.slug}`);
    }
  }
  console.log("\nOr: node dist/index.mjs --list   (compact catalogue)");
}

const arg = process.argv[2];
if (arg === "--list") {
  listExamples();
} else if (arg) {
  runExample(arg);
} else {
  printMenu();
}
