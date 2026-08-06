#!/usr/bin/env bun

import { writeFile } from "node:fs/promises";
import {
  Box,
  type CliRenderer,
  type KeyEvent,
  type PasteEvent,
  Text,
  createCliRenderer,
  decodePasteBytes,
} from "@bettertui/core";
import { ScrollBox } from "@bettertui/core";
import { env, registerEnvVar } from "@bettertui/core";
import { formatScalar, safeJson } from "../lib/formatUtils";
import {
  type KeySnapshot,
  type PasteSnapshot,
  formatBaseCode,
  formatBaseCodeBrief,
  formatCombo,
  formatInline,
  formatModifiers,
} from "../lib/keyboardFormatting";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";
import { pad, truncate } from "../lib/textUtils";
import { formatClock } from "../lib/timeUtils";

registerEnvVar({
  name: "OTUI_KEYPRESS_DEBUG_SHOW_JSON",
  description: "Show full JSON for the latest parsed event in the keypress debug tool",
  type: "boolean",
  default: false,
});

const MAX_VISIBLE_EVENTS = 120;

type DebugEventType = "keypress" | "keyrelease" | "paste";

interface RawInputRecord {
  timestamp: string;
  sequence: string;
}

type DebugSnapshot = KeySnapshot | PasteSnapshot;

interface DebugEntry {
  id: number;
  timestamp: string;
  type: DebugEventType;
  snapshot: DebugSnapshot;
}

interface SavedEventRecord {
  timestamp: string;
  type: DebugEventType;
  event: DebugSnapshot;
}

let mainContainer: Box | null = null;
let statusText: Text | null = null;
let footerText: Text | null = null;
let eventFeed: ScrollBox | null = null;
let eventListText: Text | null = null;
let detailFeed: ScrollBox | null = null;
let detailText: Text | null = null;
let helpModal: Box | null = null;
let helpContent: Text | null = null;

let showingHelp = false;
let showJson = false;
let eventCount = 0;
let lastSavedFile: string | null = null;
let lastSaveError: string | null = null;

let inputHandler: ((sequence: string) => boolean) | null = null;
let keypressHandler: ((event: KeyEvent) => void) | null = null;
let keyreleaseHandler: ((event: KeyEvent) => void) | null = null;
let pasteHandler: ((event: PasteEvent) => void) | null = null;

let allRawInputs: RawInputRecord[] = [];
let allKeyEvents: SavedEventRecord[] = [];
let visibleEvents: DebugEntry[] = [];
let lastRawInput: RawInputRecord | null = null;

function snapshotKeyEvent(event: KeyEvent): KeySnapshot {
  const e = event as unknown as Record<string, unknown>;
  return {
    name: event.name,
    ctrl: event.ctrl,
    meta: event.meta,
    shift: event.shift,
    option: e.option as boolean,
    sequence: event.sequence,
    raw: e.raw as string,
    eventType: e.eventType as string,
    source: e.source as string,
    number: e.number as boolean,
    code: e.code as string | undefined,
    super: e.super as boolean | undefined,
    hyper: e.hyper as boolean | undefined,
    capsLock: e.capsLock as boolean | undefined,
    numLock: e.numLock as boolean | undefined,
    baseCode: e.baseCode as number | undefined,
    repeated: e.repeated as boolean | undefined,
  };
}

function snapshotPasteEvent(event: PasteEvent): PasteSnapshot {
  return {
    byteLength: event.bytes.length,
    bytes: Array.from(event.bytes),
    text: decodePasteBytes(event.bytes),
    metadata: event.metadata,
  };
}

function pushVisibleEvent(entry: DebugEntry): void {
  visibleEvents.push(entry);
  if (visibleEvents.length > MAX_VISIBLE_EVENTS) {
    visibleEvents.shift();
  }
}

function terminalSummary(renderer: CliRenderer): string {
  const caps = renderer.capabilities as unknown as Record<string, unknown>;
  const terminal = caps?.terminal as Record<string, unknown> | undefined;
  const terminalName = (terminal?.name as string) ?? caps?.brand ?? "unknown";
  const terminalVersion = terminal?.version as string | undefined;
  return terminalVersion ? `${terminalName} ${terminalVersion}` : String(terminalName);
}

