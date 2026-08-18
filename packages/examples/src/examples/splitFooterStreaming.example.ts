#!/usr/bin/env bun

/**
 * Split Footer Surface Streaming Demo
 *
 * Demonstrates progressive text, code, and markdown streaming in a split-footer
 * layout. Content builds chunk-by-chunk across three edge-case scenarios, with
 * live status rendered in the footer table.
 *
 * Adapted from the OpenTUI split-footer-streaming-demo reference. BetterTUI has
 * no `createScrollbackSurface`, so content accumulates inside a contentBox that
 * fills the engine viewport above the footerTable.
 */

import {
  Box,
  CliRenderEvents,
  type CliRenderer,
  Code,
  type CodeOptions,
  Markdown,
  type MarkdownOptions,
  RGBA,
  Text,
  TextTable,
  createCliRenderer,
  parseColor,
} from "@bettertui/core";
import { SyntaxStyle } from "@bettertui/core";
import type { KeyEvent, TextChunk, TextTableContent } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

const FOOTER_HEIGHT = 10;
const DEFAULT_INTERVAL_MS = 180;
const MIN_INTERVAL_MS = 60;
const MAX_INTERVAL_MS = 1000;
const INTERVAL_STEP_MS = 40;

type StreamKind = "text" | "code" | "markdown";

interface ScenarioDefinition {
  kind: StreamKind;
  title: string;
  description: string;
  prefix: string;
  chunks: string[];
}

interface ActiveRun {
  id: number;
  scenario: ScenarioDefinition;
  renderable: Text | Code | Markdown;
  content: string;
  chunkIndex: number;
  committedRows: number;
  committedBlocks: number;
  cancelled: boolean;
  done: boolean;
}

const PALETTE = {
  background: "#0B1220",
  panel: "#101A2D",
  border: "#3B5B82",
  title: "#F4F8FF",
  status: "#D7E5FA",
  detail: "#A8C0E4",
  hint: "#8BA6CD",
  textAccent: "#66D9EF",
  codeAccent: "#FFD580",
  markdownAccent: "#C7A6FF",
  error: "#FF9B9B",
} as const;

const SURFACE_SYNTAX_STYLE = SyntaxStyle.fromStyles({
  default: { fg: RGBA.fromInts(230, 237, 243, 255) },
  keyword: { fg: RGBA.fromInts(255, 123, 114, 255), bold: true },
  string: { fg: RGBA.fromInts(165, 214, 255, 255) },
  comment: { fg: RGBA.fromInts(139, 148, 158, 255), italic: true },
  number: { fg: RGBA.fromInts(121, 192, 255, 255) },
  function: { fg: RGBA.fromInts(210, 168, 255, 255) },
  type: { fg: RGBA.fromInts(255, 166, 87, 255) },
  variable: { fg: RGBA.fromInts(230, 237, 243, 255) },
  property: { fg: RGBA.fromInts(121, 192, 255, 255) },
  "markup.heading": { fg: RGBA.fromInts(88, 166, 255, 255), bold: true },
  "markup.heading.1": { fg: RGBA.fromInts(0, 255, 136, 255), bold: true },
  "markup.strong": { fg: RGBA.fromInts(244, 248, 255, 255), bold: true },
  "markup.italic": { fg: RGBA.fromInts(200, 210, 220, 255), italic: true },
  "markup.list": { fg: RGBA.fromInts(121, 192, 255, 255) },
  "markup.raw": { fg: RGBA.fromInts(255, 213, 128, 255) },
  "markup.link": { fg: RGBA.fromInts(88, 166, 255, 255) },
  "markup.link.label": { fg: RGBA.fromInts(88, 166, 255, 255) },
  "markup.link.url": { fg: RGBA.fromInts(88, 166, 255, 255) },
  conceal: { fg: RGBA.fromInts(98, 114, 130, 255) },
});

