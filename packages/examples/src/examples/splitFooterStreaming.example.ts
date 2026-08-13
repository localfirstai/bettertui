#!/usr/bin/env bun

import {
  Box,
  type CliRenderer,
  FrameBuffer,
  type FrameBufferLike,
  type FrameBufferOptions,
  RGBA,
  Text,
  TextTable,
  createCliRenderer,
  parseColor,
} from "@bettertui/core";
import type { TextChunk, TextTableContent } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

const FOOTER_HEIGHT = 10;
const DEFAULT_INTERVAL_MS = 180;
const MIN_INTERVAL_MS = 60;
const MAX_INTERVAL_MS = 1000;
const INTERVAL_STEP_MS = 40;
const IMG_WIDTH_RATIO = 0.36;
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
  renderable: Text;
  content: string;
  chunkIndex: number;
  committedRows: number;
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

const SCENARIOS: Record<StreamKind, ScenarioDefinition> = {
  text: {
    kind: "text",
    title: "text",
    prefix: "text> ",
    description: "Dummy text streaming with word boundaries and natural flow.",
    chunks: [
      "Welcome to the streaming demo! ",
      "This text is being streamed ",
      "chunk by chunk to simulate ",
      "real-time data flow. ",
      "Each chunk arrives sequentially ",
      "and gets appended to the display.\n\n",
      "Features demonstrated:\n",
      "• Smooth text streaming\n",
      "• Word boundary handling\n",
      "• Multi-paragraph support\n",
      "• Real-time updates\n\n",
      "Press keys to control the stream:",
    ],
  },
  code: {
    kind: "code",
    title: "code",
    prefix: "code> ",
    description: "Code streaming with syntax highlighting simulation.",
    chunks: [
      "function streamData() {\n",
      "  const chunks = [\n",
      "    'async ',\n",
      "    'function ',\n",
      "    'processStream ',\n",
      "    '(data) {',\n",
      "  ];\n\n",
      "  return chunks\n",
      "    .map((c, i) => `${i}: ${c}`)\n",
      "    .join('');\n",
      "}\n",
    ],
  },
  markdown: {
    kind: "markdown",
    title: "markdown",
    prefix: "md> ",
    description: "Markdown streaming with headers and formatting.",
    chunks: [
      "# Streaming Demo\n\n",
      "This is a **markdown** streaming ",
      "example that shows how formatted ",
      "text can be streamed progressively.\n\n",
      "## Features\n\n",
      "- Bullet points\n",
      "- Bold and *italic* text\n",
      "- Code blocks\n\n",
      "```\nStream complete!\n```",
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
    default:
      return PALETTE.status;
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

function hslToRgba(h: number, s: number, l: number): RGBA {
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = l - c / 2;
  let r = 0;
  let g = 0;
  let b = 0;
  if (h < 60) {
    r = c;
    g = x;
  } else if (h < 120) {
    r = x;
    g = c;
  } else if (h < 180) {
    g = c;
    b = x;
  } else if (h < 240) {
    g = x;
    b = c;
  } else if (h < 300) {
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

function plasmaPixel(x: number, y: number, width: number, height: number, time: number): RGBA {
  const nx = x / width;
  const ny = y / height;
  const v =
    Math.sin(nx * 10 + time) +
    Math.sin(ny * 10 + time * 0.7) +
    Math.sin((nx + ny) * 5 + time * 1.2) +
    Math.sin(Math.sqrt(nx * nx + ny * ny) * 8 + time * 0.5);
  const norm = (v + 4) / 8;
  const hue = (norm * 360 + time * 30) % 360;
  return hslToRgba(hue, 0.7, 0.3 + norm * 0.3);
}

function drawPlasmaFrame(
  buffer: FrameBufferLike,
  time: number,
  _kind: StreamKind,
  progress: number,
): void {
  const W = buffer.width;
  const H = buffer.height;
  const CH = Math.max(0, H - UI_OVERHEAD_LINES);
  const PH = Math.max(1, CH - 2);
  const _baseHue = (time * 30) % 360;
  const bg = RGBA.fromInts(10, 15, 25, 255);
  const fg = RGBA.fromInts(200, 210, 230, 255);

  buffer.clear(bg);

  // Draw plasma effect in main area
  for (let y = 1; y < PH + 1; y++) {
    for (let x = 1; x < W - 1; x++) {
      const color = plasmaPixel(x - 1, y - 1, W - 2, PH, time);
      buffer.setCell(x, y, "█", color, color);
    }
  }

  // Draw border
  const borderColor = RGBA.fromInts(59, 91, 130, 255);
  for (let x = 0; x < W; x++) {
    buffer.setCell(x, 0, "─", borderColor, bg);
    buffer.setCell(x, H - 1, "─", borderColor, bg);
  }
  for (let y = 0; y < H; y++) {
    buffer.setCell(0, y, "│", borderColor, bg);
    buffer.setCell(W - 1, y, "│", borderColor, bg);
  }
  buffer.setCell(0, 0, "┌", borderColor, bg);
  buffer.setCell(W - 1, 0, "┐", borderColor, bg);
  buffer.setCell(0, H - 1, "└", borderColor, bg);
  buffer.setCell(W - 1, H - 1, "┘", borderColor, bg);

  // Draw title
  const title = "Plasma Visualizer";
  const titleX = Math.floor((W - title.length) / 2);
  for (let i = 0; i < title.length; i++) {
    buffer.setCell(titleX + i, 0, title[i] ?? "", fg, borderColor);
  }

  // Draw progress bar
  const barY = H - 3;
  const barWidth = W - 4;
  const filledWidth = Math.floor(barWidth * progress);
  const filledFg = RGBA.fromInts(100, 200, 255, 255);
  const emptyFg = RGBA.fromInts(60, 60, 80, 255);
  const barBg = RGBA.fromInts(20, 20, 30, 255);

  buffer.setCell(1, barY, "[", fg, barBg);
  for (let x = 0; x < barWidth; x++) {
    const char = x < filledWidth ? "█" : "░";
    const color = x < filledWidth ? filledFg : emptyFg;
    buffer.setCell(2 + x, barY, char, color, barBg);
  }
  buffer.setCell(2 + barWidth, barY, "]", fg, barBg);

  // Draw stats
  const statsText = `Time: ${time.toFixed(1)}s | Progress: ${Math.floor(progress * 100)}%`;
  const statsX = Math.max(1, Math.floor((W - statsText.length) / 2));
  const statsColor = RGBA.fromInts(150, 170, 200, 255);
  for (let i = 0; i < statsText.length && statsX + i < W - 1; i++) {
    buffer.setCell(statsX + i, H - 2, statsText[i] ?? "", statsColor, bg);
  }
}

class SplitFooterStreamingDemo {
  private shell: Box;
  private titleText: Text;
  private mainRow: Box;
  private contentBox: Box;
  private imageBox: Box;
  private imagePanel: FrameBuffer | null = null;
  private footerTable: TextTable;

  private currentKind: StreamKind = "text";
  private inlinePrefix = false;
  private autoAdvance = true;
  private intervalMs = DEFAULT_INTERVAL_MS;
  private destroyed = false;
  private stepping = false;
  private autoTimer: ReturnType<typeof setInterval> | null = null;
  private activeRun: ActiveRun | null = null;
  private nextRunId = 1;
  private lastStatus = "";
  private pendingReplayReason: string | null = null;
  private animTime = 0;
  private imgW = 30;
  private imgH = FOOTER_HEIGHT - 2;
  private frameCb: ((dt: number) => void) | null = null;

  constructor(private readonly renderer: CliRenderer) {
    this.shell = new Box(renderer, {
      id: "sfs-shell",
      width: "100%",
      height: "100%",
      border: false,
      backgroundColor: PALETTE.background,
      padding: 1,
      gap: 1,
      flexDirection: "column",
      zIndex: 1,
    });

    const headerRow = new Box(renderer, {
      id: "sfs-header",
      width: "100%",
      height: 1,
      flexDirection: "row",
      justifyContent: "space-between",
      alignItems: "center",
    });

    this.titleText = new Text(renderer, {
      id: "sfs-title",
      content: "Split Footer Streaming Demo",
      fg: PALETTE.title,
    });

    const modeText = new Text(renderer, {
      id: "sfs-mode-indicator",
      content: "MODE: streaming",
      fg: PALETTE.hint,
    });

    headerRow.add(this.titleText);
    headerRow.add(modeText);

    this.mainRow = new Box(renderer, {
      id: "sfs-main-row",
      width: "100%",
      flexGrow: 1,
      flexDirection: "row",
      gap: 1,
      overflow: "hidden",
    });

    this.contentBox = new Box(renderer, {
      id: "sfs-content-box",
      flexGrow: 1,
      flexDirection: "column",
      gap: 1,
      overflow: "hidden",
      paddingTop: 1,
    });

    this._computeImageDimensions();
    this.imageBox = new Box(renderer, {
      id: "sfs-image-box",
      width: this.imgW,
      flexDirection: "column",
      gap: 1,
      border: true,
      borderColor: PALETTE.border,
      paddingLeft: 1,
    });

    this.imagePanel = this._createImagePanel();
    this.imageBox.add(this.imagePanel);
    this.mainRow.add(this.contentBox);
    this.mainRow.add(this.imageBox);

    const footerRow = new Box(renderer, {
      id: "sfs-footer",
      width: "100%",
      height: FOOTER_HEIGHT,
      flexDirection: "column",
      gap: 0,
    });

    this.footerTable = new TextTable(renderer, {
      id: "sfs-footer-table",
      width: "100%",
      wrapMode: "char",
      columnWidthMode: "content",
      columnFitter: "balanced",
      cellPadding: 1,
      border: false,
      outerBorder: false,
      showBorders: false,
      backgroundColor: PALETTE.panel,
      fg: PALETTE.status,
      content: [],
    });

    footerRow.add(this.footerTable);

    this.shell.add(headerRow);
    this.shell.add(this.mainRow);
    this.shell.add(footerRow);
    renderer.root.add(this.shell);

    this.frameCb = (dt: number) => {
      if (this.imagePanel && !this.imagePanel.isDestroyed) {
        this.imagePanel.draw(dt);
      }
    };
    renderer.setFrameCallback(this.frameCb);

    this.refreshFooter();
    void this.replayCurrentScenario("Initial");
    this.syncAutoTimer();
  }

  private _computeImageDimensions(): void {
    const W = this.renderer.terminalWidth;
    const H = this.renderer.terminalHeight;
    const CH = Math.max(1, H - FOOTER_HEIGHT - UI_OVERHEAD_LINES);
    const IH = Math.max(1, CH - 2);
    this.imgH = IH;
    this.imgW = Math.max(10, Math.floor((W - 10) * IMG_WIDTH_RATIO));
  }

  private _createImagePanel(): FrameBuffer {
    const { imgW, imgH } = this;
    const panelW = Math.max(1, imgW - 2);
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
    const committedState = run ? `${run.committedRows} rows` : "—";

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
    }, 50);
  }

  private async replayCurrentScenario(reason: string): Promise<void> {
    this.destroyActiveRun();
    await this.flushRun(reason);
    void this.createRun(reason);
  }

  private createRun(reason: string): ActiveRun {
    const scenario = this.currentScenario;
    const container = new Box(this.renderer, {
      id: `sfs-run-container-${this.nextRunId}`,
      width: "100%",
      height: "100%",
      flexDirection: "column",
      gap: 0,
      paddingTop: 0,
      paddingBottom: 0,
    });

    const renderable = new Text(this.renderer, {
      id: `sfs-run-text-${this.nextRunId}`,
      content: this.inlinePrefix ? `${reason} | ${scenario.prefix}` : reason,
      width: "100%",
      wrapMode: "word",
      fg: PALETTE.status,
    });

    container.add(renderable);
    this.contentBox.add(container);

    const run: ActiveRun = {
      id: this.nextRunId++,
      scenario,
      container,
      renderable,
      content: this.inlinePrefix ? `${reason} | ${scenario.prefix}` : reason,
      chunkIndex: 0,
      committedRows: 1,
      cancelled: false,
      done: false,
    };

    this.activeRun = run;
    this.refreshFooter();
    return run;
  }

  private async stepCurrentRun(): Promise<void> {
    if (this.destroyed || this.stepping) return;
    const run = this.activeRun;
    if (!run || run.cancelled || run.done) return;

    this.stepping = true;
    const _runId = run.id;
    const scenario = run.scenario;

    if (run.chunkIndex >= scenario.chunks.length) {
      run.done = true;
      this.stepping = false;
      this.refreshFooter();
      return;
    }

    const chunk = scenario.chunks[run.chunkIndex];
    run.chunkIndex++;
    run.content += chunk;
    run.renderable.content = run.content;

    // Estimate rows based on content length and terminal width
    const termWidth = this.renderer.terminalWidth;
    const contentWidth = (this.contentBox.width as number) ?? termWidth;
    const charsPerRow = Math.max(1, contentWidth - 4);
    run.committedRows = Math.ceil(run.content.length / charsPerRow);

    this.stepping = false;
    this.refreshFooter();
  }

  private async flushRun(reason: string): Promise<void> {
    const run = this.activeRun;
    if (!run) return;

    const text = run.renderable;
    const targetRows = run.committedRows + 1;

    while (run.committedRows < targetRows && run.chunkIndex < run.scenario.chunks.length) {
      run.content += run.scenario.chunks[run.chunkIndex];
      run.chunkIndex++;
      run.committedRows++;
    }

    run.done = true;
    text.content = `${run.content}\n[${reason}]`;
    this.refreshFooter();
  }

  private setScenario(kind: StreamKind): void {
    if (this.currentKind === kind) return;
    this.currentKind = kind;
    this.requestReplay("Switch");
    this.refreshFooter();
  }

  private toggleAutoAdvance(): void {
    this.autoAdvance = !this.autoAdvance;
    this.syncAutoTimer();
    const status = this.autoAdvance ? "resumed" : "paused";
    this.lastStatus = `Auto ${status}`;
    this.refreshFooter();
  }

  private toggleInlinePrefix(): void {
    this.inlinePrefix = !this.inlinePrefix;
    this.requestReplay("Prefix");
  }

  private adjustInterval(delta: number): void {
    const next = Math.max(MIN_INTERVAL_MS, Math.min(MAX_INTERVAL_MS, this.intervalMs + delta));
    if (next !== this.intervalMs) {
      this.intervalMs = next;
      this.syncAutoTimer();
      this.refreshFooter();
    }
  }

  private handleKeyPress = (key: {
    name?: string;
    sequence?: string;
  }): void => {
    const keyName = key.name;
    const seq = key.sequence;

    if (keyName === "t") {
      this.setScenario("text");
    } else if (keyName === "c") {
      this.setScenario("code");
    } else if (keyName === "m") {
      this.setScenario("markdown");
    } else if (keyName === "r") {
      this.requestReplay("Replay");
    } else if (keyName === "a") {
      this.toggleAutoAdvance();
    } else if (keyName === "p") {
      this.toggleInlinePrefix();
    } else if (seq === "=") {
      this.adjustInterval(-INTERVAL_STEP_MS);
    } else if (seq === "-") {
      this.adjustInterval(INTERVAL_STEP_MS);
    } else if (keyName === "space" && this.activeRun?.done) {
      this.requestReplay("Replay");
    } else if (!this.autoAdvance && (keyName === "n" || keyName === "return")) {
      void this.stepCurrentRun();
    }
  };

  private handleResize = (): void => {
    this._rebuildImagePanel();
    this.refreshFooter();
  };

  private handleRendererDestroy = (): void => {
    this.destroy();
  };

  public destroy(): void {
    if (this.destroyed) return;
    this.destroyed = true;

    if (this.autoTimer) {
      clearInterval(this.autoTimer);
      this.autoTimer = null;
    }

    this.destroyActiveRun();

    this.frameCb = null;
    this.renderer.clearFrameCallbacks();

    if (this.imagePanel && !this.imagePanel.isDestroyed) {
      this.imagePanel.destroy();
      this.imagePanel = null;
    }

    try {
      this.renderer.root.remove(this.shell);
    } catch {
      // ignore
    }
    this.shell.destroyRecursively();
  }
}

let activeDemo: SplitFooterStreamingDemo | null = null;

export function run(renderer: CliRenderer): void {
  activeDemo = new SplitFooterStreamingDemo(renderer);
  renderer.keyInput.on("keypress", activeDemo["handleKeyPress"]);
  renderer.on("resize", activeDemo["handleResize"]);
  renderer.on("destroy", activeDemo["handleRendererDestroy"]);
}

export function destroy(renderer: CliRenderer): void {
  if (activeDemo) {
    renderer.keyInput.off("keypress", activeDemo["handleKeyPress"]);
    renderer.off("resize", activeDemo["handleResize"]);
    renderer.off("destroy", activeDemo["handleRendererDestroy"]);
    activeDemo.destroy();
    activeDemo = null;
  }
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    targetFps: 30,
    exitOnCtrlC: true,
    useMouse: true,
    screenMode: "split-footer",
    footerHeight: FOOTER_HEIGHT,
    externalOutputMode: "capture-stdout",
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