function latestEventSummary(): string {
  const latest = visibleEvents[visibleEvents.length - 1];
  if (!latest) {
    return "none";
  }

  if (latest.type === "paste") {
    const snapshot = latest.snapshot as PasteSnapshot;
    return `paste ${snapshot.byteLength}B`;
  }

  return formatCombo(latest.snapshot as KeySnapshot);
}

function createStatusText(renderer: CliRenderer): string {
  const summary = [
    "Keypress Debug",
    `events ${allKeyEvents.length}`,
    `visible ${visibleEvents.length}`,
    `raw ${allRawInputs.length}`,
    // biome-ignore lint/suspicious/noExplicitAny: accessing internal renderer flag
    `kitty ${(renderer as any).useKittyKeyboard ? "on" : "off"}`,
    `json ${showJson ? "on" : "off"}`,
  ].join(" | ");

  let feedback =
    "capture is centered on parsed events; raw input stays in the session detail and export";
  if (lastSaveError) {
    feedback = `save failed: ${truncate(lastSaveError, 64)}`;
  } else if (lastSavedFile) {
    feedback = `saved ${truncate(lastSavedFile, 64)}`;
  }

  return [
    summary,
    `latest ${latestEventSummary()} | terminal ${terminalSummary(renderer)} | ${feedback}`,
  ].join("\n");
}

function createEventRow(entry: DebugEntry, isLatest: boolean): string {
  const prefix = isLatest ? ">" : " ";
  const id = String(entry.id).padStart(3, "0");
  const time = formatClock(entry.timestamp);

  if (entry.type === "paste") {
    const snapshot = entry.snapshot as PasteSnapshot;
    const preview = truncate(JSON.stringify(snapshot.text), 30);
    const note = `bytes=${snapshot.byteLength} text=${preview}`;
    return `${prefix} ${id} ${time} ${pad("paste", 6)} ${pad("Paste", 18)} ${note}`;
  }

  const snapshot = entry.snapshot as KeySnapshot;
  const eventLabel = entry.type === "keypress" ? "down" : "up";
  const noteParts = [`src=${snapshot.source}`];

  if (snapshot.baseCode !== undefined) {
    noteParts.push(`base=${formatBaseCodeBrief(snapshot.baseCode)}`);
  }

  if (snapshot.raw && snapshot.raw !== snapshot.sequence) {
    noteParts.push(`raw=${formatInline(snapshot.raw, 20)}`);
  } else if (snapshot.sequence) {
    noteParts.push(`seq=${formatInline(snapshot.sequence, 20)}`);
  }

  if (snapshot.repeated) {
    noteParts.push("repeat");
  }

  return `${prefix} ${id} ${time} ${pad(eventLabel, 6)} ${pad(truncate(formatCombo(snapshot), 18), 18)} ${noteParts.join(" ")}`;
}

function createEventListText(): string {
  const lines = [
    "  ID  TIME         TYPE   KEY                NOTES",
    "  --- ------------ ------ ------------------ ----------------------------------------",
  ];

  if (visibleEvents.length === 0) {
    lines.push("  --  waiting for parsed input events --");
    return lines.join("\n");
  }

  for (let index = 0; index < visibleEvents.length; index += 1) {
    const entry = visibleEvents[index];
    if (!entry) continue;
    lines.push(createEventRow(entry, index === visibleEvents.length - 1));
  }

  return lines.join("\n");
}