const SCENARIOS: Record<StreamKind, ScenarioDefinition> = {
  text: {
    kind: "text",
    title: "text",
    prefix: "text> ",
    description: "Chunks cut through words, spaces, newlines, long tokens, and repeated padding.",
    chunks: [
      "Text chunks can la",
      "nd mid-word, mid-space, o",
      "r mid-newline. LongTokenWithoutNatural",
      "Breaks_1234567890_keeps_growing while previous wrap decisions stay under pressure.\n\n",
      "Bullets can start before their content arrives:\n- first item keep",
      "s expanding after later chunks\n- second item includes emoji 🚀 and CJK 漢",
      "字 across chunk boundaries\n\nIndented columns: alpha    be",
      "ta    gamma\nTrailing text lands in small fragments to expose unstable row endings.\n",
    ],
  },
  code: {
    kind: "code",
    title: "code",
    prefix: "code> ",
    description: "Chunks cut through keywords, identifiers, comments, strings, and punctuation.",
    chunks: [
      "export as",
      "ync function buildSurfaceRepo",
      "rt<TRecord extends Record<string, string>>(chunks: string[]) {\n",
      '  const longIdentifier = "LongTokenWithoutNatural',
      'Breaks_1234567890"\n',
      "  /* block comments can also arrive in pie",
      "ces while highlighting is still pending */\n",
      "  return chunks\n    .map((chunk, index) => `${index}:${chunk.trim()}-${longId",
      'entifier}`)\n    .join("\\n")\n}\n',
    ],
  },
  markdown: {
    kind: "markdown",
    title: "markdown",
    prefix: "md> ",
    description: "Chunks cut through headings, emphasis, table rows, blockquotes, and fenced code.",
    chunks: [
      "# Split Footer Ma",
      "rkdown Edge Cases\n\nParag",
      "raph with **bo",
      "ld**, `inline c",
      "ode`, emoji 🚀, CJK 漢",
      "字, and a [li",
      "nk](https://example.com/very/long/path) that arrives in pieces.\n\n",
      "| Key | Statu",
      "s | Notes |\n| --- | --- | --- |\n| text | partial | inline `LongTokenWithoutNatural",
      "Breaks_1234567890` grows |\n| code | async | escaped pipe A\\|B stays in one cell |\n",
      "| markdown | streaming | delimiter row and data rows arrived separately |\n\n| 甲 | 乙 | 丙 |\n| --- | --- | --- |\n| 漢 | 字 | 表 |\n",
      "| 流 | 式 | 測 |\n| 邊 | 界 | 行 |\n\n| 😀 | 🚀 | 🧪 |\n| --- | --- | --- |\n| 🎯 | ✨ | 📦 |\n",
      "| 🌊 | 🔥 | 🪄 |\n\n> Quote starts here and the rest of the block arrives",
      ' in the next chunk. Unicode repeats: 🚀 漢字.\n\n```ts\nconst rows = ["text", "code", "markdown"]\n',
      ".map((kind, index) => `${index}:${kind}`)\n```\n\n- list item opened",
      " in one chunk\n- second item closes the sample\n",
    ],
  },
};

function getScenarioAccent(kind: StreamKind): string {
  switch (kind) {
    case "text":
      return PALETTE.textAccent;
    case "code":
      return PALETTE.codeAccent;
    case "markdown":
      return PALETTE.markdownAccent;
  }
}

function tableCell(text: string, color: string, attributes = 0): TextChunk[] {
  return [
    {
      __isChunk: true,
      text,
      fg: parseColor(color),
      attributes,
    },
  ];
}

function footerRow(label: string, value: string, valueColor: string): TextTableContent[number] {
  return [
    tableCell(label.toUpperCase().padEnd(6, " "), PALETTE.hint, 1),
    tableCell(":", PALETTE.border),
    tableCell(` ${value}`, valueColor),
  ];
}

type ContentWidget = Box | Text | Code | Markdown;

class SplitFooterStreamingDemo {
  private shell: Box;
  private titleText: Text;
  private contentBox: Box;
  private footerTable: TextTable;

  private readonly contentItems: ContentWidget[] = [];

  private currentKind: StreamKind = "markdown";
  private inlinePrefix = false;
  private autoAdvance = true;
  private intervalMs = DEFAULT_INTERVAL_MS;
  private destroyed = false;
  private stepping = false;
  private autoTimer: ReturnType<typeof setInterval> | null = null;
  private activeRun: ActiveRun | null = null;
  private nextRunId = 1;
  private lastStatus = "Ready. Press R to replay the current sample.";
  private pendingReplayReason: string | null = null;
  private wrote = false;

