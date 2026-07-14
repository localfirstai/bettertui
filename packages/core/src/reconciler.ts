import type { Style } from "@bettertui/shared";
import {
  appendChild,
  commitTextUpdate,
  commitUpdate,
  createInstance,
  createTextInstance,
  finalizeInitialChildren,
  insertBefore,
  prepareUpdate,
  removeChild,
  resetAfterCommit,
} from "./command";
import type { CommandBuffer, Instance, TextInstance } from "./command";

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
    emitCreateNode(instance.id, "Text");
    emitSetText(instance.id, text);
    return instance;
  }

  function wrappedAppendChild(parent: Instance, child: Instance | TextInstance): void {
    appendChild(parent, child);
    emitAppendChild(parent.id, child.id);
  }

  function wrappedRemoveChild(parent: Instance, child: Instance | TextInstance): void {
    removeChild(parent, child);
    emitRemoveNode(child.id);
  }

  function wrappedInsertBefore(
    parent: Instance,
    child: Instance | TextInstance,
    reference: Instance | TextInstance,
  ): void {
    insertBefore(parent, child, reference);
    emitInsertBefore(reference.id, child.id);
  }

  function wrappedCommitUpdate(instance: Instance, updatePayload: Record<string, unknown>): void {
    commitUpdate(instance, updatePayload);
    if (updatePayload["style"]) {
      emitSetStyle(instance.id, updatePayload["style"] as Style);
    }
  }

  function wrappedCommitTextUpdate(textInstance: TextInstance, text: string): void {
    commitTextUpdate(textInstance, text);
    if (textInstance.id) {
      emitSetText(textInstance.id, text);
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
