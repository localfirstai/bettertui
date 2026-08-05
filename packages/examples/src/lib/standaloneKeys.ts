import { type CliRenderer, type RawKeyEvent, resolveRenderLib } from "@bettertui/core";

export function setupCommonDemoKeys(renderer: CliRenderer) {
  renderer.keyInput.on("keypress", (key: RawKeyEvent) => {
    if ((key.ctrl && key.name === "`") || (key.ctrl && key.name === "f12")) {
      renderer.console.toggle();
    } else if (key.name === "f12" || (key.ctrl && key.shift && key.name === "d")) {
      renderer.toggleDebugOverlay();
    } else if (key.name === "g" && key.ctrl) {
      console.log("dumping hit grid");
      renderer.dumpHitGrid();
    } else if (key.ctrl && key.shift && key.name === "l") {
      renderer.start();
    } else if (key.ctrl && key.shift && key.name === "s") {
      renderer.stop();
    } else if (key.ctrl && key.shift && key.name === "a") {
      renderer.auto();
    } else if (key.name === "a" && key.ctrl) {
      const lib = resolveRenderLib();
      const rawBytes = lib.getArenaAllocatedBytes();
      const formattedBytes = `${(rawBytes / 1024 / 1024).toFixed(2)} MB`;
      console.log("arena allocated bytes:", formattedBytes);
    }
  });
}
