#!/usr/bin/env bun

import {
  Box,
  type BoxOptions,
  type CliRenderer,
  FrameBuffer,
  type MouseEvent,
  RGBA,
  Text,
  createCliRenderer,
  createTimeline,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

interface TrailCell {
  x: number;
  y: number;
  timestamp: number;
  isDrag?: boolean;
}

let demoContainer: MouseInteractionFrameBuffer | null = null;
let titleText: Text | null = null;
let instructionsText: Text | null = null;
let draggableBoxes: Box[] = [];
let nextZIndex = 101;

function DraggableBox(
  rendererArg: CliRenderer,
  props: BoxOptions & {
    x: number;
    y: number;
    width: number;
    height: number;
    color: RGBA;
    label: string;
  },
  _children?: Box,
): Box {
  const bgColor = RGBA.fromValues(props.color.r, props.color.g, props.color.b, 0.8);
  const borderColor = RGBA.fromValues(
    Math.min(1, props.color.r * 1.2),
    Math.min(1, props.color.g * 1.2),
    Math.min(1, props.color.b * 1.2),
    1.0,
  );
  const dragBg = RGBA.fromValues(props.color.r, props.color.g, props.color.b, 0.3);
  const dragBorderColor = RGBA.fromValues(
    Math.min(1, props.color.r * 1.2),
    Math.min(1, props.color.g * 1.2),
    Math.min(1, props.color.b * 1.2),
    0.5,
  );
  const baseWidth = props.width;
  const baseHeight = props.height;
  const bounceScale = { value: 1 };

  let isDragging = false;
  let dragOffsetX = 0;
  let dragOffsetY = 0;

  const box = new Box(rendererArg, {
    ...props,
    position: "absolute",
    left: props.x,
    top: props.y,
    width: props.width,
    height: props.height,
    backgroundColor: bgColor,
    borderColor: borderColor,
    borderStyle: "round",
    title: props.label,
    titleAlignment: "center",
    border: true,
    zIndex: 100,
    onMouseDown: (event: unknown) => {
      const e = event as MouseEvent;
      isDragging = true;
      dragOffsetX = (e.position?.x ?? 0) - box.x;
      dragOffsetY = (e.position?.y ?? 0) - box.y;
      box.zIndex = nextZIndex++;
      box.backgroundColor = dragBg;
      box.borderColor = dragBorderColor;
    },
    onMouseDrag: (event: unknown) => {
      if (isDragging) {
        const e = event as MouseEvent;
        const newX = (e.position?.x ?? 0) - dragOffsetX;
        const newY = (e.position?.y ?? 0) - dragOffsetY;
        box.setPosition({ left: Math.max(0, newX), top: Math.max(4, newY) });
      }
    },
    onMouseDragEnd: (_event: unknown) => {
      if (isDragging) {
        isDragging = false;
        box.zIndex = 100;
        box.backgroundColor = bgColor;
        box.borderColor = borderColor;
      }
    },
    onMouseDrop: (_event: unknown) => {
      const timeline = createTimeline();
      timeline.add(bounceScale, {
        value: 1.5,
        duration: 200,
        ease: "outExpo",
        onUpdate: (values: { targets: Array<{ value: number }> }) => {
          const scale = values.targets[0]?.value ?? 1;
          box.width = Math.round(baseWidth * scale);
          box.height = Math.round(baseHeight * scale);
        },
      });
      timeline.add(
        bounceScale,
        {
          value: 1.0,
          duration: 400,
          ease: "outExpo",
          onUpdate: (values: { targets: Array<{ value: number }> }) => {
            const scale = values.targets[0]?.value ?? 1;
            box.width = Math.round(baseWidth * scale);
            box.height = Math.round(baseHeight * scale);
          },
        },
        200,
      );
    },
  });

  return box;
}

class MouseInteractionFrameBuffer extends FrameBuffer {
  private readonly trailCells = new Map<string, TrailCell>();
  private readonly activatedCells = new Set<string>();
  private readonly TRAIL_FADE_DURATION = 3000;

  private readonly TRAIL_COLOR = RGBA.fromInts(64, 224, 208, 255);
  private readonly DRAG_COLOR = RGBA.fromInts(255, 165, 0, 255);
  private readonly ACTIVATED_COLOR = RGBA.fromInts(255, 20, 147, 255);
  private readonly BACKGROUND_COLOR = RGBA.fromInts(15, 15, 35, 255);
  private readonly CURSOR_COLOR = RGBA.fromInts(255, 255, 255, 255);

  constructor(id: string, renderer: CliRenderer) {
    super(renderer, {
      id,
      width: renderer.terminalWidth,
      height: renderer.terminalHeight,
      zIndex: 0,
    });
  }

  renderSelf(_buffer: unknown): void {
    const currentTime = Date.now();

    this.frameBuffer.clear(this.BACKGROUND_COLOR);

    for (const [key, cell] of this.trailCells.entries()) {
      if (currentTime - cell.timestamp > this.TRAIL_FADE_DURATION) {
        this.trailCells.delete(key);
      }
    }

    for (const [, cell] of this.trailCells.entries()) {
      const age = currentTime - cell.timestamp;
      const fadeRatio = 1 - age / this.TRAIL_FADE_DURATION;

      if (fadeRatio > 0) {
        const baseColor = cell.isDrag ? this.DRAG_COLOR : this.TRAIL_COLOR;
        const smoothAlpha = fadeRatio;

        const fadedColor = RGBA.fromValues(baseColor.r, baseColor.g, baseColor.b, smoothAlpha);

        this.frameBuffer.setCell(cell.x, cell.y, "█", fadedColor, this.BACKGROUND_COLOR);
      }
    }

    for (const cellKey of this.activatedCells) {
      const [x, y] = cellKey.split(",").map(Number);

      this.frameBuffer.drawText("█", x ?? 0, y ?? 0, this.ACTIVATED_COLOR, this.BACKGROUND_COLOR);
    }

    const recentTrails = Array.from(this.trailCells.values())
      .filter((cell) => currentTime - cell.timestamp < 100)
      .sort((a, b) => b.timestamp - a.timestamp);

    if (recentTrails.length > 0) {
      const latest = recentTrails[0];
      if (latest) {
        this.frameBuffer.setCell(latest.x, latest.y, "+", this.CURSOR_COLOR, this.BACKGROUND_COLOR);
      }
    }

    // Flush frame buffer content
    this.draw(0);
  }

  onMouseEvent(event: unknown): void {
    const e = event as MouseEvent;
    const x = e.position?.x ?? 0;
    const y = e.position?.y ?? 0;
    const type = (e as { type?: string }).type;
    const cellKey = `${x},${y}`;

    switch (type) {
      case "move":
        this.trailCells.set(cellKey, {
          x,
          y,
          timestamp: Date.now(),
          isDrag: false,
        });
        this.renderSelf(null);
        break;

      case "drag":
        this.trailCells.set(cellKey, {
          x,
          y,
          timestamp: Date.now(),
          isDrag: true,
        });
        this.renderSelf(null);
        break;

      case "down":
        if (this.activatedCells.has(cellKey)) {
          this.activatedCells.delete(cellKey);
        } else {
          this.activatedCells.add(cellKey);
        }
        this.renderSelf(null);
        break;
    }
  }

  public clearState(): void {
    this.trailCells.clear();
    this.activatedCells.clear();
  }
}

export function run(renderer: CliRenderer): void {
  renderer.start();
  renderer.setBackgroundColor("#0f0f23");

  const mainGroup = new Box(renderer, {
    id: "mouse-demo-main-group",
    zIndex: 10,
  });
  renderer.root.add(mainGroup);

  titleText = new Text(renderer, {
    id: "mouse_demo_title",
    content: "Mouse Interaction Demo with Draggable Objects",
    width: "100%",
    position: "absolute",
    left: 2,
    top: 1,
    fg: RGBA.fromInts(72, 209, 204),
    zIndex: 1000,
  });
  mainGroup.add(titleText);

  instructionsText = new Text(renderer, {
    id: "mouse_demo_instructions",
    content: t`Drag boxes around • Move mouse: turquoise trails
Hold + move: orange drag trails • Click cells: toggle pink
Scroll on boxes: shows direction • Escape: menu`,
    position: "absolute",
    left: 2,
    top: 2,
    width: renderer.terminalWidth - 4,
    height: 3,
    fg: RGBA.fromInts(176, 196, 222),
    zIndex: 1000,
  });
  mainGroup.add(instructionsText);

  demoContainer = new MouseInteractionFrameBuffer("mouse-demo-buffer", renderer);
  mainGroup.add(demoContainer);

  draggableBoxes = [
    DraggableBox(renderer, {
      id: "drag-box-1",
      x: 10,
      y: 8,
      width: 20,
      height: 10,
      color: RGBA.fromInts(200, 100, 150),
      label: "Box 1",
    }),
    DraggableBox(renderer, {
      id: "drag-box-2",
      x: 30,
      y: 12,
      width: 18,
      height: 10,
      color: RGBA.fromInts(100, 200, 150),
      label: "Box 2",
    }),
    DraggableBox(renderer, {
      id: "drag-box-3",
      x: 50,
      y: 15,
      width: 20,
      height: 11,
      color: RGBA.fromInts(150, 150, 200),
      label: "Box 3",
    }),
    DraggableBox(renderer, {
      id: "drag-box-4",
      x: 15,
      y: 20,
      width: 18,
      height: 11,
      color: RGBA.fromInts(200, 200, 100),
      label: "O hidden",
      overflow: "hidden",
    }),
  ];

  for (const box of draggableBoxes) {
    mainGroup.add(box);
  }
}

export function destroy(renderer: CliRenderer): void {
  renderer.clearFrameCallbacks();
  renderer.root.getRenderable("mouse-demo-main-group")?.destroyRecursively();
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });
  run(renderer);
  setupCommonDemoKeys(renderer);
}
