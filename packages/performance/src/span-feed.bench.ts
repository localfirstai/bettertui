/**
 * span-feed.bench.ts
 *
 * NativeSpanFeed write/drain throughput and backpressure behaviour.
 *
 * OpenTUI counterparts:
 *   - `.opencode/references/opentui/packages/core/src/benchmark/native-span-feed-benchmark.ts`
 *     (596 LoC) — three suites: quick / default / large / all
 *   - `native-span-feed-async-benchmark.ts` — async variant
 *   - `native-span-feed-compare.ts` — cross-version comparison
 *
 * OpenTUI persists `latest-{quick,default,large,async,all}-bench-run.json`.
 * Once BetterTUI numbers are stable we should emit the same JSON shape for
 * direct cross-comparison.
 *
 * Preconditions: `@bettertui/core` must be built.
 */

import { type NapiSpanFeed, type NativeSpanFeedOptions, createSpanFeed } from "@bettertui/core";
import { bench, describe } from "vitest";

const DEFAULT_OPTS: NativeSpanFeedOptions = {};

function makeFeed(opts?: NativeSpanFeedOptions): NapiSpanFeed {
  return createSpanFeed(opts ?? DEFAULT_OPTS);
}

function makePayload(bytes: number): Buffer {
  return Buffer.alloc(bytes, 0x41); // 'A' repeating
}

describe("NativeSpanFeed — write throughput", () => {
  bench(
    "write 100 × 64-byte spans (6.4 KB)",
    () => {
      const feed = makeFeed();
      const payload = makePayload(64);
      for (let i = 0; i < 100; i++) feed.write(payload);
      feed.close();
    },
    { iterations: 20, time: 1000 },
  );

  bench(
    "write 1000 × 64-byte spans (64 KB)",
    () => {
      const feed = makeFeed();
      const payload = makePayload(64);
      for (let i = 0; i < 1000; i++) feed.write(payload);
      feed.close();
    },
    { iterations: 10, time: 1500 },
  );

  bench(
    "write 10 × 64-KB spans (640 KB)",
    () => {
      const feed = makeFeed();
      const payload = makePayload(64 * 1024);
      for (let i = 0; i < 10; i++) feed.write(payload);
      feed.close();
    },
    { iterations: 5, time: 2000 },
  );
});

describe("NativeSpanFeed — drain throughput", () => {
  bench(
    "write 1000 spans, single drain",
    () => {
      const feed = makeFeed();
      const payload = makePayload(64);
      for (let i = 0; i < 1000; i++) feed.write(payload);
      const sink = Buffer.alloc(64 * 1024 + 4096);
      feed.drainSpans(sink);
      feed.close();
    },
    { iterations: 10, time: 1500 },
  );

  bench(
    "interleaved write+drain (100 rounds of 10 writes + drain)",
    () => {
      const feed = makeFeed();
      const payload = makePayload(64);
      const sink = Buffer.alloc(64 * 1024 + 4096);
      for (let r = 0; r < 100; r++) {
        for (let i = 0; i < 10; i++) feed.write(payload);
        feed.drainSpans(sink);
      }
      feed.close();
    },
    { iterations: 10, time: 1500 },
  );
});

describe("NativeSpanFeed — backpressure + stats", () => {
  bench(
    "stats() call cost",
    () => {
      const feed = makeFeed();
      const payload = makePayload(64);
      for (let i = 0; i < 100; i++) feed.write(payload);
      for (let i = 0; i < 1000; i++) feed.stats();
      feed.close();
    },
    { iterations: 5, time: 1000 },
  );

  bench(
    "isBackpressured() poll cost",
    () => {
      const feed = makeFeed();
      for (let i = 0; i < 1000; i++) {
        // isBackpressured is a getter on NapiSpanFeed
        void feed.isBackpressured;
      }
      feed.close();
    },
    { iterations: 5, time: 1000 },
  );
});

describe("NativeSpanFeed — reset / lifecycle", () => {
  bench(
    "write + reset + write again",
    () => {
      const feed = makeFeed();
      const payload = makePayload(64);
      for (let i = 0; i < 100; i++) feed.write(payload);
      feed.reset();
      for (let i = 0; i < 100; i++) feed.write(payload);
      feed.close();
    },
    { iterations: 20, time: 1000 },
  );
});
