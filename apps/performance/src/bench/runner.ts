// Benchmark runner entrypoint.
//
// Today this runs the OpenTUI suite (published, installable). The BetterTUI
// suite is BLOCKED until @bettertui/* is published to npm — see
// apps/performance/README.md (PACKAGE BLOCKER).

import { runOpenTuiBenchmarks } from "./frameworks";
import { writeReport } from "./metrics";

async function main() {
  console.log("[bench] OpenTUI vs BetterTUI — published-package benchmark");
  console.log("[bench] OpenTUI: RUNNABLE (installed from npm)");
  console.log("[bench] BetterTUI: BLOCKED (not published to npm)\n");

  const opentui = await runOpenTuiBenchmarks();
  writeReport(opentui, "bench-opentui.json");

  // const bettertui = await runBetterTuiBenchmarks(); // blocked on publish
  // writeReport(bettertui, "bench-bettertui.json");

  console.log(
    "\n[bench] Pair the two result sets for side-by-side comparison at performance.bettertui.com",
  );
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
