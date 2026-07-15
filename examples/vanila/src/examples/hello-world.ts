/**
 * Hello World Example
 * Demonstrates basic rendering with the native engine
 */

import { defineExample } from "../lib/types";
import type { CliRenderer } from "../renderer";

let textNode: number | null = null;
let infoNode: number | null = null;
let hintNode: number | null = null;
let keyHandler: ((key: { name: string; ctrl: boolean }) => void) | null = null;

export const helloWorldExample = defineExample("Core", {
  name: "Hello World",
  slug: "hello-world",
  description: "Basic text rendering with the native engine",
  run(renderer: CliRenderer) {
    renderer.clearTree();
    renderer.clearScreen();

    const rootId = 0;

    textNode = renderer.createNode("text");
    renderer.appendChild(rootId, textNode);
    renderer.setText(textNode, "Hello, BetterTUI!");

    infoNode = renderer.createNode("text");
    renderer.appendChild(rootId, infoNode);
    renderer.setText(infoNode, `Engine v${renderer.version}`);

    hintNode = renderer.createNode("text");
    renderer.appendChild(rootId, hintNode);
    renderer.setText(hintNode, "Press Escape to return to menu");

    renderer.render();

    keyHandler = (key) => {
      if (key.name === "escape" || (key.ctrl && key.name === "c")) {
        renderer.stop();
      }
    };
    renderer.keyHandler.on("keypress", keyHandler);
  },

  destroy(renderer: CliRenderer) {
    if (keyHandler) {
      renderer.keyHandler.off("keypress", keyHandler);
      keyHandler = null;
    }
    textNode = null;
    infoNode = null;
    hintNode = null;
  },
});