  constructor(private renderer: CliRenderer) {
    this.renderer.setScreenMode("split-footer", FOOTER_HEIGHT);
    this.renderer.setBackgroundColor(PALETTE.background);

    this.shell = new Box(this.renderer, {
      id: "sfs-shell",
      width: "100%",
      height: "100%",
      border: ["top"],
      borderColor: PALETTE.border,
      backgroundColor: PALETTE.panel,
      paddingTop: 1,
      paddingBottom: 0,
      paddingLeft: 1,
      paddingRight: 1,
      gap: 0,
      flexDirection: "column",
    });

    this.titleText = new Text(this.renderer, {
      id: "sfs-title",
      width: "100%",
      height: 1,
      content: "Split Footer Surface Streaming Demo",
      fg: PALETTE.title,
    });

    this.contentBox = new Box(this.renderer, {
      id: "sfs-content",
      width: "100%",
      flexGrow: 1,
      flexDirection: "column",
      gap: 0,
      overflow: "hidden",
      paddingTop: 1,
    });

    this.footerTable = new TextTable(this.renderer, {
      id: "sfs-footer-table",
      width: "100%",
      wrapMode: "word",
      columnWidthMode: "content",
      columnFitter: "proportional",
      cellPadding: 0,
      border: false,
      outerBorder: false,
      showBorders: false,
      backgroundColor: "transparent",
      fg: PALETTE.detail,
      content: [],
    });

    this.shell.add(this.titleText);
    this.shell.add(this.contentBox);
    this.shell.add(this.footerTable);
    this.renderer.root.add(this.shell);

    this.renderer.keyInput.on("keypress", this.handleKeyPress);
    this.renderer.on(CliRenderEvents.RESIZE, this.handleResize);
    this.renderer.on(CliRenderEvents.DESTROY, this.handleRendererDestroy);

    this.refreshFooter();
    this.syncAutoTimer();
    this.requestReplay("Started markdown sample.");
  }

  private get currentScenario(): ScenarioDefinition {
    return SCENARIOS[this.currentKind];
  }

  private refreshFooter(): void {
    if (this.destroyed) {
      return;
    }

    const scenario = this.currentScenario;
    const run = this.activeRun;
    const runState = !run
      ? "idle"
      : run.done
        ? `done ${run.chunkIndex}/${run.scenario.chunks.length}`
        : `chunk ${run.chunkIndex}/${run.scenario.chunks.length}`;
    const committedState =
      scenario.kind === "markdown"
        ? `${run?.committedBlocks ?? 0} blocks committed`
        : `${run?.committedRows ?? 0} rows committed`;

    this.footerTable.content = [
      footerRow(
        "mode",
        `${scenario.title} · start ${this.inlinePrefix ? "inline-prefix" : "newline"} · auto ${this.autoAdvance ? `${this.intervalMs}ms` : "off"} · ${runState}`,
        getScenarioAccent(this.currentKind),
      ),
      footerRow(
        "status",
        this.lastStatus,
        this.lastStatus.startsWith("Error:") ? PALETTE.error : PALETTE.status,
      ),
      footerRow("stats", `${run?.content.length ?? 0} bytes · ${committedState}`, PALETTE.detail),
      footerRow("about", scenario.description, PALETTE.detail),
      footerRow("scene", "1 text · 2 code · 3 markdown · i inline-prefix", PALETTE.hint),
      footerRow(
        "flow",
        "r replay · n next · a auto · [ slower · ] faster · resize -> r",
        PALETTE.hint,
      ),
    ];
  }

  private syncAutoTimer(): void {
    if (this.autoTimer) {
      clearInterval(this.autoTimer);
      this.autoTimer = null;
    }

    if (!this.autoAdvance || this.destroyed) {
      return;
    }

    this.autoTimer = setInterval(() => {
      void this.stepCurrentRun();
    }, this.intervalMs);
  }

  private destroyActiveRun(): void {
    if (!this.activeRun) {
      return;
    }

    this.activeRun.cancelled = true;
    this.activeRun = null;

    for (const item of this.contentItems) {
      try {
        this.contentBox.remove(item);
        item.destroy();
      } catch {
        // ignore teardown races
      }
    }
    this.contentItems.length = 0;
    this.wrote = false;
  }

  private requestReplay(reason: string): void {
    this.pendingReplayReason = reason;
    this.destroyActiveRun();
    this.lastStatus = reason;
    this.refreshFooter();

    if (!this.stepping) {
      const nextReason = this.pendingReplayReason;
      this.pendingReplayReason = null;
      if (nextReason) {
        void this.replayCurrentScenario(nextReason);
      }
    }
  }

  private async replayCurrentScenario(reason: string): Promise<void> {
    if (this.destroyed) {
      return;
    }

    this.destroyActiveRun();
    this.lastStatus = reason;
    this.activeRun = this.createRun(this.currentScenario);
    this.refreshFooter();
    await this.stepCurrentRun();
  }

