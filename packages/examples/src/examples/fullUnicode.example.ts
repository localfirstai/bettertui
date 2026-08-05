import {
  Box,
  type CliRenderer,
  FrameBuffer,
  type KeyEvent,
  RGBA,
  Text,
  blue,
  bold,
  createCliRenderer,
  fg,
  t,
  underline,
} from "@bettertui/core";

type FullUnicodeDemoState = {
  keyHandler: (key: KeyEvent) => void;
};

let demoState: FullUnicodeDemoState | null = null;

const GRAPHEME_LINES: string[] = [
  "✅ 👩🏽‍💻  👨‍👩‍👧‍👦  🏳️‍🌈  🇺🇸  🇩🇪  🇯🇵  🇮🇳",
  "a̐éö̲  Z͑͗͛̒͘a̴͈͚̐̓l̷͓̱͉g̶̙̗̓͘o̵͍͈  क्‍ष",
  "مرحبا  こんにちは  สวัสดี  Здравствуйте",
  "𝔘𝔫𝔦𝔠𝔬𝔡𝔢  𝒻𝓊𝓁𝓁 𝓌𝒾𝒹𝓉𝒽：ＡＢＣ  ½ ⅞ ⅓",
];

class DraggableGraphemeBox extends FrameBuffer {
  private isDragging = false;
  private dragOffsetX = 0;
  private dragOffsetY = 0;

  constructor(
    ctx: CliRenderer,
    id: string,
    x: number,
    y: number,
    width: number,
    height: number,
    bg: RGBA,
    _respectAlpha = true,
  ) {
    super(ctx, {
      id,
      width,
      height,
      position: "absolute",
      left: x,
      top: y,
    });

    // Fill the internal framebuffer with graphemes
    this.frameBuffer.clear(RGBA.fromInts(0, 0, 0, Math.round(bg.a * 255)));

    const fgColor = RGBA.fromInts(255, 255, 255, 255);
    let row = 0;
    for (const line of GRAPHEME_LINES) {
      if (row >= height) break;
      this.frameBuffer.drawText(line, 1, row, fgColor, bg);
      row += 1;
    }

    // Wire up mouse handlers after super()
    this._options.onMouseDown = (e: unknown) => this._onMouse(e, "down");
    this._options.onMouseDrag = (e: unknown) => this._onMouse(e, "drag");
    this._options.onMouseDragEnd = (e: unknown) => this._onMouse(e, "drag-end");
  }

  private _onMouse(event: unknown, type: string): void {
    const e = event as { x?: number; y?: number; stopPropagation?: () => void };
    const ex = (e.x ?? 0) as number;
    const ey = (e.y ?? 0) as number;

    switch (type) {
      case "down":
        this.isDragging = true;
        this.dragOffsetX = ex - this.x;
        this.dragOffsetY = ey - this.y;
        this._renderer.requestRender();
        e.stopPropagation?.();
        break;
      case "drag":
        if (this.isDragging) {
          this.setPosition({
            left: ex - this.dragOffsetX,
            top: ey - this.dragOffsetY,
          });
          this._renderer.requestRender();
          e.stopPropagation?.();
        }
        break;
      case "drag-end":
        if (this.isDragging) {
          this.isDragging = false;
          this._renderer.requestRender();
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
      top: 0,
    });

    // Fill entire background with repeating grapheme lines
    const fgColor = RGBA.fromInts(220, 220, 220, 255);
    const bgColor = RGBA.fromInts(0, 17, 34, 255);
    this.frameBuffer.clear(RGBA.fromInts(0, 17, 34, 255));
    for (let y = 0; y < height; y++) {
      const line = GRAPHEME_LINES[y % GRAPHEME_LINES.length];
      this.frameBuffer.drawText(line, 2, y, fgColor, bgColor);
    }
  }
}

class DraggableStyledText extends Text {
  private isDragging = false;
  private dragOffsetX = 0;
  private dragOffsetY = 0;

  constructor(ctx: CliRenderer, id: string, x: number, y: number) {
    super(ctx, {
      id,
      position: "absolute",
      left: x,
      top: y,
      zIndex: 2,
    });

    // Styled text content with graphemes
    const content = t`${bold(blue("Graphemes:"))} ✅ 👩🏽‍💻  👨‍👩‍👧‍👦  🏳️‍🌈  🇺🇸  🇩🇪  🇯🇵  🇮🇳
${underline("Complex:")} a̐éö̲  Z͑͗͛̒͘a̴͈͚̐̓l̷͓̱͉g̶̙̗̓͘o̵͍͈  क्‍ष`;

    this.content = content;
    this.fg = RGBA.fromInts(255, 255, 255, 255);
    this.bg = RGBA.fromInts(0, 0, 0, 0);

    // Wire up mouse handlers after super()
    this._options.onMouseDown = (e: unknown) => this._onMouse(e, "down");
    this._options.onMouseDrag = (e: unknown) => this._onMouse(e, "drag");
    this._options.onMouseDragEnd = (e: unknown) => this._onMouse(e, "drag-end");
  }

