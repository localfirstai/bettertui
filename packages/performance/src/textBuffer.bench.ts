/**
 * text-buffer.bench.ts
 *
 * NativeTextEngine insert/delete/undo/redo benchmark.
 *
 * OpenTUI counterpart: `.opencode/references/opentui/packages/core/src/benchmark/text-buffer-render-benchmark.ts`
 * (874 LoC). OpenTUI drives the Zig-side text buffer via FFI; here we route
 * through the napi `NativeTextEngine` wrapper exported from
 * `packages/core/src/platform/binding.ts`.
 *
 * Preconditions: `@bettertui/core` must be built.
 */

import { createTextEngine } from "@bettertui/core";
import { bench, describe } from "vitest";

const SAMPLE_TEXT =
  "The quick brown fox jumps over the lazy dog.\n" +
  "BetterTUI is a high-performance terminal UI framework.\n" +
  "Line three contains unicode: \u00e9\u00e8\u00ea\u00eb \u4e2d\u6587\u5b57\u5e9c \ud83d\ude00\n";

describe("NativeTextEngine — construction", () => {
  bench("create empty", () => {
    const te = createTextEngine();
    te.clear();
  });

  bench("create with 1 KB seed text", () => {
    const seed = SAMPLE_TEXT.repeat(25); // ~1 KB
    const te = createTextEngine(seed);
    if (te.length() === 0) throw new Error("seed failed");
  });
});

describe("NativeTextEngine — sequential inserts", () => {
  bench(
    "insert 1k single chars at cursor",
    () => {
      const te = createTextEngine();
      for (let i = 0; i < 1000; i++) {
        te.insertChar("a");
      }
      if (te.length() !== 1000) throw new Error(`expected 1000 chars, got ${te.length()}`);
    },
    { iterations: 20, time: 1000 },
  );

  bench(
    "insert 100 multi-char strings",
    () => {
      const te = createTextEngine();
      for (let i = 0; i < 100; i++) {
        te.insertStr("hello world ");
      }
    },
    { iterations: 20, time: 1000 },
  );
});

describe("NativeTextEngine — cursor + editing", () => {
  bench(
    "walk cursor across 10k chars + back",
    () => {
      const te = createTextEngine("x".repeat(10_000));
      for (let i = 0; i < 10_000; i++) te.cursorRight();
      for (let i = 0; i < 10_000; i++) te.cursorLeft();
    },
    { iterations: 10, time: 1500 },
  );

  bench(
    "delete 5k chars from front (setCursor + deleteChar)",
    () => {
      const te = createTextEngine("y".repeat(10_000));
      te.setCursorPosition(0);
      for (let i = 0; i < 5000; i++) te.deleteChar();
      if (te.length() !== 5000) throw new Error("delete miscount");
    },
    { iterations: 10, time: 1500 },
  );
});

describe("NativeTextEngine — undo / redo", () => {
  bench(
    "undo 100 single-char inserts",
    () => {
      const te = createTextEngine();
      for (let i = 0; i < 100; i++) te.insertChar("z");
      while (te.canUndo()) te.undo();
    },
    { iterations: 10, time: 1000 },
  );

  bench(
    "undo then redo 100 single-char inserts",
    () => {
      const te = createTextEngine();
      for (let i = 0; i < 100; i++) te.insertChar("z");
      while (te.canUndo()) te.undo();
      while (te.canRedo()) te.redo();
    },
    { iterations: 10, time: 1000 },
  );
});

describe("NativeTextEngine — derived stats", () => {
  bench("lineCount over 10-line buffer", () => {
    const te = createTextEngine(SAMPLE_TEXT.repeat(10));
    const n = te.lineCount();
    if (n < 10) throw new Error("lineCount miscount");
  });

  bench("wordCount over 1 KB buffer", () => {
    const te = createTextEngine(SAMPLE_TEXT.repeat(25));
    const n = te.wordCount();
    if (n === 0) throw new Error("wordCount returned zero");
  });

  bench("getText over 1 KB buffer", () => {
    const te = createTextEngine(SAMPLE_TEXT.repeat(25));
    const s = te.getText();
    if (s.length === 0) throw new Error("getText returned empty");
  });
});
