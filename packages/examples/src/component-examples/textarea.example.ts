import {
  Box,
  type CliRenderer,
  InputEvents,
  RenderableEvents,
  Text,
  Textarea,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let rootContainer: Box | null = null;
let renderer: CliRenderer | null = null;
let editor: Textarea | null = null;
let statsText: Text | null = null;

function countWords(text: string): number {
  return text.trim() === "" ? 0 : text.trim().split(/\s+/).length;
}

function updateStats(): void {
  if (!statsText || !editor) return;
  const val = editor.plainText;
  const chars = val.length;
  const words = countWords(val);
  const lines = val === "" ? 0 : val.split("\n").length;
  statsText.content = t`${bold(fg("#7aa2f7")("Stats:"))} ${fg("#c0caf5")(
    `${chars} chars  ·  ${words} words  ·  ${lines} lines`,
  )}`;
}

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  rootContainer = new Box(renderer, {
    id: "textarea-example-root",
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
      content: t`${bold(fg("#7aa2f7")("Textarea Component Example"))}`,
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

  // Wrapping mode label
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("Multi-line text editor  ·  wrapMode: word")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  // Main editor
  const editorBox = new Box(renderer, {
    flexGrow: 1,
    flexShrink: 1,
    flexBasis: 0,
    minHeight: 0,
    border: true,
    borderStyle: "round",
    borderColor: "#414868",
    focusedBorderColor: "#7aa2f7",
    title: " editor.ts ",
    titleAlignment: "left",
  });

  const initialContent = [
    "// Welcome to the Textarea component example.",
    "// This is a multi-line editable field.",
    "//",
    "// Features:",
    "//   • Full cursor movement (arrows, Home, End, PgUp, PgDn)",
    "//   • Word-wrap mode",
    "//   • Text selection",
    "//   • Read/write mode toggle (press Ctrl+R)",
    "//",
    "// Start typing below:",
    "",
  ].join("\n");

  editor = new Textarea(renderer, {
    id: "main-editor",
    width: "100%",
    height: "100%",
    initialValue: initialContent,
    textColor: "#c0caf5",
    focusedTextColor: "#ffffff",
    cursorColor: "#7aa2f7",
    backgroundColor: "transparent",
    focusedBackgroundColor: "transparent",
    wrapMode: "word",
    showCursor: true,
    selectionBg: "#364A82",
    selectionFg: "#c0caf5",
  });

  editor.on(InputEvents.INPUT, () => updateStats());
  editor.on(RenderableEvents.FOCUSED, () => updateStats());

  editorBox.add(editor);
  rootContainer.add(editorBox);

  // Stats bar
  statsText = new Text(renderer, {
    height: 1,
    flexShrink: 0,
    fg: "#a9b1d6",
  });
  rootContainer.add(statsText);

  // Hint
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#414868")("Ctrl+C quit")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  updateStats();
  editor.focus();
}

export function destroy(rendererInstance: CliRenderer): void {
  if (rootContainer) {
    rendererInstance.root.remove(rootContainer);
    rootContainer.destroy();
    rootContainer = null;
  }
  editor = null;
  statsText = null;
  renderer = null;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