function createDetailText(renderer: CliRenderer): string {
  const latest = visibleEvents[visibleEvents.length - 1] ?? null;
  const lines = [
    "Session",
    `terminal      ${terminalSummary(renderer)}`,
    // biome-ignore lint/suspicious/noExplicitAny: accessing internal renderer flag
    `kitty kb      ${(renderer as any).useKittyKeyboard ? "on" : "off"}`,
    `raw inputs    ${allRawInputs.length}`,
    `parsed events ${allKeyEvents.length}`,
    `last raw      ${formatInline(lastRawInput?.sequence, 56)}`,
  ];

  if (!latest) {
    lines.push("", "Latest Event", "no parsed event yet");
    return lines.join("\n");
  }

  lines.push(
    "",
    "Latest Event",
    `index         #${latest.id}`,
    `type          ${latest.type}`,
    `time          ${latest.timestamp}`,
  );

  if (latest.type === "paste") {
    const snapshot = latest.snapshot as PasteSnapshot;
    lines.push(
      `bytes         ${snapshot.byteLength}`,
      `text          ${formatScalar(snapshot.text)}`,
      `metadata      ${formatScalar(snapshot.metadata)}`,
    );
  } else {
    const snapshot = latest.snapshot as KeySnapshot;
    const flagParts = [
      `repeated=${snapshot.repeated ? "yes" : "no"}`,
      `caps=${snapshot.capsLock ? "on" : "off"}`,
      `num=${snapshot.numLock ? "on" : "off"}`,
      `number=${snapshot.number ? "yes" : "no"}`,
    ];

    lines.push(
      `combo         ${formatCombo(snapshot)}`,
      `name          ${formatScalar(snapshot.name)}`,
      `sequence      ${formatScalar(snapshot.sequence)}`,
      `raw           ${formatScalar(snapshot.raw)}`,
      `source        ${snapshot.source}`,
      `event type    ${snapshot.eventType}`,
      `modifiers     ${formatModifiers(snapshot)}`,
      `code          ${formatScalar(snapshot.code)}`,
      `base code     ${formatBaseCode(snapshot.baseCode)}`,
      `flags         ${flagParts.join(" ")}`,
    );
  }

  if (showJson) {
    lines.push("", "JSON", safeJson(latest.snapshot, 2));
  }

  return lines.join("\n");
}

function createHelpText(): string {
  return [
    "Keypress Debug",
    "",
    "This demo now keeps the feed compact and centers the parsed key events.",
    "Raw input is still captured, but it lives in the session detail and the saved JSON instead of duplicating every row.",
    "",
    "Controls",
    "  ?       toggle help",
    "  Shift+J toggle JSON for the latest parsed event",
    "  Shift+S save the current capture to keypress-debug-*.json",
    "  Shift+L clear the current session",
    "  Ctrl+C  quit",
    "",
    "Tips",
    "  baseCode is shown in the detail pane for Kitty alternate-key events.",
    "  That makes layout and IME issues easier to inspect.",
  ].join("\n");
}

function refreshUi(renderer: CliRenderer): void {
  if (statusText) {
    statusText.content = createStatusText(renderer);
  }

  if (eventListText) {
    eventListText.content = createEventListText();
  }

  if (detailText) {
    detailText.content = createDetailText(renderer);
  }

  if (footerText) {
    footerText.content = "Controls: ?:help  Shift+J:json  Shift+S:save  Shift+L:clear  Ctrl+C:quit";
  }

  if (helpModal) {
    helpModal.visible = showingHelp;
  }

  if (helpContent) {
    helpContent.content = createHelpText();
  }

  if (detailFeed) {
    detailFeed.scrollTop = 0;
  }

  renderer.requestRender();
}

async function saveToFile(renderer: CliRenderer): Promise<void> {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
  const filename = `keypress-debug-${timestamp}.json`;

  const data = {
    exportedAt: new Date().toISOString(),
    rawInputs: allRawInputs,
    keyEvents: allKeyEvents,
    summary: {
      totalRawInputs: allRawInputs.length,
      totalKeyEvents: allKeyEvents.length,
      visibleEventWindow: visibleEvents.length,
    },
    capabilities: renderer.capabilities,
  };

  try {
    await writeFile(filename, JSON.stringify(data, null, 2));
    lastSavedFile = filename;
    lastSaveError = null;
  } catch (error) {
    lastSavedFile = null;
    lastSaveError = error instanceof Error ? error.message : String(error);
  }

  refreshUi(renderer);
}