  private _onMouse(event: unknown, type: string): void {
    const e = event as { x?: number; y?: number; stopPropagation?: () => void };
    const ex = (e.x ?? 0) as number;
    const ey = (e.y ?? 0) as number;

    switch (type) {
      case "down":
        this.isDragging = true;
        this.dragOffsetX = ex - this.x;
        this.dragOffsetY = ey - this.y;
        this._renderer.requestRender();
        e.stopPropagation?.();
        break;
      case "drag":
        if (this.isDragging) {
          this.setPosition({ left: ex - this.dragOffsetX, top: ey - this.dragOffsetY });
          this._renderer.requestRender();
          e.stopPropagation?.();
        }
        break;
      case "drag-end":
        if (this.isDragging) {
          this.isDragging = false;
          this._renderer.requestRender();
          e.stopPropagation?.();
        }
        break;
    }
  }
}

export function run(renderer: CliRenderer): void {
  renderer.start();
  renderer.setBackgroundColor(RGBA.toHex(RGBA.fromInts(0, 17, 34, 255)));

  let vignetteEnabled = false;

  const rootGroup = new Box(renderer, {
    id: "full-unicode-root",
    zIndex: 1,
  });
  renderer.root.add(rootGroup);

  const bg = new GraphemeBackground(
    renderer,
    "grapheme-bg",
    renderer.terminalWidth,
    renderer.terminalHeight,
  );
  rootGroup.add(bg);

  const box1 = new DraggableGraphemeBox(
    renderer,
    "grapheme-box-1",
    6,
    4,
    30,
    6,
    RGBA.fromInts(32, 96, 192, 160),
    true,
  );
  const box2 = new DraggableGraphemeBox(
    renderer,
    "grapheme-box-2",
    24,
    10,
    28,
    6,
    RGBA.fromInts(192, 96, 128, 180),
    true,
  );
  const box3 = new DraggableGraphemeBox(
    renderer,
    "grapheme-box-3",
    42,
    7,
    26,
    6,
    RGBA.fromInts(64, 176, 96, 128),
    true,
  );

  rootGroup.add(box1);
  rootGroup.add(box2);
  rootGroup.add(box3);

  // Draggable styled text using Text (grapheme-aware via TextBuffer)
  const styledText = new DraggableStyledText(renderer, "draggable-styled-text", 8, 12);
  rootGroup.add(styledText);

  const styledText2 = new DraggableStyledText(renderer, "draggable-styled-text-2", 18, 16);
  styledText2.content = t`${bold(fg("#55FF55")("Emoji Check:"))} ✅ 👩🏽‍💻  👨‍👩‍👧‍👦  🏳️‍🌈
${underline("Drag me too:")} 🇺🇸  🇩🇪  🇯🇵  🇮🇳  a̐éö̲`;
  rootGroup.add(styledText2);

  const hintText = new Text(renderer, {
    id: "full-unicode-hint",
    position: "absolute",
    left: 2,
    top: 1,
    zIndex: 3,
    content: "V: Toggle vignette",
    fg: "#AAFFAA",
  });
  rootGroup.add(hintText);

  const keyHandler = (key: KeyEvent): void => {
    if (key.name?.toLowerCase() !== "v") return;
    vignetteEnabled = !vignetteEnabled;
    hintText.content = `V: Toggle vignette (${vignetteEnabled ? "ON" : "OFF"})`;
    // Note: clearPostProcessFns/addPostProcessFn not available in current API
    renderer.requestRender();
  };

  renderer.keyInput.on("keypress", keyHandler);

  demoState = {
    keyHandler,
  };

  renderer.requestRender();
}

export function destroy(renderer: CliRenderer): void {
  if (demoState) {
    renderer.keyInput.off("keypress", demoState.keyHandler);
  }
  // Note: clearPostProcessFns not available in current API
  const root = renderer.root.getRenderable("full-unicode-root");
  if (root) renderer.root.remove(root);
  demoState = null;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({ exitOnCtrlC: true });
  run(renderer);
}
