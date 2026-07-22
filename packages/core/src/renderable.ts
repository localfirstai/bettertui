import { generateId } from "@bettertui/shared";
import type { KeyEvent, MouseEvent } from "@bettertui/shared";
import type { Command, CommandBufferConsumer } from "./command/command.types";
import type { CliRenderer } from "./platform/cliRenderer";

export interface WidgetContext {
  buffer: CommandBufferConsumer;
  onKey?: (handler: (key: KeyEvent) => boolean) => void;
  offKey?: (handler: (key: KeyEvent) => boolean) => void;
}

export interface WidgetLifecycle {
  mount(ctx: WidgetContext): void;
  unmount(): void;
}

export interface ImperativeContext {
  renderer: CliRenderer;
  parentId: number;
}

export abstract class Renderable<TOptions = Record<string, unknown>> {
  readonly id: string;
  protected ctx: WidgetContext | null = null;
  protected opts: TOptions;
  protected children: Renderable[] = [];
  protected _focused = false;
  protected _visible = true;
  protected _isDestroyed = false;
  protected _nodeId: number | null = null;

  constructor(options: TOptions = {} as TOptions) {
    this.id = generateId();
    this.opts = { ...options };
  }

  get options(): Readonly<TOptions> {
    return this.opts;
  }

  get visible(): boolean {
    return this._visible;
  }

  get focused(): boolean {
    return this._focused;
  }

  get isDestroyed(): boolean {
    return this._isDestroyed;
  }

  get nodeId(): number | null {
    return this._nodeId;
  }

  mount(ctx: WidgetContext): void {
    this.ctx = ctx;
    for (const child of this.children) {
      child.mount(ctx);
    }
  }

  unmount(): void {
    for (const child of this.children) {
      child.unmount();
    }
    this.ctx = null;
  }

  update(options: Partial<TOptions>): void {
    this.opts = { ...this.opts, ...options } as TOptions;
  }

  abstract renderCommands(id: string): Command[];

  add(child: Renderable): void {
    this.children.push(child);
    if (this.ctx) {
      child.mount(this.ctx);
    }
  }

  remove(child: Renderable): void {
    const index = this.children.indexOf(child);
    if (index !== -1) {
      child.unmount();
      this.children.splice(index, 1);
    }
  }

  handleKey?(key: KeyEvent): boolean;
  handleMouse?(event: MouseEvent): boolean;
  handleFocus?(): void;
  handleBlur?(): void;

  focus(): void {
    this._focused = true;
    this.handleFocus?.();
  }

  blur(): void {
    this._focused = false;
    this.handleBlur?.();
  }

  destroy(): void {
    if (this._isDestroyed) return;
    this._isDestroyed = true;
    for (const child of [...this.children]) {
      child.destroy();
    }
    this.children = [];
    this.unmount();
  }

  protected emitCommands(cmds: Command[]): void {
    if (!this.ctx) return;
    for (const cmd of cmds) {
      this.ctx.buffer.push(cmd);
    }
  }

  renderImperative(ctx: ImperativeContext): number {
    const nodeId = ctx.renderer.createNode(this.getNodeKind());
    this._nodeId = nodeId;

    this.applyImperativeStyle(ctx.renderer, nodeId);
    this.applyImperativeLayout(ctx.renderer, nodeId);
    this.applyImperativeContent(ctx.renderer, nodeId);

    ctx.renderer.appendChild(ctx.parentId, nodeId);

    for (const child of this.children) {
      child.renderImperative({ renderer: ctx.renderer, parentId: nodeId });
    }

    return nodeId;
  }

  protected getNodeKind(): string {
    return "Box";
  }

  protected applyImperativeStyle(_renderer: CliRenderer, _nodeId: number): void {}
  protected applyImperativeLayout(_renderer: CliRenderer, _nodeId: number): void {}
  protected applyImperativeContent(_renderer: CliRenderer, _nodeId: number): void {}

