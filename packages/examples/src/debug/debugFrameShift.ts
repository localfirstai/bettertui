import fs from "node:fs";
import path from "node:path";
import { createCliRenderer } from "@bettertui/core";
import * as inputSelectLayoutExample from "../examples/inputSelectLayout.example";

const logsDir = path.resolve(process.cwd(), "logs");
if (!fs.existsSync(logsDir)) {
  fs.mkdirSync(logsDir, { recursive: true });
}
const logFile = path.join(logsDir, "debug-frame.log");
fs.writeFileSync(logFile, `CWD: ${process.cwd()}\nLog started\n`, "utf8");

function log(msg: string) {
  fs.appendFileSync(logFile, `${msg}\n`, "utf8");
}

async function main() {
  log("Initializing renderer...");
  const renderer = await createCliRenderer({
    width: 100,
    height: 30,
  });

  for (let i = 1; i <= 3; i++) {
    log(`\n=== ENTER DEMO ${i} ===`);
    inputSelectLayoutExample.run(renderer);
    renderer.render();
    log(`Children of root (run ${i}): ${renderer.getChildrenOf(renderer.rootNodeId).join(", ")}`);
    log(`Tree summary (run ${i}):\n${renderer.engine.treeSummary()}`);

    log(`\n=== RETURN TO MENU ${i} ===`);
    inputSelectLayoutExample.destroy(renderer);
    renderer.render();
    log(
      `Children of root (return ${i}): ${renderer.getChildrenOf(renderer.rootNodeId).join(", ")}`,
    );
  }

  renderer.destroy();
  log("\n=== COMPLETED SUCCESSFULLY ===");
  process.exit(0);
}

main().catch((err) => {
  log(`Error: ${err}`);
  process.exit(1);
});