function clearSession(renderer: CliRenderer): void {
  eventCount = 0;
  allRawInputs = [];
  allKeyEvents = [];
  visibleEvents = [];
  lastRawInput = null;
  lastSavedFile = null;
  lastSaveError = null;
  refreshUi(renderer);
}

function recordParsedEvent(
  renderer: CliRenderer,
  type: DebugEventType,
  snapshot: DebugSnapshot,
): void {
  eventCount += 1;

  const timestamp = new Date().toISOString();
  const entry: DebugEntry = {
    id: eventCount,
    timestamp,
    type,
    snapshot,
  };

  allKeyEvents.push({
    timestamp,
    type,
    event: snapshot,
  });
  pushVisibleEvent(entry);
  refreshUi(renderer);
}

export function run(renderer: CliRenderer): void {
  renderer.setBackgroundColor("#0D1117");
  showJson = Boolean(env.OTUI_KEYPRESS_DEBUG_SHOW_JSON);

  // biome-ignore lint/suspicious/noExplicitAny: accessing internal renderer debug API
  const cachedDebugInputs = (renderer as any).getDebugInputs?.() ?? [];
  if (cachedDebugInputs.length > 0) {
    allRawInputs.push(...cachedDebugInputs);
    lastRawInput = cachedDebugInputs[cachedDebugInputs.length - 1] ?? null;
  }

  mainContainer = new Box(renderer, {
    id: "keypress-debug-main",
    width: "100%",
    height: "100%",
    padding: 1,
    flexDirection: "column",
  });

  statusText = new Text(renderer, {
    id: "keypress-debug-status",
    width: "100%",
    height: 2,
    flexShrink: 0,
    fg: "#E6EDF3",
    wrapMode: "word",
    selectable: false,
  });

  const body = new Box(renderer, {
    id: "keypress-debug-body",
    width: "100%",
    flexGrow: 1,
    flexShrink: 1,
    flexDirection: "row",
  });

  eventFeed = new ScrollBox(renderer, {
    id: "keypress-debug-feed",
    flexGrow: 1,
    flexShrink: 1,
    border: true,
    borderColor: "#58A6FF",
    title: "Event Feed",
    titleAlignment: "left",
    stickyScroll: true,
    stickyStart: "bottom",
    backgroundColor: "#0F1722",
    contentOptions: {
      padding: 1,
    },
  });
  // biome-ignore lint/suspicious/noExplicitAny: setting internal ScrollBox option
  (eventFeed as any).verticalScrollbarOptions = { visible: false };

  eventListText = new Text(renderer, {
    id: "keypress-debug-feed-text",
    width: "100%",
    wrapMode: "none",
    truncate: true,
    fg: "#C9D1D9",
    selectable: false,
  });
  eventFeed.add(eventListText);

  detailFeed = new ScrollBox(renderer, {
    id: "keypress-debug-detail-feed",
    width: "42%",
    flexShrink: 0,
    marginLeft: 1,
    border: true,
    borderColor: "#A371F7",
    title: "Session / Latest Event",
    titleAlignment: "left",
    backgroundColor: "#111827",
    contentOptions: {
      padding: 1,
    },
  });
  // biome-ignore lint/suspicious/noExplicitAny: setting internal ScrollBox option
  (detailFeed as any).verticalScrollbarOptions = { visible: false };

  detailText = new Text(renderer, {
    id: "keypress-debug-detail-text",
    width: "100%",
    wrapMode: "word",
    fg: "#E6EDF3",
    selectable: false,
  });
  detailFeed.add(detailText);

  footerText = new Text(renderer, {
    id: "keypress-debug-footer",
    width: "100%",
    height: 1,
    flexShrink: 0,
    fg: "#8B949E",
    selectable: false,
  });

  body.add(eventFeed);
  body.add(detailFeed);
  mainContainer.add(statusText);
  mainContainer.add(body);
  mainContainer.add(footerText);
  renderer.root.add(mainContainer);

  helpModal = new Box(renderer, {
    id: "keypress-debug-help-modal",
    position: "absolute",
    left: "12%",
    top: 3,
    width: "76%",
    height: 16,
    padding: 1,
    border: true,
    borderStyle: "double",
    borderColor: "#4ECDC4",
    backgroundColor: "#0D1117",
    title: "Help",
    titleAlignment: "center",
    visible: false,
    zIndex: 100,
  });

  helpContent = new Text(renderer, {
    id: "keypress-debug-help-content",
    width: "100%",
    wrapMode: "word",
    fg: "#E6EDF3",
    selectable: false,
  });
  helpModal.add(helpContent);
  renderer.root.add(helpModal);

  inputHandler = (sequence: string) => {
    const record = {
      timestamp: new Date().toISOString(),
      sequence,
    };

    allRawInputs.push(record);
    lastRawInput = record;
    refreshUi(renderer);
    return false;
  };
  // biome-ignore lint/suspicious/noExplicitAny: accessing internal renderer input handler API
  (renderer as any).prependInputHandler(inputHandler);

  keypressHandler = (event: KeyEvent) => {
    const e = event as unknown as Record<string, unknown>;
    if (e.raw === "?" && !event.ctrl && !event.meta && !e.super && !e.hyper) {
      showingHelp = !showingHelp;
      refreshUi(renderer);
      return;
    }

    if (showingHelp && event.name === "escape") {
      showingHelp = false;
      refreshUi(renderer);
      return;
    }

    if (event.name === "j" && event.shift) {
      showJson = !showJson;
      refreshUi(renderer);
      return;
    }

    if (event.name === "s" && event.shift) {
      void saveToFile(renderer);
      return;
    }

    if (event.name === "l" && event.shift) {
      clearSession(renderer);
      return;
    }

    recordParsedEvent(renderer, "keypress", snapshotKeyEvent(event));
  };
  renderer.keyInput.on("keypress", keypressHandler);

  keyreleaseHandler = (event: KeyEvent) => {
    recordParsedEvent(renderer, "keyrelease", snapshotKeyEvent(event));
  };
  renderer.keyInput.on("keyrelease", keyreleaseHandler);

  pasteHandler = (event: PasteEvent) => {
    recordParsedEvent(renderer, "paste", snapshotPasteEvent(event));
  };
  renderer.keyInput.on("paste", pasteHandler);

  refreshUi(renderer);
}

