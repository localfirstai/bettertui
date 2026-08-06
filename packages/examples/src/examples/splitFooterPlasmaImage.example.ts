import {
  Box,
  CliRenderEvents,
  type CliRenderer,
  Code,
  FrameBuffer,
  type FrameBufferLike,
  type KeyEvent,
  Markdown,
  RGBA,
  Text,
  TextTable,
  createCliRenderer,
  parseColor,
} from "@bettertui/core";
import type { FrameBufferOptions, TextTableContent } from "@bettertui/core";
import { SyntaxStyle } from "@bettertui/core";
import type { TextChunk } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

const FOOTER_HEIGHT = 10;
const DEFAULT_INTERVAL_MS = 180;
const MIN_INTERVAL_MS = 60;
const MAX_INTERVAL_MS = 1000;
const INTERVAL_STEP_MS = 40;
const IMG_WIDTH_RATIO = 0.36;
/** Lines consumed by title + stats + inter-row gaps. */
const UI_OVERHEAD_LINES = 7;

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
  container: Box;
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

// ── Plasma image helpers ──────────────────────────────────────────────────────

function hslToRgba(h: number, s: number, l: number): RGBA {
  const hue = ((h % 360) + 360) % 360;
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((hue / 60) % 2) - 1));
  const m = l - c / 2;
  let r = 0;
  let g = 0;
  let b = 0;
  if (hue < 60) {
    r = c;
    g = x;
  } else if (hue < 120) {
    r = x;
    g = c;
  } else if (hue < 180) {
    g = c;
    b = x;
  } else if (hue < 240) {
    g = x;
    b = c;
  } else if (hue < 300) {
    r = x;
    b = c;
  } else {
    r = c;
    b = x;
  }
  return RGBA.fromInts(
    Math.round((r + m) * 255),
    Math.round((g + m) * 255),
    Math.round((b + m) * 255),
    255,
  );
}

function plasmaPixel(
  px: number,
  py: number,
  W: number,
  H: number,
  t: number,
  baseHue: number,
): RGBA {
  const nx = (px / W) * 2 * Math.PI;
  const ny = (py / H) * 2 * Math.PI;
  const v =
    Math.sin(nx * 2 + t) +
    Math.sin(ny * 3 + t * 0.7) +
    Math.sin((nx + ny) * 1.5 + t * 1.2) +
    Math.sin(Math.sqrt((nx - Math.PI) ** 2 + (ny - Math.PI) ** 2) * 2 - t * 1.5);
  const norm = v / 4 + 0.5;
  const hue = (baseHue + norm * 55) % 360;
  return hslToRgba(hue, 0.82, 0.22 + norm * 0.45);
}

/** Renders a half-block plasma frame into `buffer` with a streaming progress bar. */
function drawPlasmaFrame(
  buffer: FrameBufferLike,
  t: number,
  kind: StreamKind,
  progress: number,
): void {
  const W = buffer.width;
  const CH = buffer.height;
  const PH = CH * 2;
  const baseHue = kind === "text" ? 185 : kind === "code" ? 38 : 270;

  for (let cy = 0; cy < CH; cy++) {
    for (let cx = 0; cx < W; cx++) {
      // Half-block: ▄ uses fg for the lower pixel, bg for the upper pixel.
      const bg = plasmaPixel(cx, cy * 2, W, PH, t, baseHue);
      const fg = plasmaPixel(cx, cy * 2 + 1, W, PH, t, baseHue);
      buffer.setCell(cx, cy, "▄", fg, bg);
    }
  }

  // Progress bar at the very bottom row (overwrite with solid blocks).
  if (CH > 1) {
    const barW = Math.round(W * Math.min(1, Math.max(0, progress)));
    const filledFg = RGBA.fromInts(255, 255, 255, 255);
    const emptyFg = RGBA.fromInts(60, 70, 90, 255);
    const barBg = RGBA.fromInts(15, 20, 35, 255);
    for (let x = 0; x < W; x++) {
      buffer.setCell(x, CH - 1, x < barW ? "█" : "▁", x < barW ? filledFg : emptyFg, barBg);
    }
  }
}

// ── Main demo class ───────────────────────────────────────────────────────────

