import type { Style } from "@bettertui/shared";
import {
  appendChild,
  commitTextUpdate,
  commitUpdate,
  createInstance,
  createTextInstance,
  finalizeInitialChildren,
  generateId,
  insertBefore,
  prepareUpdate,
  removeChild,
  resetAfterCommit,
} from "./command-buffer";
import type { CommandBuffer, Instance, TextInstance } from "./command-buffer";

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
    if (updatePayload["style"]) {
      emitSetStyle(instance.id, updatePayload["style"] as Style);
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
