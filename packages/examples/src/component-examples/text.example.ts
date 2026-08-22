import {
  Box,
  type CliRenderer,
  Text,
  type TextChunk,
  TextNode,
  blink,
  bold,
  createCliRenderer,
  dim,
  fg,
  italic,
  strikethrough,
  t,
  underline,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let rootContainer: Box | null = null;
let renderer: CliRenderer | null = null;
let frameCount = 0;
let liveCounterNode: TextNode | null = null;
let frameCallback: ((dt: number) => Promise<void>) | null = null;

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  rootContainer = new Box(renderer, {
    id: "text-example-root",
    flexDirection: "column",
    width: "100%",
    height: "100%",
    padding: 1,
    gap: 1,
  });
  renderer.root.add(rootContainer);

  // ── Header ──────────────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${bold(fg("#7aa2f7")("Text Component Examples"))}`,
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

  // ── 1. Plain text ────────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, { content: t`${fg("#565f89")("1. Plain text")}`, height: 1, flexShrink: 0 }),
  );
  rootContainer.add(
    new Text(renderer, {
      content: "Plain text — no styling applied.",
      fg: "#c0caf5",
      flexShrink: 0,
    }),
  );

  // ── 2. Style helpers ─────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("2. Style helpers")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  const stylesRow = new Box(renderer, {
    flexDirection: "row",
    gap: 2,
    flexShrink: 0,
    flexWrap: "wrap",
  });

  const styleExamples: [string, (s: string) => TextChunk][] = [
    ["bold", bold],
    ["italic", italic],
    ["underline", underline],
    ["dim", dim],
    ["strikethrough", strikethrough],
    ["blink", blink],
  ];

  for (const [label, fn] of styleExamples) {
    stylesRow.add(new Text(renderer, { content: t`${fn(label)}`, fg: "#c0caf5", flexShrink: 0 }));
  }
  rootContainer.add(stylesRow);

  // ── 3. Colors ────────────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, { content: t`${fg("#565f89")("3. Colors")}`, height: 1, flexShrink: 0 }),
  );

  const colorsRow = new Box(renderer, {
    flexDirection: "row",
    gap: 2,
    flexShrink: 0,
    flexWrap: "wrap",
  });

  const colors = [
    ["red", "#f7768e"],
    ["green", "#9ece6a"],
    ["blue", "#7aa2f7"],
    ["yellow", "#e0af68"],
    ["purple", "#bb9af7"],
    ["cyan", "#7dcfff"],
  ] as const;

  for (const [name, hex] of colors) {
    colorsRow.add(new Text(renderer, { content: t`${fg(hex)(bold(name))}`, flexShrink: 0 }));
  }
  rootContainer.add(colorsRow);

  // ── 4. Word-wrap ─────────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("4. Word-wrap (width: 50)")}`,
      height: 1,
      flexShrink: 0,
    }),
  );
  rootContainer.add(
    new Text(renderer, {
      width: 50,
      content:
        "This is a long paragraph that wraps at word boundaries when it exceeds the container width.",
      wrapMode: "word",
      fg: "#a9b1d6",
      flexShrink: 0,
    }),
  );

  // ── 5. Truncation ────────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("5. Truncation (width: 30)")}`,
      height: 1,
      flexShrink: 0,
    }),
  );
  rootContainer.add(
    new Text(renderer, {
      width: 30,
      content: "This text is very long and will be truncated with an ellipsis",
      truncate: true,
      fg: "#a9b1d6",
      flexShrink: 0,
    }),
  );

  // ── 6. TextNode live update ───────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("6. Live TextNode update (frame counter)")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  const liveText = new Text(renderer, { width: 40, flexShrink: 0 });

  liveCounterNode = new TextNode({ fg: "#e0af68", bold: true });
  liveCounterNode.add("0");

  const liveLabel = new TextNode({ fg: "#a9b1d6" });
  liveLabel.add("Frames rendered: ");

  liveText.rootTextNode.add(liveLabel);
  liveText.rootTextNode.add(liveCounterNode);
  rootContainer.add(liveText);

  frameCallback = async () => {
    frameCount++;
    if (liveCounterNode) {
      liveCounterNode.clear();
      liveCounterNode.add(String(frameCount));
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
  liveCounterNode = null;
  frameCount = 0;
  renderer = null;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
