#!/usr/bin/env bun

import {
  Box,
  CliRenderEvents,
  type CliRenderer,
  type KeyEvent,
  type PasteEvent,
  ScrollBox,
  type Selection,
  Text,
  Textarea,
  bg,
  bold,
  createCliRenderer,
  decodePasteBytes,
  fg,
  stripAnsiSequences,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

const P = {
  bg: "#08111f",
  panel: "#0f1b2d",
  border: "#34507c",
  borderHot: "#22d3ee",
  text: "#d7e3f7",
  muted: "#7d8da8",
  cyan: "#22d3ee",
  lime: "#bef264",
  rose: "#fb7185",
  amber: "#fbbf24",
  violet: "#a78bfa",
} as const;

type Tone = "muted" | "info" | "ok" | "warn" | "bad";

const TONE_COLOR: Record<Tone, string> = {
  muted: P.muted,
  info: P.cyan,
  ok: P.lime,
  warn: P.amber,
  bad: P.rose,
};

const TONE_ICON: Record<Tone, string> = {
  muted: "·",
  info: "→",
  ok: "✓",
  warn: "…",
  bad: "✗",
};

const MAX_LOG_ROWS = 80;
const SELECTION_BG = "#264f78";
const SELECTION_FG = "#ffffff";

interface Status {
  tone: Tone;
  text: string;
}

interface Fixture {
  name: string;
  short: string;
  purpose: string;
  payload: string;
}

const FIXTURES: readonly Fixture[] = [
  {
    name: "Unicode + LF",
    short: "Unicode + LF",
    purpose: "UTF-8 decoding and multiline insertion",
    payload: "OpenTUI clipboard round-trip\nUnicode: 世界 café 🚀\nLine endings: LF\nEnd",
  },
  {
    name: "CRLF + lone CR",
    short: "CRLF + CR",
    purpose: "Raw transport preserves CR; the editor normalizes to LF",
    payload: "OpenTUI newline fixture\r\nCRLF line\rLone CR line\nLF line",
  },
  {
    name: "ANSI text",
    short: "ANSI",
    purpose: "Raw event preserves ANSI; the textarea strips it on insertion",
    payload: "OpenTUI ANSI fixture: \x1b[31mred\x1b[0m plain",
  },
  {
    name: "Large (16 KiB)",
    short: "Large 16 KiB",
    purpose: "Exercises OSC 52 beyond the former fixed-buffer limit",
    payload: `OpenTUI large OSC 52 payload\n${"0123456789abcdef".repeat(1024)}`,
  },
];

const encoder = new TextEncoder();

let container: Box | null = null;
let tabsText: Text | null = null;
let fixtureText: Text | null = null;
let editor: Textarea | null = null;
let checksText: Text | null = null;
let logList: ScrollBox | null = null;
let logRows: Text[] = [];
let logRowId = 0;
let keypressHandler: ((event: KeyEvent) => void) | null = null;
let pasteHandler: ((event: PasteEvent) => void) | null = null;
let capabilityHandler: (() => void) | null = null;
let selectionHandler: ((selection: Selection) => void) | null = null;
let selectedFixture = 0;
let fixturePayloadEmitted = false;
let lastLoggedCapability = "";
let copyStatus: Status = { tone: "muted", text: "not attempted" };
let pasteStatus: Status = { tone: "muted", text: "waiting for a PasteEvent" };
let editorStatus: Status = {
  tone: "muted",
  text: "waiting for default insertion",
};
let roundTripStatus: Status = { tone: "muted", text: "not evaluated" };

function fixture(): Fixture {
  const f = FIXTURES[selectedFixture];
  if (!f) throw new Error(`No fixture at index ${selectedFixture}`);
  return f;
}

function normalizeNewlines(value: string): string {
  return value.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
}

function escapedPreview(value: string, maxLength = 64): string {
  const escaped = JSON.stringify(value);
  return escaped.length <= maxLength ? escaped : `${escaped.slice(0, maxLength - 3)}...`;
}

function byteLength(value: string): number {
  return encoder.encode(value).length;
}

function hexPrefix(bytes: Uint8Array, count = 12): string {
  const slice = bytes.slice(0, count);
  const hex = Array.from(slice, (byte) => byte.toString(16).padStart(2, "0")).join("");
  return bytes.length > count ? `${hex}…` : hex;
}

function timestamp(): string {
  const now = new Date();
  const hh = `${now.getHours()}`.padStart(2, "0");
  const mm = `${now.getMinutes()}`.padStart(2, "0");
  const ss = `${now.getSeconds()}`.padStart(2, "0");
  const ms = `${now.getMilliseconds()}`.padStart(3, "0");
  return `${hh}:${mm}:${ss}.${ms}`;
}

function capabilityStatus(renderer: CliRenderer): Status {
  const capabilities = renderer.capabilities;
  if (!capabilities) return { tone: "muted", text: "detecting" };
  const hint = capabilities.osc52 ? "yes" : "no";
  if (capabilities.osc52_support) {
    return { tone: "ok", text: `supported — emits (legacy hint: ${hint})` };
  }
  return {
    tone: "bad",
    text: `unsupported — emission blocked (legacy hint: ${hint})`,
  };
}

function metadataLabel(event: PasteEvent): string {
  if (!event.metadata) return "meta absent";
  return `meta kind=${event.metadata.kind ?? "unset"} mime=${event.metadata.mimeType ?? "unset"}`;
}

function statusChunk(status: Status) {
  return fg(TONE_COLOR[status.tone])(`${TONE_ICON[status.tone]} ${status.text}`);
}

function label(text: string) {
  return fg(P.muted)(text.padEnd(12));
}

function addLog(renderer: CliRenderer, tone: Tone, message: string, detail?: string): void {
  if (!logList) return;

  const row = new Text(renderer, {
    id: `clipboard-paste-log-${logRowId++}`,
    content: t`${fg(P.muted)(timestamp())} ${fg(TONE_COLOR[tone])(`${TONE_ICON[tone]} ${message}`)}`,
    flexGrow: 0,
    flexShrink: 0,
    selectionBg: SELECTION_BG,
    selectionFg: SELECTION_FG,
  });
  logList.add(row);
  logRows.push(row);

  if (detail) {
    const detailRow = new Text(renderer, {
      id: `clipboard-paste-log-${logRowId++}`,
      content: t`${fg(P.muted)(`             ${detail}`)}`,
      flexGrow: 0,
      flexShrink: 0,
      selectionBg: SELECTION_BG,
      selectionFg: SELECTION_FG,
    });
    logList.add(detailRow);
    logRows.push(detailRow);
  }

  while (logRows.length > MAX_LOG_ROWS) {
    const oldRow = logRows.shift();
    oldRow?.destroyRecursively();
  }
}

function updateTabs(): void {
  if (!tabsText) return;
  const chunks = FIXTURES.map((entry, index) => {
    const text = ` ${index + 1} ${entry.short} `;
    return index === selectedFixture ? bg(P.cyan)(fg(P.bg)(bold(text))) : fg(P.muted)(text);
  });
  tabsText.content = t`${chunks[0] ?? ""} ${chunks[1] ?? ""} ${chunks[2] ?? ""} ${chunks[3] ?? ""}`;
}

function updateFixturePanel(): void {
  if (!fixtureText) return;
  const current = fixture();
  fixtureText.content = t`${bold(fg(P.text)(current.name))} ${fg(P.muted)(`— ${byteLength(current.payload)} UTF-8 bytes`)}
${label("Purpose")} ${fg(P.text)(current.purpose)}
${label("Expected")} ${fg(P.violet)(escapedPreview(current.payload))}`;
}

function updateChecks(renderer: CliRenderer): void {
  if (!checksText) return;
  const capability = capabilityStatus(renderer);
  checksText.content = t`${label("Capability")} ${statusChunk(capability)}
${label("Copy OSC 52")} ${statusChunk(copyStatus)}
${label("Paste event")} ${statusChunk(pasteStatus)}
${label("Editor text")} ${statusChunk(editorStatus)}
${label("Round trip")} ${statusChunk(roundTripStatus)}`;
}

function resetTest(renderer: CliRenderer, reason: string): void {
  editor?.setText("");
  fixturePayloadEmitted = false;
  copyStatus = { tone: "muted", text: "not attempted" };
  pasteStatus = { tone: "muted", text: "waiting for a PasteEvent" };
  editorStatus = { tone: "muted", text: "waiting for default insertion" };
  roundTripStatus = { tone: "muted", text: "not evaluated" };
  updateTabs();
  updateFixturePanel();
  updateChecks(renderer);
  editor?.focus();
  addLog(renderer, "muted", `${reason} — editor cleared, checks idle`);
}

function panel(renderer: CliRenderer, id: string, title: string, height?: number): Box {
  return new Box(renderer, {
    id,
    title: ` ${title} `,
    titleAlignment: "left",
    border: true,
    borderStyle: "round",
    borderColor: P.border,
    backgroundColor: P.panel,
    paddingLeft: 1,
    paddingRight: 1,
    ...(height === undefined ? {} : { height }),
  });
}

export function run(renderer: CliRenderer): void {
  renderer.setBackgroundColor(P.bg);

  container = new Box(renderer, {
    id: "clipboard-paste-container",
    width: "100%",
    height: "100%",
    padding: 1,
    flexDirection: "column",
    backgroundColor: P.bg,
  });

  const header = new Text(renderer, {
    id: "clipboard-paste-header",
    height: 2,
    marginBottom: 1,
    content: t`${bold(fg(P.cyan)("CLIPBOARD + PASTE TEST BED"))} ${fg(P.muted)("— OSC 52 → PasteEvent → textarea")}
${bold(fg(P.amber)("1-4"))} ${fg(P.muted)("fixture")}  ${bold(fg(P.amber)("Ctrl+Y"))} ${fg(P.muted)("copy")}  ${bold(fg(P.amber)("Ctrl+K"))} ${fg(P.muted)("clear")}  ${bold(fg(P.amber)("Ctrl+R"))} ${fg(P.muted)("reset")}  ${bold(fg(P.amber)("drag-select"))} ${fg(P.muted)("copies via OSC 52")}`,
  });

  tabsText = new Text(renderer, {
    id: "clipboard-paste-tabs",
    height: 1,
    content: "",
  });

  const fixturePanel = panel(renderer, "clipboard-paste-fixture-panel", "Fixture", 5);
  fixturePanel.marginBottom = 1;
  fixtureText = new Text(renderer, {
    id: "clipboard-paste-fixture",
    content: "",
    selectionBg: SELECTION_BG,
    selectionFg: SELECTION_FG,
  });
  fixturePanel.add(fixtureText);

  const editorPanel = new Box(renderer, {
    id: "clipboard-paste-editor-box",
    title: " Paste target ",
    titleAlignment: "left",
    border: true,
    borderStyle: "round",
    borderColor: P.borderHot,
    backgroundColor: P.panel,
    paddingLeft: 1,
    paddingRight: 1,
    height: 6,
    marginBottom: 1,
  });

  editor = new Textarea(renderer, {
    id: "clipboard-paste-editor",
    width: "100%",
    height: "100%",
    placeholder: "Paste here... (Ctrl+R resets before an exact editor check)",
    textColor: P.text,
    backgroundColor: P.panel,
    focusedBackgroundColor: P.panel,
    cursorColor: P.amber,
    wrapMode: "word",
  });
  editorPanel.add(editor);

  const checksPanel = panel(renderer, "clipboard-paste-checks-panel", "Checks", 7);
  checksPanel.marginBottom = 1;
  checksText = new Text(renderer, {
    id: "clipboard-paste-checks",
    content: "",
    selectionBg: SELECTION_BG,
    selectionFg: SELECTION_FG,
  });
  checksPanel.add(checksText);

  const logPanel = new Box(renderer, {
    id: "clipboard-paste-log-panel",
    title: " Events ",
    titleAlignment: "left",
    border: true,
    borderStyle: "round",
    borderColor: P.border,
    backgroundColor: P.panel,
    paddingLeft: 1,
    paddingRight: 1,
    flexGrow: 1,
    flexShrink: 1,
    minHeight: 5,
    flexDirection: "column",
  });

  logList = new ScrollBox(renderer, {
    id: "clipboard-paste-log-list",
    stickyScroll: true,
    stickyStart: "bottom",
    rootOptions: { backgroundColor: P.panel, border: false },
    wrapperOptions: { backgroundColor: P.panel },
    viewportOptions: { backgroundColor: P.panel },
    contentOptions: { backgroundColor: P.panel },
    scrollbarOptions: {
      trackOptions: {
        foregroundColor: P.cyan,
        backgroundColor: P.border,
      },
    },
    height: "100%",
    width: "auto",
    flexGrow: 1,
    flexShrink: 1,
  });
  logPanel.add(logList);

  container.add(header);
  container.add(tabsText);
  container.add(fixturePanel);
  container.add(editorPanel);
  container.add(checksPanel);
  container.add(logPanel);
  renderer.root.add(container);

  pasteHandler = (event) => {
    const current = fixture();
    const pasted = decodePasteBytes(event.bytes);
    const exact = pasted === current.payload;
    const normalized = normalizeNewlines(pasted) === normalizeNewlines(current.payload);
    pasteStatus = exact
      ? {
          tone: "ok",
          text: `raw and normalized match — ${event.bytes.length} bytes`,
        }
      : normalized
        ? {
            tone: "warn",
            text: `normalized match only — raw differs (${event.bytes.length} bytes)`,
          }
        : {
            tone: "bad",
            text: `no fixture match — ${event.bytes.length} bytes`,
          };
    roundTripStatus =
      fixturePayloadEmitted && exact
        ? {
            tone: "ok",
            text: "observed after emission (terminal acceptance unacknowledged)",
          }
        : {
            tone: "warn",
            text: "not established — needs emission plus an exact raw match",
          };
    editorStatus = {
      tone: "warn",
      text: "pending default focused-renderable handling",
    };
    updateChecks(renderer);
    addLog(
      renderer,
      pasteStatus.tone,
      `paste ← ${event.bytes.length} B · ${metadataLabel(event)} · raw ${exact ? "✓" : "✗"} norm ${normalized ? "✓" : "✗"}`,
      `${escapedPreview(pasted, 44)} · hex ${hexPrefix(event.bytes, 8)}`,
    );

    queueMicrotask(() => {
      if (!editor || editor.isDestroyed) return;
      const expected = normalizeNewlines(stripAnsiSequences(current.payload));
      const pass = editor.plainText === expected;
      editorStatus = pass
        ? {
            tone: "ok",
            text: `matches expected editor text — ${byteLength(editor.plainText)} bytes retained`,
          }
        : {
            tone: "bad",
            text: `expected ${byteLength(expected)} bytes, retained ${byteLength(editor.plainText)} — ${escapedPreview(editor.plainText, 36)}`,
          };
      updateChecks(renderer);
      addLog(
        renderer,
        pass ? "ok" : "bad",
        pass
          ? `editor retained ${byteLength(editor.plainText)} B (ANSI stripped, newlines normalized)`
          : `editor mismatch — expected ${byteLength(expected)} B, retained ${byteLength(editor.plainText)} B`,
      );
    });
  };

  keypressHandler = (event) => {
    if (!event.ctrl && /^[1-4]$/.test(event.name)) {
      event.preventDefault();
      selectedFixture = Number(event.name) - 1;
      const current = fixture();
      resetTest(
        renderer,
        `fixture → ${selectedFixture + 1} ${current.name} (${byteLength(current.payload)} B)`,
      );
      return;
    }

    if (event.ctrl && event.name === "y") {
      event.preventDefault();
      const current = fixture();
      renderer.copyToClipboardOSC52(current.payload);
      const emitted = true;
      fixturePayloadEmitted = emitted;
      copyStatus = {
        tone: "info",
        text: `emitted ${byteLength(current.payload)} UTF-8 bytes`,
      };
      roundTripStatus = { tone: "warn", text: "waiting for an exact paste after emission" };
      updateChecks(renderer);
      addLog(
        renderer,
        "info",
        `osc52 copy → emitted ${byteLength(current.payload)} B (default clipboard target)`,
      );
      return;
    }

    if (event.ctrl && event.name === "k") {
      event.preventDefault();
      fixturePayloadEmitted = false;
      renderer.clearClipboardOSC52();
      copyStatus = { tone: "info", text: "emitted clear request" };
      roundTripStatus = {
        tone: "muted",
        text: "not applicable to clear requests",
      };
      updateChecks(renderer);
      addLog(renderer, "info", "osc52 clear → emitted");
      return;
    }

    if (event.ctrl && event.name === "r") {
      event.preventDefault();
      resetTest(renderer, "reset");
    }
  };

  capabilityHandler = () => {
    updateChecks(renderer);
    const capabilities = renderer.capabilities;
    if (!capabilities) return;
    const snapshot = `${capabilities.osc52_support}/${capabilities.osc52 ? "hint-yes" : "hint-no"}`;
    if (snapshot !== lastLoggedCapability) {
      lastLoggedCapability = snapshot;
      addLog(
        renderer,
        "info",
        `capabilities → osc52_support=${capabilities.osc52_support} legacy-hint=${capabilities.osc52 ? "yes" : "no"}`,
      );
    }
  };

  selectionHandler = (selection) => {
    if (selection.isDragging) return;
    const text = selection.getSelectedText();
    if (!text || text.trim().length === 0) return;

    renderer.clearSelection();
    renderer.copyToClipboardOSC52(text);
    fixturePayloadEmitted = false;
    copyStatus = {
      tone: "info",
      text: `emitted selection (${byteLength(text)} UTF-8 bytes)`,
    };
    if (roundTripStatus.text.startsWith("waiting")) {
      roundTripStatus = {
        tone: "muted",
        text: "superseded by selection copy",
      };
    }
    updateChecks(renderer);
    addLog(
      renderer,
      "info",
      `selection copy → emitted ${byteLength(text)} B`,
      escapedPreview(text, 56),
    );
  };

  renderer.on(CliRenderEvents.CAPABILITIES, capabilityHandler);
  renderer.on(CliRenderEvents.SELECTION, selectionHandler);
  renderer.keyInput.on("paste", pasteHandler);
  renderer.keyInput.on("keypress", keypressHandler);
  resetTest(renderer, "ready");
}

export function destroy(renderer: CliRenderer): void {
  if (pasteHandler) renderer.keyInput.off("paste", pasteHandler);
  if (keypressHandler) renderer.keyInput.off("keypress", keypressHandler);
  if (capabilityHandler) renderer.off(CliRenderEvents.CAPABILITIES, capabilityHandler);
  if (selectionHandler) renderer.off(CliRenderEvents.SELECTION, selectionHandler);
  renderer.clearSelection();
  container?.destroyRecursively();
  container = null;
  tabsText = null;
  fixtureText = null;
  editor = null;
  checksText = null;
  logList = null;
  logRows = [];
  logRowId = 0;
  lastLoggedCapability = "";
  pasteHandler = null;
  keypressHandler = null;
  capabilityHandler = null;
  selectionHandler = null;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
    targetFps: 30,
  });
  run(renderer);
  setupCommonDemoKeys(renderer);
}
