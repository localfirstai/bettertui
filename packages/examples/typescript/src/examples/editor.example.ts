import {
  BoxRenderable,
  type CliRenderer,
  LineNumberRenderable,
  type RawKeyEvent,
  TextRenderable,
  TextareaRenderable,
  createCliRenderer,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

const initialContent = `Welcome to the TextareaRenderable Demo!

This is an interactive text editor powered by EditBuffer and EditorView.

\tThis is a tab
\t\t\tMultiple tabs

Emojis:
👩🏽‍💻  👨‍👩‍👧‍👦  🏳️‍🌈  🇺🇸  🇩🇪  🇯🇵  🇮🇳

NAVIGATION:
  • Arrow keys to move cursor
  • Ctrl+A/Ctrl+E for line start/end
  • Home/End for buffer start/end
  • Ctrl+F/Ctrl+B to move right/left (Emacs-style)
  • Alt+F/Alt+B for word forward/backward
  • Alt+Left/Alt+Right for word forward/backward
  • Ctrl+Left/Ctrl+Right for word forward/backward
  • Alt+A/Alt+E for visual line start/end

SELECTION:
  • Shift+Arrow keys to select
  • Ctrl+Shift+A/E to select to line start/end
  • Shift+Home/End to select to buffer start/end
  • Alt+Shift+F/B to select word forward/backward
  • Alt+Shift+Left/Right to select word forward/backward
  • Alt+Shift+A/E to select to visual line start/end

EDITING:
  • Type any text to insert
  • Backspace/Delete to remove text
  • Enter to create new lines
  • Ctrl+Shift+D to delete current line
  • Ctrl+D to delete character forward
  • Ctrl+K to delete to line end
  • Ctrl+U to delete to line start
  • Alt+D to delete word forward
  • Alt+Backspace or Ctrl+W to delete word backward
  • Ctrl+Delete or Alt+Delete to delete word forward

UNDO/REDO:
  • Ctrl+- to undo or Cmd+Z (Mac)
  • Ctrl+. to redo or Cmd+Shift+Z (Mac)

VIEW:
  • Shift+W to toggle wrap mode (word/char/none)
  • Shift+L to toggle line numbers
  • Shift+H to toggle diff highlights (colors + +/- signs)
  • Shift+D to toggle diagnostics (error/warning/info emojis)
  • Ctrl+] to increase scroll speed
  • Ctrl+[ to decrease scroll speed

FEATURES:
  ✓ Grapheme-aware cursor movement
  ✓ Unicode (emoji 🌟 and CJK 世界, 你好世界, 中文, 한글)
  ✓ Incremental editing
  ✓ Text wrapping and viewport management
  ✓ Undo/redo support
  ✓ Word-based navigation and deletion
  ✓ Text selection with shift keys

Press ESC to return to main menu`;

let renderer: CliRenderer | null = null;
let parentContainer: BoxRenderable | null = null;
let editor: TextareaRenderable | null = null;
let editorWithLines: LineNumberRenderable | null = null;
let statusText: TextRenderable | null = null;
let highlightsEnabled = false;
let diagnosticsEnabled = false;
let localWrapMode: "word" | "char" | "none" = "word";
let localScrollSpeed = 4;
let localShowLineNumbers = true;

export async function run(rendererInstance: CliRenderer): Promise<void> {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#0D1117");

  parentContainer = new BoxRenderable(renderer, {
    id: "parent-container",
    zIndex: 10,
    padding: 1,
  });
  renderer.root.add(parentContainer);

  const editorBox = new BoxRenderable(renderer, {
    id: "editor-box",
    borderStyle: "single",
    borderColor: "#6BCF7F",
    backgroundColor: "#0D1117",
    title: "Interactive Editor (TextareaRenderable)",
    titleAlignment: "left",
    border: true,
  });
  parentContainer.add(editorBox);

  // Create interactive editor
  editor = new TextareaRenderable(renderer, {
    id: "editor",
    initialValue: initialContent,
    textColor: "#F0F6FC",
    wrapMode: "word",
    showCursor: true,
    cursorColor: "#4ECDC4",
    placeholder: "Enter text here...",
  });

  editorWithLines = new LineNumberRenderable(renderer, {
    id: "editor-lines",
    target: editor,
    minWidth: 3,
    paddingRight: 1,
    width: "100%",
    height: "100%",
  });
  editorWithLines.fg = "#6b7280"; // Dimmed gray for line numbers
  editorWithLines.bg = "#161b22"; // Slightly darker than editor background

  editorBox.add(editorWithLines);

  statusText = new TextRenderable(renderer, {
    id: "status",
    content: "",
    fg: "#A5D6FF",
    height: 1,
  });
  parentContainer.add(statusText);

  editor.focus();

  rendererInstance.setFrameCallback(async () => {
    if (statusText && editor && !editor.isDestroyed) {
      try {
        const wrap = localWrapMode !== "none" ? "ON" : "OFF";
        const highlights = highlightsEnabled ? "ON" : "OFF";
        const diagnostics = diagnosticsEnabled ? "ON" : "OFF";
        const scrollSpeed = localScrollSpeed;
        statusText.content = `Wrap: ${wrap} | Diff: ${highlights} | Diag: ${diagnostics} | Scroll: ${scrollSpeed} lines/s`;
      } catch {
        // Ignore errors during shutdown
      }
    }
  });

  rendererInstance.keyInput.on("keypress", (key: RawKeyEvent) => {
    if (key.shift && key.name === "l") {
      key.preventDefault();
      if (editorWithLines && !editorWithLines.isDestroyed) {
        localShowLineNumbers = !localShowLineNumbers;
        editorWithLines.visible = localShowLineNumbers;
      }
    }
    if (key.shift && key.name === "w") {
      key.preventDefault();
      localWrapMode =
        localWrapMode === "word" ? "char" : localWrapMode === "char" ? "none" : "word";
    }
    if (key.shift && key.name === "h") {
      key.preventDefault();
      if (editorWithLines && !editorWithLines.isDestroyed) {
        highlightsEnabled = !highlightsEnabled;
        if (highlightsEnabled) {
          // Add diff-style line colors and signs throughout the document
          editorWithLines.setLineColor(2, "#1a4d1a");
          editorWithLines.setLineSign(2, "+", "#22c55e");
          editorWithLines.setLineColor(5, "#4d1a1a");
          editorWithLines.setLineSign(5, "-", "#ef4444");
          editorWithLines.setLineColor(8, "#1a4d1a");
          editorWithLines.setLineSign(8, "+", "#22c55e");
          editorWithLines.setLineColor(11, "#4d1a1a");
          editorWithLines.setLineSign(11, "-", "#ef4444");
          editorWithLines.setLineColor(14, "#1a4d1a");
          editorWithLines.setLineSign(14, "+", "#22c55e");
          editorWithLines.setLineColor(17, "#4d1a1a");
          editorWithLines.setLineSign(17, "-", "#ef4444");
          editorWithLines.setLineColor(20, "#1a4d1a");
          editorWithLines.setLineSign(20, "+", "#22c55e");
          editorWithLines.setLineColor(23, "#4d1a1a");
          editorWithLines.setLineSign(23, "-", "#ef4444");
          editorWithLines.setLineColor(27, "#1a4d1a");
          editorWithLines.setLineSign(27, "+", "#22c55e");
          editorWithLines.setLineColor(30, "#4d1a1a");
          editorWithLines.setLineSign(30, "-", "#ef4444");
          editorWithLines.setLineColor(34, "#1a4d1a");
          editorWithLines.setLineSign(34, "+", "#22c55e");
          editorWithLines.setLineColor(38, "#4d1a1a");
          editorWithLines.setLineSign(38, "-", "#ef4444");
          editorWithLines.setLineColor(42, "#1a4d1a");
          editorWithLines.setLineSign(42, "+", "#22c55e");
          editorWithLines.setLineColor(46, "#4d1a1a");
          editorWithLines.setLineSign(46, "-", "#ef4444");
          editorWithLines.setLineColor(50, "#1a4d1a");
          editorWithLines.setLineSign(50, "+", "#22c55e");
          editorWithLines.setLineColor(54, "#4d1a1a");
          editorWithLines.setLineSign(54, "-", "#ef4444");
          editorWithLines.setLineColor(58, "#1a4d1a");
          editorWithLines.setLineSign(58, "+", "#22c55e");
        } else {
          editorWithLines.clearAllLineColors();
          // Clear diff signs (simplified - clear known diff lines)
          for (const line of [2, 5, 8, 11, 14, 17, 20, 23, 27, 30, 34, 38, 42, 46, 50, 54, 58]) {
            editorWithLines.clearLineSign(line);
          }
        }
      }
    }
    if (key.shift && key.name === "d") {
      key.preventDefault();
      if (editorWithLines && !editorWithLines.isDestroyed) {
        diagnosticsEnabled = !diagnosticsEnabled;
        if (diagnosticsEnabled) {
          // Add diagnostic signs (errors, warnings, info) on some lines
          editorWithLines.setLineSign(0, "❌", "#ef4444"); // Line 1: Error
          editorWithLines.setLineSign(4, "⚠️", "#f59e0b"); // Line 5: Warning
          editorWithLines.setLineSign(10, "💡", "#3b82f6"); // Line 11: Info
          editorWithLines.setLineSign(25, "❌", "#ef4444"); // Line 26: Error
          editorWithLines.setLineSign(40, "⚠️", "#f59e0b"); // Line 41: Warning
          editorWithLines.setLineSign(52, "💡", "#3b82f6"); // Line 53: Info
        } else {
          // Clear diagnostic signs (simplified - clear known diagnostic lines)
          for (const line of [0, 4, 10, 25, 40, 52]) {
            editorWithLines.clearLineSign(line);
          }
        }
      }
    }
    if (key.ctrl && (key.name === "pageup" || key.name === "pagedown")) {
      key.preventDefault();
      // editBuffer/gotoBufferEnd not available in stub — no-op
    }
    if (key.ctrl && key.name === "]") {
      key.preventDefault();
      localScrollSpeed = Math.min(100, localScrollSpeed + 4);
    }
    if (key.ctrl && key.name === "[") {
      key.preventDefault();
      localScrollSpeed = Math.max(4, localScrollSpeed - 4);
    }
  });
}

export function destroy(rendererInstance: CliRenderer): void {
  rendererInstance.clearFrameCallbacks();
  parentContainer?.destroy();
  parentContainer = null;
  editorWithLines = null;
  editor = null;
  statusText = null;
  renderer = null;
  highlightsEnabled = false;
  diagnosticsEnabled = false;
  localWrapMode = "word";
  localScrollSpeed = 4;
  localShowLineNumbers = true;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
    targetFps: 60,
  });
  run(renderer);
  setupCommonDemoKeys(renderer);
}
