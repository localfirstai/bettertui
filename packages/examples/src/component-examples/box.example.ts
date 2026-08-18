import { Box, type CliRenderer, Text, bold, createCliRenderer, fg, t } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let rootContainer: Box | null = null;
let renderer: CliRenderer | null = null;

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  rootContainer = new Box(renderer, {
    id: "box-example-root",
    flexDirection: "column",
    width: "100%",
    height: "100%",
    padding: 1,
    gap: 1,
  });
  renderer.root.add(rootContainer);

  // ── Section header ──────────────────────────────────────────────────────
  const header = new Text(renderer, {
    content: t`${bold(fg("#7aa2f7")("Box Component Examples"))}`,
    height: 1,
    flexShrink: 0,
  });
  rootContainer.add(header);

  const sep = new Text(renderer, {
    content: "─".repeat(60),
    fg: "#414868",
    height: 1,
    flexShrink: 0,
  });
  rootContainer.add(sep);

  // ── Row: side-by-side boxes ─────────────────────────────────────────────
  const rowSection = new Box(renderer, {
    id: "row-section",
    flexDirection: "column",
    flexShrink: 0,
    gap: 0,
  });

  rowSection.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("1. Row layout with gap")}`,
      height: 1,
    }),
  );

  const rowBox = new Box(renderer, {
    id: "row-box",
    flexDirection: "row",
    gap: 2,
    height: 5,
    flexShrink: 0,
  });

  for (const [label, color] of [
    ["flexGrow: 1", "#9ece6a"],
    ["flexGrow: 2", "#e0af68"],
    ["flexGrow: 1", "#bb9af7"],
  ] as const) {
    const cell = new Box(renderer, {
      flexGrow: label === "flexGrow: 2" ? 2 : 1,
      border: true,
      borderStyle: "single",
      borderColor: color,
      justifyContent: "center",
      alignItems: "center",
    });
    cell.add(new Text(renderer, { content: label, fg: color }));
    rowBox.add(cell);
  }

  rowSection.add(rowBox);
  rootContainer.add(rowSection);

  // ── Center: centered content ────────────────────────────────────────────
  const centerSection = new Box(renderer, {
    id: "center-section",
    flexDirection: "column",
    flexShrink: 0,
    gap: 0,
  });

  centerSection.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("2. Centered content")}`,
      height: 1,
    }),
  );

  const centeredBox = new Box(renderer, {
    id: "centered-box",
    height: 5,
    justifyContent: "center",
    alignItems: "center",
    border: true,
    borderStyle: "round",
    borderColor: "#7aa2f7",
    backgroundColor: "#24283b",
    flexShrink: 0,
  });
  centeredBox.add(
    new Text(renderer, {
      content: t`${bold(fg("#c0caf5")("Centered!"))}`,
    }),
  );

  centerSection.add(centeredBox);
  rootContainer.add(centerSection);

  // ── Border styles ───────────────────────────────────────────────────────
  const borderSection = new Box(renderer, {
    id: "border-section",
    flexDirection: "column",
    flexShrink: 0,
    gap: 0,
  });

  borderSection.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("3. Border styles")}`,
      height: 1,
    }),
  );

  const bordersRow = new Box(renderer, {
    id: "borders-row",
    flexDirection: "row",
    gap: 2,
    height: 4,
    flexShrink: 0,
  });

  for (const style of ["single", "double", "round", "thick", "dashed", "ascii"] as const) {
    const b = new Box(renderer, {
      flexGrow: 1,
      border: true,
      borderStyle: style,
      borderColor: "#565f89",
      justifyContent: "center",
      alignItems: "center",
    });
    b.add(new Text(renderer, { content: style, fg: "#a9b1d6" }));
    bordersRow.add(b);
  }

  borderSection.add(bordersRow);
  rootContainer.add(borderSection);

  // ── Titled box ──────────────────────────────────────────────────────────
  const titledSection = new Box(renderer, {
    id: "titled-section",
    flexDirection: "column",
    flexShrink: 0,
    gap: 0,
  });

  titledSection.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("4. Titled border")}`,
      height: 1,
    }),
  );

  const titledBox = new Box(renderer, {
    id: "titled-box",
    height: 5,
    border: true,
    borderStyle: "round",
    borderColor: "#e0af68",
    title: " Panel Title ",
    titleAlignment: "center",
    bottomTitle: " press q to return ",
    padding: 1,
    flexDirection: "column",
    gap: 1,
    flexShrink: 0,
  });

  titledBox.add(new Text(renderer, { content: "Content inside a titled panel.", fg: "#c0caf5" }));
  titledBox.add(new Text(renderer, { content: "Bottom title shows below.", fg: "#a9b1d6" }));
  titledSection.add(titledBox);
  rootContainer.add(titledSection);
}

export function destroy(rendererInstance: CliRenderer): void {
  if (rootContainer) {
    rendererInstance.root.remove(rootContainer);
    rootContainer.destroy();
    rootContainer = null;
  }
  renderer = null;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