  private createRun(scenario: ScenarioDefinition): ActiveRun {
    if (this.wrote) {
      this.addSpacer();
    }

    if (this.inlinePrefix) {
      this.addInlinePrefix(scenario);
    }

    let renderable: Text | Code | Markdown;
    switch (scenario.kind) {
      case "text":
        renderable = new Text(this.renderer, {
          id: `sfs-stream-text-${this.nextRunId}`,
          content: "",
          width: "100%",
          wrapMode: "char",
          fg: PALETTE.title,
        });
        break;
      case "code":
        renderable = new Code(this.renderer, {
          id: `sfs-stream-code-${this.nextRunId}`,
          code: "",
          filetype: "typescript",
          syntaxStyle: SURFACE_SYNTAX_STYLE,
          width: "100%",
          wrapMode: "char",
        } as CodeOptions);
        break;
      case "markdown":
        renderable = new Markdown(this.renderer, {
          id: `sfs-stream-markdown-${this.nextRunId}`,
          content: "",
          width: "100%",
        } as MarkdownOptions);
        break;
    }

    this.contentBox.add(renderable);
    this.contentItems.push(renderable);
    this.wrote = true;

    return {
      id: this.nextRunId++,
      scenario,
      renderable,
      content: "",
      chunkIndex: 0,
      committedRows: 0,
      committedBlocks: 0,
      cancelled: false,
      done: false,
    };
  }

  private addSpacer(): void {
    const spacer = new Text(this.renderer, {
      id: `sfs-spacer-${this.nextRunId}`,
      width: "100%",
      height: 1,
      content: "",
    });
    this.contentBox.add(spacer);
    this.contentItems.push(spacer);
  }

  private addInlinePrefix(scenario: ScenarioDefinition): void {
    const prefix = new Text(this.renderer, {
      id: `sfs-prefix-${this.nextRunId}`,
      width: scenario.prefix.length,
      height: 1,
      content: scenario.prefix,
      fg: getScenarioAccent(scenario.kind),
    });
    this.contentBox.add(prefix);
    this.contentItems.push(prefix);
  }

  private async stepCurrentRun(): Promise<void> {
    if (this.destroyed || this.stepping) {
      return;
    }

    let run = this.activeRun;
    if (!run) {
      run = this.createRun(this.currentScenario);
      this.activeRun = run;
      this.lastStatus = `Started ${run.scenario.title} sample.`;
      this.refreshFooter();
    }

    if (run.done || run.cancelled) {
      return;
    }

    if (run.chunkIndex >= run.scenario.chunks.length) {
      run.done = true;
      this.lastStatus = `${run.scenario.title} sample finished. Press R to replay.`;
      this.refreshFooter();
      return;
    }

    this.stepping = true;

    const runId = run.id;
    const chunk = run.scenario.chunks[run.chunkIndex] ?? "";
    const isFinalChunk = run.chunkIndex === run.scenario.chunks.length - 1;
    run.chunkIndex += 1;
    run.content += chunk;

    try {
      await this.flushRun(run, isFinalChunk);

      if (run.cancelled || this.destroyed || this.activeRun?.id !== runId) {
        return;
      }

      if (isFinalChunk) {
        run.done = true;
        this.lastStatus = `${run.scenario.title} sample finished. Press R to replay.`;
      } else {
        this.lastStatus = `${run.scenario.title} chunk ${run.chunkIndex}/${run.scenario.chunks.length} committed.`;
      }
    } catch (error) {
      if (run.cancelled || this.destroyed) {
        return;
      }

      this.lastStatus = `Error: ${run.scenario.title} sample failed.`;
      console.error("split-footer-streaming-demo step failed", error);
      this.destroyActiveRun();
    } finally {
      this.stepping = false;
      this.refreshFooter();

      if (this.pendingReplayReason && !this.destroyed) {
        const reason = this.pendingReplayReason;
        this.pendingReplayReason = null;
        void this.replayCurrentScenario(reason);
      }
    }
  }

  private async flushRun(run: ActiveRun, done: boolean): Promise<void> {
    switch (run.scenario.kind) {
      case "text":
        await this.flushTextRun(run);
        return;
      case "code":
        await this.flushCodeRun(run);
        return;
      case "markdown":
        await this.flushMarkdownRun(run, done);
        return;
    }
  }

  private async flushTextRun(run: ActiveRun): Promise<void> {
    const renderable = run.renderable as Text;
    renderable.content = run.content;
    const targetRows = Math.max(1, Math.ceil(run.content.length / 80));
    run.committedRows = Math.max(run.committedRows, targetRows);
  }

  private async flushCodeRun(run: ActiveRun): Promise<void> {
    const renderable = run.renderable as Code;
    renderable.code = run.content;
    const targetRows = Math.max(1, (run.content.match(/\n/g) ?? []).length + 1);
    run.committedRows = Math.max(run.committedRows, targetRows);
  }

