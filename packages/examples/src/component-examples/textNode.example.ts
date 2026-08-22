import {
  Box,
  type CliRenderer,
  Text,
  TextNode,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let rootContainer: Box | null = null;
let renderer: CliRenderer | null = null;
let frameCallback: ((dt: number) => Promise<void>) | null = null;
let liveNode: TextNode | null = null;
let tickCount = 0;

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  rootContainer = new Box(renderer, {
    id: "textnode-example-root",
    flexDirection: "column",
    width: "100%",
    height: "100%",
    padding: 1,
    gap: 1,
  });
  renderer.root.add(rootContainer);

  // Header
  rootContainer.add(
    new Text(renderer, {
      content: t`${bold(fg("#7aa2f7")("TextNode Component Example"))}`,
      height: 1,
      flexShrink: 0,
    }),
  );
  rootContainer.add(
    new Text(renderer, {
      content: "─".repeat(70),
      fg: "#414868",
      height: 1,
      flexShrink: 0,
    }),
  );

  // ── 1. Basic node composition ─────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("1. Basic composition")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  const compText = new Text(renderer, { width: 70, flexShrink: 0 });
  const label = new TextNode({ fg: "#a9b1d6" });
  label.add("Status: ");
  const statusNode = new TextNode({ fg: "#9ece6a", bold: true });
  statusNode.add("OK");
  compText.rootTextNode.add(label);
  compText.rootTextNode.add(statusNode);
  rootContainer.add(compText);

  // ── 2. Nested styles ──────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("2. Nested style inheritance")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  const nestText = new Text(renderer, { width: 70, flexShrink: 0 });
  const outerNode = new TextNode({ fg: "#c0caf5" });
  outerNode.add("Plain → ");

  const boldNode = new TextNode({ bold: true, fg: "#7aa2f7" });
  boldNode.add("Bold Blue");

  const italicNode = new TextNode({ italic: true, fg: "#e0af68" });
  italicNode.add(" → Italic Yellow");

  const ulNode = new TextNode({ underline: true, fg: "#bb9af7" });
  ulNode.add(" → Underline Purple");

  outerNode.add(boldNode);
  outerNode.add(italicNode);
  outerNode.add(ulNode);
  nestText.rootTextNode.add(outerNode);
  rootContainer.add(nestText);

  // ── 3. All attributes ─────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("3. All text attributes")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  const attrText = new Text(renderer, { width: 70, flexShrink: 0 });
  const attrDefs: Array<[string, TextNode]> = [
    ["bold", new TextNode({ bold: true, fg: "#c0caf5" })],
    ["italic", new TextNode({ italic: true, fg: "#c0caf5" })],
    ["underline", new TextNode({ underline: true, fg: "#c0caf5" })],
    ["dim", new TextNode({ dim: true, fg: "#c0caf5" })],
    ["strikethrough", new TextNode({ strikethrough: true, fg: "#c0caf5" })],
  ];

  for (const [name, node] of attrDefs) {
    node.add(name);
    attrText.rootTextNode.add(node);
    attrText.rootTextNode.add("  ");
  }
  rootContainer.add(attrText);

  // ── 4. Dynamic live update ────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("4. Live update (mutate children each frame)")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  const liveText = new Text(renderer, { width: 70, flexShrink: 0 });
  const liveLabel = new TextNode({ fg: "#a9b1d6" });
  liveLabel.add("Tick: ");

  liveNode = new TextNode({ fg: "#e0af68", bold: true });
  liveNode.add("0");

  const liveSuffix = new TextNode({ fg: "#565f89" });
  liveSuffix.add("  (updates every frame)");

  liveText.rootTextNode.add(liveLabel);
  liveText.rootTextNode.add(liveNode);
  liveText.rootTextNode.add(liveSuffix);
  rootContainer.add(liveText);

  // ── 5. fromNodes factory ──────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("5. TextNode.fromNodes()")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  const factoryText = new Text(renderer, { width: 70, flexShrink: 0 });

  const fn1 = new TextNode({ bold: true, fg: "#7aa2f7" });
  fn1.add("BetterTUI");
  const fn2 = TextNode.fromString(" — ");
  const fn3 = new TextNode({ italic: true, fg: "#9ece6a" });
  fn3.add("fast");
  const fn4 = TextNode.fromString(", ");
  const fn5 = new TextNode({ underline: true, fg: "#bb9af7" });
  fn5.add("typed");
  const fn6 = TextNode.fromString(", composable terminal UI");

  const composed = TextNode.fromNodes([fn1, fn2, fn3, fn4, fn5, fn6], { fg: "#c0caf5" });
  factoryText.rootTextNode.add(composed);
  rootContainer.add(factoryText);

  // Frame callback for live update
  frameCallback = async () => {
    tickCount++;
    if (liveNode) {
      liveNode.clear();
      liveNode.add(String(tickCount));
    }
  };
  rendererInstance.on("frame", frameCallback);
}

export function destroy(rendererInstance: CliRenderer): void {
  if (frameCallback) {
    rendererInstance.off("frame", frameCallback);
    frameCallback = null;
  }
  if (rootContainer) {
    rendererInstance.root.remove(rootContainer);
    rootContainer.destroy();
    rootContainer = null;
  }
  liveNode = null;
  tickCount = 0;
  renderer = null;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