class SplitFooterStreamingDemo {
  private shell: Box;
  private titleText: Text;
  private mainRow: Box;
  private contentBox: Box;
  private imageBox: Box;
  private imagePanel: FrameBuffer | null = null;
  private footerTable: TextTable;

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
  private animTime = 0;
  private imgW = 32;
  private imgH = 16;
  private frameCb: ((dt: number) => void) | null = null;

  constructor(private renderer: CliRenderer) {
    this.renderer.setScreenMode("split-footer", FOOTER_HEIGHT);
    this.renderer.setBackgroundColor(PALETTE.background);

    this._computeImageDimensions();

    this.shell = new Box(this.renderer, {
      id: "sfs-shell",
      width: "100%",
      height: "100%",
      border: false,
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
      content: "▌ Split Footer Streaming  ·  Live Plasma Image",
      fg: PALETTE.title,
    });

    this.mainRow = new Box(this.renderer, {
      id: "sfs-main-row",
      width: "100%",
      flexGrow: 1,
      flexDirection: "row",
      gap: 1,
      overflow: "hidden",
    });

    this.contentBox = new Box(this.renderer, {
      id: "sfs-content",
      flexGrow: 1,
      flexDirection: "column",
      gap: 0,
      overflow: "hidden",
      paddingTop: 1,
    });

    this.imageBox = new Box(this.renderer, {
      id: "sfs-image-box",
      width: this.imgW,
      flexDirection: "column",
      gap: 0,
      border: ["left"],
      borderColor: PALETTE.border,
      paddingLeft: 1,
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

    this.imagePanel = this._createImagePanel();

    this.imageBox.add(this.imagePanel);
    this.mainRow.add(this.contentBox);
    this.mainRow.add(this.imageBox);

    this.shell.add(this.titleText);
    this.shell.add(this.mainRow);
    this.shell.add(this.footerTable);

    this.renderer.root.add(this.shell);

    this.frameCb = (dt: number) => {
      if (this.imagePanel && !this.imagePanel.isDestroyed) {
        this.imagePanel.draw(dt);
      }
    };
    this.renderer.setFrameCallback(this.frameCb);

    this.renderer.keyInput.on("keypress", this.handleKeyPress);
    this.renderer.on(CliRenderEvents.RESIZE, this.handleResize);
    this.renderer.on(CliRenderEvents.DESTROY, this.handleRendererDestroy);

    this.refreshFooter();
    this.syncAutoTimer();
    this.requestReplay("Started markdown sample.");
  }

  private _computeImageDimensions(): void {
    const W = this.renderer.terminalWidth;
    const H = this.renderer.terminalHeight - FOOTER_HEIGHT;
    this.imgW = Math.max(12, Math.floor(W * IMG_WIDTH_RATIO));
    this.imgH = Math.max(4, H - UI_OVERHEAD_LINES);
  }

  private _createImagePanel(): FrameBuffer {
    const { imgW, imgH } = this;
    const panelW = Math.max(1, imgW - 2); // subtract left border + padding
    return new FrameBuffer(this.renderer, {
      id: `sfs-image-panel-${Date.now()}`,
      width: panelW,
      height: imgH,
      drawFn: (buffer, dt) => {
        this.animTime += dt / 1000;
        const run = this.activeRun;
        const progress = run ? run.chunkIndex / this.currentScenario.chunks.length : 0;
        drawPlasmaFrame(buffer, this.animTime, this.currentKind, progress);
      },
    } as FrameBufferOptions);
  }

  private _rebuildImagePanel(): void {
    if (this.imagePanel && !this.imagePanel.isDestroyed) {
      try {
        this.imageBox.remove(this.imagePanel);
      } catch {
        // ignore
      }
      this.imagePanel.destroy();
      this.imagePanel = null;
    }
    this._computeImageDimensions();
    this.imageBox.setLayout({ width: this.imgW });
    this.imagePanel = this._createImagePanel();
    this.imageBox.add(this.imagePanel);
  }

  private get currentScenario(): ScenarioDefinition {
    return SCENARIOS[this.currentKind];
  }

  private refreshFooter(): void {
    const scenario = this.currentScenario;
    const run = this.activeRun;
    const runState = run
      ? `${run.done ? "done" : run.chunkIndex === 0 ? "starting" : "streaming"} ${run.chunkIndex}/${scenario.chunks.length}`
      : "idle";
    const committedState = run
      ? scenario.kind === "markdown"
        ? `${run.committedBlocks} blocks`
        : `${run.committedRows} rows`
      : "—";

    this.footerTable.content = [
      footerRow("mode", scenario.title, getScenarioAccent(scenario.kind)),
      footerRow("auto", this.autoAdvance ? `${this.intervalMs}ms` : "paused", PALETTE.status),
      footerRow("run", runState, PALETTE.detail),
      footerRow("commit", committedState, PALETTE.detail),
      footerRow(
        "keys",
        "T=text | C=code | M=md | R=replay | A=auto | P=prefix | ±=speed | ESC=exit",
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
      if (!this.stepping) {
        void this.stepCurrentRun();
      }
    }, this.intervalMs);
  }

  private destroyActiveRun(): void {
    if (!this.activeRun) {
      return;
    }
    this.activeRun.cancelled = true;
    try {
      this.contentBox.remove(this.activeRun.container);
    } catch {
      // ignore
    }
    this.activeRun = null;
  }

  private requestReplay(reason: string): void {
    if (this.pendingReplayReason) {
      this.pendingReplayReason = reason;
      return;
    }
    this.pendingReplayReason = reason;
    setTimeout(() => {
      const nextReason = this.pendingReplayReason ?? "Replay";
      this.pendingReplayReason = null;
      void this.replayCurrentScenario(nextReason);
    }, 0);
  }

  private async replayCurrentScenario(reason: string): Promise<void> {
    this.lastStatus = reason;
    this.destroyActiveRun();
    this.lastStatus = reason;
    this.activeRun = this.createRun(this.currentScenario);
    this.refreshFooter();
    await this.stepCurrentRun();
  }

  private createRun(scenario: ScenarioDefinition): ActiveRun {
    const container = new Box(this.renderer, {
      id: `sfs-stream-container-${this.nextRunId}`,
      width: "100%",
      height: "auto",
      flexDirection: "column",
      gap: 0,
      paddingTop: 0,
      paddingBottom: 0,
    });

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
          content: "",
          filetype: "typescript",
          syntaxStyle: SURFACE_SYNTAX_STYLE,
          width: "100%",
          wrapMode: "char",
        } as import("@bettertui/core").CodeOptions);
        break;
      case "markdown":
        renderable = new Markdown(this.renderer, {
          id: `sfs-stream-markdown-${this.nextRunId}`,
          content: "",
          width: "100%",
        } as import("@bettertui/core").MarkdownOptions);
        break;
    }

