import {
  Box,
  type CliRenderer,
  FrameBuffer,
  type KeyEvent,
  RGBA,
  Text,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

const GRAPHEME_LINES: string[] = [
  "東京都  北京市  서울시  大阪府  名古屋  横浜市  上海市",
  "👨‍👩‍👧‍👦  👩🏽‍💻  🏳️‍🌈  🇺🇸  🇩🇪  🇯🇵  🇮🇳  家族  絵文字  🎉🎊🎈",
  "こんにちは世界  你好世界  안녕하세요  สวัสดี  مرحبا",
  "漢字テスト  中文测试  한국어  日本語  繁體中文  简体中文",
  "🚀 Full-width: ＡＢＣＤＥＦ  Half: abcdef  ½ ⅞ ⅓",
  "混合テキスト mixed text with 漢字 and emoji 🎯",
];

const HEADER_HEIGHT = 2;

let nextZIndex = 101;
let draggableBoxes: DraggableBox[] = [];
let scrimVisible = false;
let scrim: Box | null = null;
let headerDisplay: Text | null = null;

class DraggableBox extends Box {
  private isDragging = false;
  private dragOffsetX = 0;
  private dragOffsetY = 0;
  private alphaPercentage: number;

  constructor(
    ctx: CliRenderer,
    id: string,
    x: number,
    y: number,
    width: number,
    height: number,
    bg: RGBA,
    zIndex: number,
  ) {
    super(ctx, {
      id,
      width,
      height,
      zIndex,
      backgroundColor: bg,
      position: "absolute",
      left: x,
      top: y,
    });
    this.alphaPercentage = Math.round(bg.a * 100);

    // Alpha percentage label
    const alphaText = `${this.alphaPercentage}%`;
    const w = typeof width === "number" ? width : 0;
    const h = typeof height === "number" ? height : 0;
    const label = new Text(ctx, {
      id: `${id}-label`,
      content: alphaText,
      position: "absolute",
      left: Math.max(0, Math.floor(w / 2) - Math.floor(alphaText.length / 2)),
      top: Math.floor(h / 2),
    });
    label.fg = RGBA.fromInts(255, 255, 255, 220);
    this.add(label);

    // Wire up mouse event handling after super() returns
    this._options.onMouseDown = (e: unknown) => this._onMouse(e, "down");
    this._options.onMouseDrag = (e: unknown) => this._onMouse(e, "drag");
    this._options.onMouseDragEnd = (e: unknown) => this._onMouse(e, "drag-end");
  }

  private _onMouse(event: unknown, type: string): void {
    const e = event as { x?: number; y?: number; stopPropagation?: () => void };
    const ex = (e.x ?? 0) as number;
    const ey = (e.y ?? 0) as number;
    const w = typeof this._options.width === "number" ? this._options.width : 0;
    const h = typeof this._options.height === "number" ? this._options.height : 0;

    switch (type) {
      case "down":
        this.isDragging = true;
        this.dragOffsetX = ex - this.x;
        this.dragOffsetY = ey - this.y;
        this.zIndex = nextZIndex++;
        e.stopPropagation?.();
        break;
      case "drag-end":
        if (this.isDragging) {
          this.isDragging = false;
          e.stopPropagation?.();
        }
        break;
      case "drag":
        if (this.isDragging) {
          const newX = ex - this.dragOffsetX;
          const newY = ey - this.dragOffsetY;
          this.setPosition({
            left: Math.max(0, Math.min(newX, this._renderer.terminalWidth - w)),
            top: Math.max(0, Math.min(newY, this._renderer.terminalHeight - h)),
          });
          e.stopPropagation?.();
        }
        break;
    }
  }
}

class GraphemeBackground extends FrameBuffer {
  constructor(ctx: CliRenderer, id: string, width: number, height: number) {
    super(ctx, {
      id,
      width,
      height,
      position: "absolute",
      left: 0,
      top: HEADER_HEIGHT,
    });

    this.fillGraphemes(width, height);
  }

  private fillGraphemes(_width: number, _height: number) {
    const fgColor = RGBA.fromInts(220, 220, 220, 255);
    const bgColor = RGBA.fromInts(10, 14, 20, 255);
    this.frameBuffer.clear(bgColor);
    for (let y = 0; y < _height; y++) {
      const line = GRAPHEME_LINES[y % GRAPHEME_LINES.length];
      this.frameBuffer.drawText(line, 2, y, fgColor, bgColor);
    }
  }
}

function toggleScrim(renderer: CliRenderer) {
  scrimVisible = !scrimVisible;
  if (scrim) scrim.visible = scrimVisible;
  updateHeader();
  renderer.requestRender();
}

function updateHeader() {
  if (!headerDisplay) return;
  const dimLabel = scrimVisible ? "D: hide scrim" : "D: show scrim";
  headerDisplay.content = t`${bold(fg("#00D4AA")("Wide Grapheme Overlay"))} ${fg("#A8A8B2")(`| ${dimLabel} | Drag boxes over CJK/emoji | Ctrl+C: quit`)}`;
}

export function run(renderer: CliRenderer): void {
  renderer.start();
  renderer.setBackgroundColor("#0A0E14");

  const root = new Box(renderer, { id: "wg-overlay-root" });
  renderer.root.add(root);

  // Header row
  headerDisplay = new Text(renderer, {
    id: "wg-header",
    height: HEADER_HEIGHT,
    position: "absolute",
    left: 2,
    top: 0,
    zIndex: 200,
  });
  updateHeader();
  root.add(headerDisplay);

  // Background filled with repeating wide grapheme lines, below the header
  const bgHeight = renderer.terminalHeight - HEADER_HEIGHT;
  const background = new GraphemeBackground(
    renderer,
    "wg-background",
    renderer.terminalWidth,
    bgHeight,
  );
  root.add(background);

  // Full-screen dimming scrim (same as opencode dialog backdrop: RGBA(0,0,0,150))
  scrim = new Box(renderer, {
    id: "wg-scrim",
    position: "absolute",
    left: 0,
    top: HEADER_HEIGHT,
    width: renderer.terminalWidth,
    height: bgHeight,
    backgroundColor: RGBA.fromInts(0, 0, 0, 150),
    zIndex: 50,
  });
  scrim.visible = false;
  root.add(scrim);

  // Draggable boxes at various alpha levels
  const box1 = new DraggableBox(
    renderer,
    "wg-box-50",
    4,
    HEADER_HEIGHT + 1,
    25,
    8,
    RGBA.fromValues(64 / 255, 176 / 255, 255 / 255, 128 / 255),
    100,
  );
  root.add(box1);
  draggableBoxes.push(box1);

  const box2 = new DraggableBox(
    renderer,
    "wg-box-75",
    20,
    HEADER_HEIGHT + 5,
    25,
    8,
    RGBA.fromValues(255 / 255, 107 / 255, 129 / 255, 192 / 255),
    100,
  );
  root.add(box2);
  draggableBoxes.push(box2);

  const box3 = new DraggableBox(
    renderer,
    "wg-box-25",
    40,
    HEADER_HEIGHT + 3,
    25,
    8,
    RGBA.fromValues(139 / 255, 69 / 255, 193 / 255, 64 / 255),
    100,
  );
  root.add(box3);
  draggableBoxes.push(box3);

  const box4 = new DraggableBox(
    renderer,
    "wg-box-opaque",
    60,
    HEADER_HEIGHT + 7,
    25,
    8,
    RGBA.fromValues(30 / 255, 30 / 255, 42 / 255, 1.0),
    100,
  );
  root.add(box4);
  draggableBoxes.push(box4);

  renderer.keyInput.on("keypress", (key: KeyEvent) => {
    if (key.name === "d") {
      key.preventDefault();
      toggleScrim(renderer);
    }
  });

  renderer.on("resize", (width: number, height: number) => {
    const h = height - HEADER_HEIGHT;
    background.width = width;
    background.height = h;
    if (scrim) {
      scrim.width = width;
      scrim.height = h;
    }
    renderer.requestRender();
  });
}

export function destroy(renderer: CliRenderer): void {
  renderer.clearFrameCallbacks();

  for (const box of draggableBoxes) {
    renderer.root.remove(box);
  }
  draggableBoxes = [];
  nextZIndex = 101;
  scrimVisible = false;
  scrim = null;
  headerDisplay = null;

  const root = renderer.root.getRenderable("wg-overlay-root");
  if (root) renderer.root.remove(root);
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
  renderer.start();
}
