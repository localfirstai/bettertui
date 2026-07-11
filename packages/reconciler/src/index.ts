import type { LayoutConstraints, Style } from "@bettertui/shared";

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
  | { type: "Shutdown" };

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

let nextId = 0;
function generateId(): string {
  return `${nextId++}`;
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

export function createReconciler(buffer: CommandBuffer): {
  createInstance: (type: string, props: Record<string, unknown>) => Instance;
  createTextInstance: (text: string) => TextInstance;
  appendChild: (parent: Instance, child: Instance | TextInstance) => void;
  removeChild: (parent: Instance, child: Instance | TextInstance) => void;
  insertBefore: (
    parent: Instance,
    child: Instance | TextInstance,
    reference: Instance | TextInstance,
  ) => void;
  prepareUpdate: (
    instance: Instance,
    type: string,
    oldProps: Record<string, unknown>,
    newProps: Record<string, unknown>,
  ) => Record<string, unknown> | null;
  commitUpdate: (instance: Instance, updatePayload: Record<string, unknown>) => void;
  commitTextUpdate: (textInstance: TextInstance, text: string) => void;
  finalizeInitialChildren: (instance: Instance) => boolean;
  resetAfterCommit: () => void;
} {
  function emitCreateNode(id: string, type: string): void {
    buffer.push({ type: "CreateNode", id, kind: type });
  }

  function emitAppendChild(parentId: string, childId: string): void {
    buffer.push({ type: "AppendChild", parent: parentId, child: childId });
  }

  function emitRemoveNode(id: string): void {
    buffer.push({ type: "RemoveNode", id });
  }

  function emitInsertBefore(referenceId: string, childId: string): void {
    buffer.push({ type: "InsertBefore", reference: referenceId, child: childId });
  }

  function emitSetText(id: string, text: string): void {
    buffer.push({ type: "SetText", id, text });
  }

  function emitSetStyle(id: string, style: Style): void {
    buffer.push({ type: "SetStyle", id, style });
  }

  function wrappedCreateInstance(type: string, props: Record<string, unknown>): Instance {
    const instance = createInstance(type, props);
    emitCreateNode(instance.id, type);
    if (Object.keys(instance.style).length > 0) {
      emitSetStyle(instance.id, instance.style);
    }
    return instance;
  }

  function wrappedCreateTextInstance(text: string): TextInstance {
    const instance = createTextInstance(text);
    const id = generateId();
    (instance as unknown as { id: string }).id = id;
    emitCreateNode(id, "Text");
    emitSetText(id, text);
    return instance;
  }

  function wrappedAppendChild(parent: Instance, child: Instance | TextInstance): void {
    appendChild(parent, child);
    const childId =
      "id" in child ? (child as Instance).id : (child as unknown as { id: string }).id;
    emitAppendChild(parent.id, childId);
  }

  function wrappedRemoveChild(parent: Instance, child: Instance | TextInstance): void {
    removeChild(parent, child);
    const childId =
      "id" in child ? (child as Instance).id : (child as unknown as { id: string }).id;
    emitRemoveNode(childId);
  }

  function wrappedInsertBefore(
    parent: Instance,
    child: Instance | TextInstance,
    reference: Instance | TextInstance,
  ): void {
    insertBefore(parent, child, reference);
    const childId =
      "id" in child ? (child as Instance).id : (child as unknown as { id: string }).id;
    const refId =
      "id" in reference ? (reference as Instance).id : (reference as unknown as { id: string }).id;
    emitInsertBefore(refId, childId);
  }

  function wrappedCommitUpdate(instance: Instance, updatePayload: Record<string, unknown>): void {
    commitUpdate(instance, updatePayload);
    if (updatePayload.style) {
      emitSetStyle(instance.id, updatePayload.style as Style);
    }
  }

  function wrappedCommitTextUpdate(textInstance: TextInstance, text: string): void {
    commitTextUpdate(textInstance, text);
    const id = (textInstance as unknown as { id: string }).id;
    if (id) {
      emitSetText(id, text);
    }
  }

  return {
    createInstance: wrappedCreateInstance,
    createTextInstance: wrappedCreateTextInstance,
    appendChild: wrappedAppendChild,
    removeChild: wrappedRemoveChild,
    insertBefore: wrappedInsertBefore,
    prepareUpdate,
    commitUpdate: wrappedCommitUpdate,
    commitTextUpdate: wrappedCommitTextUpdate,
    finalizeInitialChildren,
    resetAfterCommit,
  };
}

export type {
  Container,
  CommandBufferConsumer,
  ReconcilerType,
  OpaqueRoot,
} from "./renderer";
export {
  createBetterTUIReconciler,
  createContainer,
  updateContainer,
} from "./renderer";