    container.add(renderable);
    this.contentBox.add(container);

    return {
      id: this.nextRunId++,
      scenario,
      container,
      renderable,
      content: "",
      chunkIndex: 0,
      committedRows: 0,
      committedBlocks: 0,
      cancelled: false,
      done: false,
    };
  }

  private async stepCurrentRun(): Promise<void> {
    if (this.destroyed) {
      return;
    }

    const run = this.activeRun;
    if (!run || run.cancelled || run.done) {
      return;
    }

    this.stepping = true;
    const runId = run.id;
    const chunk = run.scenario.chunks[run.chunkIndex];
    const isFinalChunk = run.chunkIndex + 1 >= run.scenario.chunks.length;

    run.content += chunk;
    run.chunkIndex += 1;
    run.done = isFinalChunk;

    try {
      await this.flushRun(run);
    } catch (error) {
      this.lastStatus = error instanceof Error ? error.message : String(error);
    }

    if (this.destroyed || !this.activeRun || this.activeRun.id !== runId) {
      this.stepping = false;
      return;
    }

    if (isFinalChunk) {
      this.lastStatus = `Finished ${run.scenario.title} sample. Press R to replay or T/C/M to switch.`;
    }

    this.refreshFooter();
    this.stepping = false;

    if (isFinalChunk && this.autoAdvance) {
      const kinds: StreamKind[] = ["text", "code", "markdown"];
      const currentIndex = kinds.indexOf(this.currentKind);
      const nextIndex = (currentIndex + 1) % kinds.length;
      if (nextIndex !== currentIndex) {
        this.currentKind = kinds[nextIndex];
        this.requestReplay(`Auto-play: ${SCENARIOS[this.currentKind].title}`);
      }
    }
  }

  private async flushRun(run: ActiveRun): Promise<void> {
    switch (run.scenario.kind) {
      case "text":
        await this.flushTextRun(run);
        break;
      case "code":
        await this.flushCodeRun(run);
        break;
      case "markdown":
        await this.flushMarkdownRun(run);
        break;
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
    renderable.content = run.content;
    const targetRows = Math.max(1, (run.content.match(/\n/g) || []).length + 1);
    run.committedRows = Math.max(run.committedRows, targetRows);
  }

  private async flushMarkdownRun(run: ActiveRun): Promise<void> {
    const renderable = run.renderable as Markdown;
    renderable.content = run.content;
    const md = run.content.trim();
    const targetBlockCount = Math.max(1, (md.match(/^#{1,6}\s/gm) || []).length);
    const firstState = performance.now();
    const lastState = await new Promise<number>((resolve) => {
      setTimeout(() => resolve(performance.now()), 0);
    });
    const nextState = Math.max(run.chunkIndex, targetBlockCount);
    const endRow = Math.min(nextState, run.scenario.chunks.length);
    if (endRow > run.committedBlocks) {
      run.committedBlocks = endRow;
    }
    const duration = lastState - firstState;
    if (duration > 50) {
      console.log(`[perf] ${duration.toFixed(2)}ms markdown render`);
    }
  }

  private setScenario(kind: StreamKind): void {
    if (this.currentKind === kind) {
      this.requestReplay("Replaying same sample.");
      return;
    }
    this.currentKind = kind;
    this.requestReplay(`Switched to ${SCENARIOS[kind].title} sample.`);
  }

  private toggleAutoAdvance(): void {
    this.autoAdvance = !this.autoAdvance;
    this.syncAutoTimer();
    const status = this.autoAdvance ? "ON" : "paused";
    this.lastStatus = `Auto-advance: ${status}`;
    this.refreshFooter();
  }

  private toggleInlinePrefix(): void {
    this.inlinePrefix = !this.inlinePrefix;
    this.lastStatus = `Inline prefix: ${this.inlinePrefix ? "ON" : "OFF"}`;
    this.refreshFooter();
  }

  private adjustInterval(delta: number): void {
    const next = Math.min(MAX_INTERVAL_MS, Math.max(MIN_INTERVAL_MS, this.intervalMs + delta));
    if (next !== this.intervalMs) {
      this.intervalMs = next;
      this.syncAutoTimer();
      this.lastStatus = `Interval: ${this.intervalMs}ms`;
      this.refreshFooter();
    }
  }

  private handleKeyPress = (key: KeyEvent): void => {
    if (key.name === "r" || key.name === "R") {
      key.preventDefault();
      this.requestReplay("Manual replay");
      return;
    }
    if (key.name === "t" || key.name === "T") {
      key.preventDefault();
      this.setScenario("text");
      return;
    }
    if (key.name === "c" || key.name === "C") {
      key.preventDefault();
      this.setScenario("code");
      return;
    }
    if (key.name === "m" || key.name === "M") {
      key.preventDefault();
      this.setScenario("markdown");
      return;
    }
    if (key.name === "a" || key.name === "A") {
      key.preventDefault();
      this.toggleAutoAdvance();
      return;
    }
    if (key.name === "p" || key.name === "P") {
      key.preventDefault();
      this.toggleInlinePrefix();
      return;
    }
    if (key.name === "=" || key.name === "+") {
      key.preventDefault();
      this.adjustInterval(-INTERVAL_STEP_MS);
      return;
    }
    if (key.name === "-" || key.name === "_") {
      key.preventDefault();
      this.adjustInterval(INTERVAL_STEP_MS);
      return;
    }
    if (key.name === "escape") {
      key.preventDefault();
      this.destroy();
      return;
    }
    setupCommonDemoKeys(this.renderer);
  };

  private handleResize = (): void => {
    this._rebuildImagePanel();
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

    this.destroyActiveRun();

    if (this.autoTimer) {
      clearInterval(this.autoTimer);
      this.autoTimer = null;
    }

    if (this.frameCb) {
      this.renderer.removeFrameCallback(this.frameCb);
      this.frameCb = null;
    }

    if (this.imagePanel && !this.imagePanel.isDestroyed) {
      this.imagePanel.destroy();
      this.imagePanel = null;
    }

    this.renderer.keyInput.off("keypress", this.handleKeyPress);
    this.renderer.off(CliRenderEvents.RESIZE, this.handleResize);
    this.renderer.off(CliRenderEvents.DESTROY, this.handleRendererDestroy);

    this.shell.destroy();
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
}