export function destroy(renderer: CliRenderer): void {
  renderer.clearFrameCallbacks();

  if (keypressHandler) {
    renderer.keyInput.off("keypress", keypressHandler);
    keypressHandler = null;
  }

  if (keyreleaseHandler) {
    renderer.keyInput.off("keyrelease", keyreleaseHandler);
    keyreleaseHandler = null;
  }

  if (pasteHandler) {
    renderer.keyInput.off("paste", pasteHandler);
    pasteHandler = null;
  }

  if (inputHandler) {
    // biome-ignore lint/suspicious/noExplicitAny: accessing internal renderer input handler API
    (renderer as any).removeInputHandler(inputHandler);
    inputHandler = null;
  }

  if (mainContainer) {
    renderer.root.remove(mainContainer);
    mainContainer.destroyRecursively();
    mainContainer = null;
  }

  helpModal?.destroyRecursively();
  helpModal = null;
  helpContent = null;
  statusText = null;
  footerText = null;
  eventFeed = null;
  eventListText = null;
  detailFeed = null;
  detailText = null;

  showingHelp = false;
  showJson = false;
  eventCount = 0;
  lastSavedFile = null;
  lastSaveError = null;
  allRawInputs = [];
  allKeyEvents = [];
  visibleEvents = [];
  lastRawInput = null;
}

if (import.meta.main) {
  const renderer = await (
    createCliRenderer as (opts: Record<string, unknown>) => Promise<CliRenderer>
  )({
    exitOnCtrlC: true,
    targetFps: 60,
    useKittyKeyboard: { events: true },
  });
  run(renderer);
  setupCommonDemoKeys(renderer);
}