  private async flushMarkdownRun(run: ActiveRun, done: boolean): Promise<void> {
    const renderable = run.renderable as Markdown;
    renderable.markdown = run.content;
    const headings = (run.content.match(/^#{1,6}\s/gm) ?? []).length;
    const paragraphs = (run.content.match(/\n\n/g) ?? []).length + 1;
    const targetBlocks = headings + paragraphs;
    run.committedBlocks = done ? targetBlocks : Math.max(run.committedBlocks, targetBlocks - 1);
  }

  private setScenario(kind: StreamKind): void {
    if (this.currentKind === kind) {
      this.requestReplay(`Replaying ${kind} sample.`);
      return;
    }

    this.currentKind = kind;
    this.requestReplay(`Switched to ${kind} sample.`);
  }

  private toggleAutoAdvance(): void {
    this.autoAdvance = !this.autoAdvance;
    this.syncAutoTimer();

    if (this.autoAdvance && (!this.activeRun || this.activeRun.done)) {
      this.requestReplay(`Auto advance enabled at ${this.intervalMs}ms.`);
      return;
    }

    this.lastStatus = this.autoAdvance
      ? `Auto advance enabled at ${this.intervalMs}ms.`
      : "Auto advance disabled.";
    this.refreshFooter();
  }

  private toggleInlinePrefix(): void {
    this.inlinePrefix = !this.inlinePrefix;
    this.requestReplay(
      this.inlinePrefix ? "Inline-prefix mode enabled." : "New-line mode enabled.",
    );
  }

  private adjustInterval(deltaMs: number): void {
    const next = Math.min(Math.max(this.intervalMs + deltaMs, MIN_INTERVAL_MS), MAX_INTERVAL_MS);
    if (next === this.intervalMs) {
      this.lastStatus = "Interval already at the limit.";
      this.refreshFooter();
      return;
    }

    this.intervalMs = next;
    this.syncAutoTimer();
    this.lastStatus = `Auto advance interval ${next}ms.`;
    this.refreshFooter();
  }

  private handleKeyPress = (key: KeyEvent): void => {
    if (key.ctrl || key.meta || key.option) {
      return;
    }

    switch (key.name) {
      case "1":
        key.preventDefault();
        this.setScenario("text");
        return;
      case "2":
        key.preventDefault();
        this.setScenario("code");
        return;
      case "3":
        key.preventDefault();
        this.setScenario("markdown");
        return;
      case "r":
        key.preventDefault();
        this.requestReplay(`Replaying ${this.currentKind} sample.`);
        return;
      case "n":
        key.preventDefault();
        void this.stepCurrentRun();
        return;
      case "a":
        key.preventDefault();
        this.toggleAutoAdvance();
        return;
      case "i":
        key.preventDefault();
        this.toggleInlinePrefix();
        return;
      case "[":
        key.preventDefault();
        this.adjustInterval(INTERVAL_STEP_MS);
        return;
      case "]":
        key.preventDefault();
        this.adjustInterval(-INTERVAL_STEP_MS);
        return;
    }
  };

  private handleResize = (): void => {
    if (this.destroyed) {
      return;
    }

    this.lastStatus = "Renderer resized. Press R to replay at the new width.";
    this.destroyActiveRun();
    this.refreshFooter();
  };

  private handleRendererDestroy = (): void => {
    this.destroy();
  };

  public destroy(): void {
    if (this.destroyed) {
      return;
    }

    this.destroyed = true;

    if (this.autoTimer) {
      clearInterval(this.autoTimer);
      this.autoTimer = null;
    }

    this.destroyActiveRun();

    this.renderer.keyInput.off("keypress", this.handleKeyPress);
    this.renderer.off(CliRenderEvents.RESIZE, this.handleResize);
    this.renderer.off(CliRenderEvents.DESTROY, this.handleRendererDestroy);

    if (!this.shell.isDestroyed) {
      this.shell.destroyRecursively();
    }
    this.renderer.setScreenMode("alternate-screen");
  }
}

let activeDemo: SplitFooterStreamingDemo | null = null;

export function run(renderer: CliRenderer): void {
  if (activeDemo) {
    activeDemo.destroy();
  }

  activeDemo = new SplitFooterStreamingDemo(renderer);
}

export function destroy(_renderer: CliRenderer): void {
  if (!activeDemo) {
    return;
  }

  activeDemo.destroy();
  activeDemo = null;
}

if (import.meta.main) {
  const renderer = await (
    createCliRenderer as (opts: Record<string, unknown>) => Promise<CliRenderer>
  )({
    targetFps: 30,
    exitOnCtrlC: true,
    useMouse: false,
    screenMode: "split-footer",
    footerHeight: FOOTER_HEIGHT,
    externalOutputMode: "capture-stdout",
    consoleMode: "disabled",
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
  renderer.start();
}
