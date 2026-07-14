import type { LayoutConstraints, Style } from "@bettertui/shared";
import { generateId } from "@bettertui/shared";

export type HostContext = Record<string, unknown>;

export interface Instance {
  id: string;
  type: string;
  props: Record<string, unknown>;
  style: Style;
  layout: LayoutConstraints;
  children: Instance[];
  parent: Instance | null;
}

export interface TextInstance {
  type: "#text";
  text: string;
  parent: Instance | null;
}

export type HostConfig = {
  type: string;
  props: Record<string, unknown>;
  container: Instance;
  instance: Instance;
  textInstance: TextInstance;
  suspenseInstance: Instance;
  hydratableInstance: Instance;
  publicInstance: Instance;
  hostContext: HostContext;
  updatePayload: Record<string, unknown>;
  childSet: Instance[];
  timeoutHandle: number;
  cornerstoneTimeoutHandle: number;
};

export type Command =
  | { type: "CreateNode"; id: string; kind: string }
  | { type: "RemoveNode"; id: string }
  | { type: "AppendChild"; parent: string; child: string }
  | { type: "InsertBefore"; reference: string; child: string }
  | { type: "MoveNode"; node: string; newParent: string }
  | { type: "ReplaceNode"; old: string; new: string }
  | { type: "DetachNode"; id: string }
  | { type: "SetText"; id: string; text: string }
  | { type: "SetStyle"; id: string; style: Style }
  | { type: "SetLayout"; id: string; layout: LayoutConstraints }
  | { type: "SetAttribute"; id: string; key: string; value: string }
  | { type: "RemoveAttribute"; id: string; key: string }
  | { type: "BeginFrame"; frameId: number }
  | { type: "CommitFrame"; frameId: number }
  | { type: "Invalidate"; id: string }
  | { type: "Shutdown" }
  | { type: "SetFlexDirection"; id: string; direction: unknown }
  | { type: "SetJustifyContent"; id: string; value: unknown }
  | { type: "SetAlignItems"; id: string; value: unknown }
  | { type: "SetAlignSelf"; id: string; value: unknown }
  | { type: "SetFlexGrow"; id: string; value: unknown }
  | { type: "SetFlexShrink"; id: string; value: unknown }
  | { type: "SetFlexBasis"; id: string; value: unknown }
  | { type: "SetPosition"; id: string; value: unknown }
  | { type: "SetWidth"; id: string; value: unknown }
  | { type: "SetHeight"; id: string; value: unknown }
  | { type: "SetMinWidth"; id: string; value: unknown }
  | { type: "SetMaxWidth"; id: string; value: unknown }
  | { type: "SetMinHeight"; id: string; value: unknown }
  | { type: "SetMaxHeight"; id: string; value: unknown }
  | { type: "SetOverflow"; id: string; value: unknown }
  | { type: "SetOpacity"; id: string; value: unknown }
  | { type: "SetZIndex"; id: string; value: unknown }
  | { type: "SetPadding"; id: string; value: unknown }
  | { type: "SetMargin"; id: string; value: unknown }
  | { type: "SetGap"; id: string; value: unknown }
  | { type: "SetInset"; id: string; value: unknown }
  | { type: "SetForeground"; id: string; color: unknown }
  | { type: "SetBackground"; id: string; color: unknown }
  | { type: "SetBold"; id: string; value: unknown }
  | { type: "SetItalic"; id: string; value: unknown }
  | { type: "SetUnderline"; id: string; value: unknown }
  | { type: "SetDim"; id: string; value: unknown }
  | { type: "SetStrikethrough"; id: string; value: unknown }
  | { type: "SetInverse"; id: string; value: unknown }
  | { type: "SetHidden"; id: string; value: unknown }
  | { type: "SetBlink"; id: string; value: unknown };

export interface CommandBufferConsumer {
  push(command: Command): void;
}

export class CommandBuffer {
  private commands: Command[] = [];

  push(command: Command): void {
    this.commands.push(command);
  }

  drain(): Command[] {
    const commands = this.commands;
    this.commands = [];
    return commands;
  }

  peek(): readonly Command[] {
    return this.commands;
  }

  clear(): void {
    this.commands = [];
  }

  get length(): number {
    return this.commands.length;
  }

  get isEmpty(): boolean {
    return this.commands.length === 0;
  }
}

export function createInstance(type: string, props: Record<string, unknown>): Instance {
  const id = generateId();
  const { children, style, layout, ...restProps } = props;

  return {
    id,
    type,
    props: restProps,
    style: (style as Style) || {},
    layout: (layout as LayoutConstraints) || {},
    children: [],
    parent: null,
  };
}

export function createTextInstance(text: string): TextInstance {
  return {
    type: "#text",
    text,
    parent: null,
  };
}

export function appendChild(parent: Instance, child: Instance | TextInstance): void {
  child.parent = parent;
  if ("children" in parent) {
    parent.children.push(child as Instance);
  }
}

export function removeChild(parent: Instance, child: Instance | TextInstance): void {
  child.parent = null;
  if ("children" in parent) {
    const index = parent.children.indexOf(child as Instance);
    if (index !== -1) {
      parent.children.splice(index, 1);
    }
  }
}

export function insertBefore(
  parent: Instance,
  child: Instance | TextInstance,
  reference: Instance | TextInstance,
): void {
  child.parent = parent;
  if ("children" in parent) {
    const index = parent.children.indexOf(reference as Instance);
    if (index !== -1) {
      parent.children.splice(index, 0, child as Instance);
    } else {
      parent.children.push(child as Instance);
    }
  }
}

export function prepareUpdate(
  _instance: Instance,
  _type: string,
  _oldProps: Record<string, unknown>,
  newProps: Record<string, unknown>,
): Record<string, unknown> | null {
  const { children, style, layout, ...restProps } = newProps;
  return restProps;
}

export function commitUpdate(instance: Instance, updatePayload: Record<string, unknown>): void {
  Object.assign(instance.props, updatePayload);
}

export function commitTextUpdate(textInstance: TextInstance, text: string): void {
  textInstance.text = text;
}

export function finalizeInitialChildren(_instance: Instance): boolean {
  return false;
}

export function resetAfterCommit(): void {
  // Flush happens at a higher level
}

export { generateId };