  protected layoutCommands(id: string, layout: Record<string, unknown>): Command[] {
    const cmds: Command[] = [];
    for (const [key, value] of Object.entries(layout)) {
      if (value === undefined || value === null) continue;
      switch (key) {
        case "flexDirection":
          cmds.push({ type: "SetFlexDirection", id, direction: value as never });
          break;
        case "justifyContent":
          cmds.push({ type: "SetJustifyContent", id, value: value as never });
          break;
        case "alignItems":
          cmds.push({ type: "SetAlignItems", id, value: value as never });
          break;
        case "alignSelf":
          cmds.push({ type: "SetAlignSelf", id, value: value as never });
          break;
        case "flexGrow":
          cmds.push({ type: "SetFlexGrow", id, value: value as number });
          break;
        case "flexShrink":
          cmds.push({ type: "SetFlexShrink", id, value: value as number });
          break;
        case "flexBasis":
          cmds.push({ type: "SetFlexBasis", id, value: value as never });
          break;
        case "position":
          cmds.push({ type: "SetPosition", id, value: value as never });
          break;
        case "width":
          cmds.push({ type: "SetWidth", id, value: value as never });
          break;
        case "height":
          cmds.push({ type: "SetHeight", id, value: value as never });
          break;
        case "minWidth":
          cmds.push({ type: "SetMinWidth", id, value: value as never });
          break;
        case "maxWidth":
          cmds.push({ type: "SetMaxWidth", id, value: value as never });
          break;
        case "minHeight":
          cmds.push({ type: "SetMinHeight", id, value: value as never });
          break;
        case "maxHeight":
          cmds.push({ type: "SetMaxHeight", id, value: value as never });
          break;
        case "overflow":
          cmds.push({ type: "SetOverflow", id, value: value as never });
          break;
        case "opacity":
          cmds.push({ type: "SetOpacity", id, value: value as number });
          break;
        case "zIndex":
          cmds.push({ type: "SetZIndex", id, value: value as number });
          break;
        case "padding":
        case "paddingTop":
        case "paddingRight":
        case "paddingBottom":
        case "paddingLeft":
          cmds.push({ type: "SetPadding", id, value: value as never });
          break;
        case "margin":
        case "marginTop":
        case "marginRight":
        case "marginBottom":
        case "marginLeft":
          cmds.push({ type: "SetMargin", id, value: value as never });
          break;
        case "gap":
          cmds.push({ type: "SetGap", id, value: value as never });
          break;
        case "inset":
        case "top":
        case "right":
        case "bottom":
        case "left":
          cmds.push({ type: "SetInset", id, value: value as never });
          break;
      }
    }
    return cmds;
  }

  protected styleCommands(id: string, style: Record<string, unknown>): Command[] {
    const cmds: Command[] = [];
    for (const [key, value] of Object.entries(style)) {
      if (value === undefined || value === null) continue;
      switch (key) {
        case "color":
        case "fg":
          cmds.push({ type: "SetForeground", id, color: value as never });
          break;
        case "bg":
        case "bgColor":
        case "backgroundColor":
          cmds.push({ type: "SetBackground", id, color: value as never });
          break;
        case "bold":
          cmds.push({ type: "SetBold", id, value: value as boolean });
          break;
        case "italic":
          cmds.push({ type: "SetItalic", id, value: value as boolean });
          break;
        case "underline":
          cmds.push({ type: "SetUnderline", id, value: value as boolean });
          break;
        case "dim":
          cmds.push({ type: "SetDim", id, value: value as boolean });
          break;
        case "strikethrough":
          cmds.push({ type: "SetStrikethrough", id, value: value as boolean });
          break;
        case "inverse":
          cmds.push({ type: "SetInverse", id, value: value as boolean });
          break;
        case "hidden":
          cmds.push({ type: "SetHidden", id, value: value as boolean });
          break;
        case "blink":
          cmds.push({ type: "SetBlink", id, value: value as boolean });
          break;
      }
    }
    return cmds;
  }
}

export type { KeyEvent, MouseEvent };
