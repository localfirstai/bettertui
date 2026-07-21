/**
 * widget-mount.bench.ts
 *
 * Mount each of the 13 BetterTUI widgets 1k times. Measures the
 * construction + Command emission cost. Designed to track regressions
 * in the abstract `Renderable` base (`packages/core/src/renderable.ts`)
 * and per-widget option handling.
 *
 * OpenTUI counterpart: each individual widget is benchmarked under
 * `.opencode/references/opentui/packages/core/src/benchmark/markdown-benchmark.ts`
 * (1,796 LoC) and `text-table-benchmark.ts` (948 LoC). BetterTUI widgets are
 * option-type-only today (see `tasks/reports/opentui-gap-analysis.md` W-7);
 * once they grow full implementations this bench should be extended to cover
 * `renderCommands()` output cost per widget.
 *
 * Preconditions: `@bettertui/core` must be built.
 */

import {
  Box,
  Code,
  Diff,
  Input,
  Markdown,
  ScrollBar,
  ScrollBox,
  Select,
  Slider,
  TabSelect,
  Text,
  TextTable,
  Textarea,
} from "@bettertui/core";
import { bench, describe } from "vitest";

function loop(
  Ctor: new (opts?: Record<string, unknown>) => unknown,
  opts: Record<string, unknown>,
  n: number,
): void {
  for (let i = 0; i < n; i++) {
    new Ctor(opts);
  }
}

describe("Widget construction — 1000 mounts per widget", () => {
  // Most BetterTUI widgets take an options object as the sole constructor arg.
  // `Text` and `Code` are special: they take a positional `content` string
  // first, then options — matching their OpenTUI counterparts. The loops
  // below use the correct arity for each.
  bench("Box", () => loop(Box, { border: "single" }, 1000));
  bench("Text", () => {
    for (let i = 0; i < 1000; i++) new Text("hello");
  });
  bench("Code", () => {
    for (let i = 0; i < 1000; i++) new Code("const x = 1", { language: "ts" });
  });
  bench("Diff", () => loop(Diff, { oldText: "a", newText: "b" }, 1000));
  bench("Input", () => loop(Input, { placeholder: "type" }, 1000));
  bench("Textarea", () => loop(Textarea, { placeholder: "write" }, 1000));
  bench("Select", () => loop(Select, { items: ["a", "b", "c"] }, 1000));
  bench("Slider", () => loop(Slider, { min: 0, max: 100, value: 50 }, 1000));
  bench("TabSelect", () => loop(TabSelect, { tabs: ["one", "two"] }, 1000));
  bench("ScrollBar", () => loop(ScrollBar, { total: 100, visible: 10 }, 1000));
  bench("ScrollBox", () => loop(ScrollBox, { height: 10 }, 1000));
  bench("Markdown", () => loop(Markdown, { content: "# hi" }, 1000));
  bench("TextTable", () => loop(TextTable, { rows: [["a", "b"]] }, 1000));
});

describe("Widget mount — scaling", () => {
  bench("Box x100", () => loop(Box, {}, 100));
  bench("Box x1k", () => loop(Box, {}, 1000));
  bench("Box x10k", () => loop(Box, {}, 10_000), { iterations: 5, time: 2000 });
});
