#!/usr/bin/env bun
import {
  Box,
  type CliRenderer,
  type KeyEvent,
  Text,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { ScrollBox } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let overlay: Box | null = null;
let dialog: Box | null = null;
let scrollBox: ScrollBox | null = null;
let baseStatusText: Text | null = null;
let dialogStatusText: Text | null = null;
let keyHandler: ((key: KeyEvent) => void) | null = null;
let dialogOpen = false;
let lastClick = "none";

const updateStatus = () => {
  const content = t`${fg("#9aa5ce")("Last click:")} ${fg("#9ece6a")(lastClick)}`;
  if (baseStatusText) {
    baseStatusText.content = content;
  }
  if (dialogStatusText) {
    dialogStatusText.content = content;
  }
};

const setDialogVisible = (visible: boolean) => {
  dialogOpen = visible;
  if (overlay) {
    overlay.visible = visible;
  }
};

const setLastClick = (value: string) => {
  lastClick = value;
  updateStatus();
};

export function run(renderer: CliRenderer): void {
  renderer.setBackgroundColor("#1a1b26");

  const app = new Box(renderer, {
    id: "app",
    flexDirection: "column",
    width: "100%",
    height: "100%",
    backgroundColor: "#1a1b26",
    paddingLeft: 1,
    paddingTop: 1,
    gap: 1,
  });
  renderer.root.add(app);

  const title = new Text(renderer, {
    content: t`${bold(fg("#7aa2f7")("Scrollbox Overlay Hit Test"))}`,
  });
  app.add(title);

  const instructions = new Text(renderer, {
    content: t`${fg("#c0caf5")("Press 'd' to toggle dialog, 'esc' to close, 'q' to quit")}`,
  });
  app.add(instructions);

  baseStatusText = new Text(renderer, {
    content: t`${fg("#9aa5ce")("Last click:")} ${fg("#9ece6a")(lastClick)}`,
  });
  app.add(baseStatusText);

  overlay = new Box(renderer, {
    id: "overlay",
    position: "absolute",
    top: 0,
    left: 0,
    width: "100%",
    height: "100%",
    backgroundColor: "#ff000033",
    zIndex: 100,
    visible: false,
    onMouseDown: () => {
      setLastClick("overlay (red)");
      setDialogVisible(false);
    },
  });
  renderer.root.add(overlay);

  dialog = new Box(renderer, {
    id: "dialog",
    position: "absolute",
    top: "25%",
    left: "25%",
    width: "50%",
    height: "50%",
    flexDirection: "column",
    gap: 1,
    padding: 1,
    backgroundColor: "#0f172a",
    border: true,
    borderColor: "#7aa2f7",
    onMouseDown: (event) => {
      setLastClick("dialog (blue)");
      (event as { stopPropagation(): void }).stopPropagation();
    },
  });
  overlay.add(dialog);

  const dialogTitle = new Text(renderer, {
    content: t`${bold(fg("#7aa2f7")("Dialog"))} ${fg("#565f89")("- scroll, then click outside the list")}`,
  });
  dialog.add(dialogTitle);

  const dialogHint = new Text(renderer, {
    content: t`${fg("#c0caf5")("Click the red overlay above/below the dialog to close it")}`,
  });
  dialog.add(dialogHint);

  dialogStatusText = new Text(renderer, {
    content: t`${fg("#9aa5ce")("Last click:")} ${fg("#9ece6a")(lastClick)}`,
  });
  dialog.add(dialogStatusText);

  scrollBox = new ScrollBox(renderer, {
    id: "scrollbox",
    flexGrow: 1,
    scrollY: true,
    onMouseDown: (event) => {
      setLastClick("scrollbox (yellow)");
      (event as { stopPropagation(): void }).stopPropagation();
    },
    rootOptions: {
      backgroundColor: "#eab308",
      border: true,
      borderColor: "#0f172a",
    },
    contentOptions: {
      backgroundColor: "#111827",
    },
  });
  dialog.add(scrollBox);

  for (let i = 0; i < 50; i++) {
    const item = new Box(renderer, {
      id: `line-${i}`,
      width: "100%",
      height: 1,
      paddingLeft: 1,
      backgroundColor: i % 2 === 0 ? "#1f2937" : "#111827",
    });
    const text = new Text(renderer, {
      content: t`${fg("#cbd5f5")(`Line ${i + 1}: This is some content`)}`,
    });
    item.add(text);
    scrollBox.add(item);
  }

  keyHandler = (key: KeyEvent) => {
    if (key.name === "q") {
      renderer.destroy();
      process.exit(0);
    }
    if (key.name === "d") {
      setDialogVisible(!dialogOpen);
    }
    if (key.name === "escape") {
      setDialogVisible(false);
    }
  };
  renderer.keyInput.on("keypress", keyHandler);

  updateStatus();
}

export function destroy(renderer: CliRenderer): void {
  if (keyHandler) {
    renderer.keyInput.off("keypress", keyHandler);
  }

  for (const child of renderer.root.getChildren()) {
    renderer.root.remove(child);
    child.destroyRecursively();
  }

  overlay = null;
  dialog = null;
  scrollBox = null;
  baseStatusText = null;
  dialogStatusText = null;
  keyHandler = null;
  dialogOpen = false;
  lastClick = "none";
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
