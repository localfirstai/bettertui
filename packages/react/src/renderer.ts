import type { CommandBufferConsumer, Instance, TextInstance } from "@bettertui/core";
import type { LayoutConstraints, Style } from "@bettertui/shared";
import Reconciler from "react-reconciler";
import { DefaultEventPriority } from "react-reconciler/constants";

let nextId = 0;
function generateId(): string {
  return `${nextId++}`;
}

export interface Container {
  id: string;
  children: Array<Instance | TextInstance>;
  buffer: CommandBufferConsumer;
}

export type ReconcilerType = Reconciler.Reconciler<
  Container,
  Instance,
  TextInstance,
  Instance,
  Instance
>;

// biome-ignore lint/suspicious/noExplicitAny: opaque react-reconciler root type
export type OpaqueRoot = any;

export function createBetterTUIReconciler(buffer: CommandBufferConsumer): ReconcilerType {
  // biome-ignore format: host config is complex
  const hostConfig: Reconciler.HostConfig<
    string,
    Record<string, unknown>,
    Container,
    Instance,
    TextInstance,
    Instance,
    Instance,
    Instance,
    Record<string, unknown>,
    Record<string, unknown>,
    Instance[],
    number,
    number
  > = {
    supportsMutation: true,
    supportsPersistence: false,
    supportsHydration: false,
    isPrimaryRenderer: true,
    noTimeout: -1,

    createInstance(
      type: string,
      props: Record<string, unknown>,
      _rootContainer: Container,
      _hostContext: Record<string, unknown>,
      // biome-ignore lint/suspicious/noExplicitAny: react-reconciler OpaqueHandle
      _internalHandle: any,
    ): Instance {
      const id = generateId();
      const { children, style, layout, ...restProps } = props;
      const instance: Instance = {
        id,
        type,
        props: restProps,
        style: (style as Style) || {},
        layout: (layout as LayoutConstraints) || {},
        children: [],
        parent: null,
      };
      buffer.push({ type: "CreateNode", id, kind: type });
      if (Object.keys(instance.style).length > 0) {
        buffer.push({ type: "SetStyle", id, style: instance.style });
      }
      return instance;
    },

    createTextInstance(
      text: string,
      _rootContainer: Container,
      _hostContext: Record<string, unknown>,
      // biome-ignore lint/suspicious/noExplicitAny: react-reconciler OpaqueHandle
      _internalHandle: any,
    ): TextInstance {
      const id = generateId();
      const instance = {
        type: "#text" as const,
        text,
        parent: null as Instance | null,
        id,
      };
      buffer.push({ type: "CreateNode", id, kind: "Text" });
      buffer.push({ type: "SetText", id, text });
      return instance;
    },

    appendInitialChild(parentInstance: Instance, child: Instance | TextInstance): void {
      const childInstance = child as Instance;
      childInstance.parent = parentInstance;
      if ("children" in parentInstance) {
        parentInstance.children.push(childInstance);
      }
      buffer.push({
        type: "AppendChild",
        parent: parentInstance.id,
        child: childInstance.id,
      });
    },

    finalizeInitialChildren(
      _instance: Instance,
      _type: string,
      _props: Record<string, unknown>,
      _rootContainer: Container,
      _hostContext: Record<string, unknown>,
    ): boolean {
      return false;
    },

    shouldSetTextContent(_type: string, _props: Record<string, unknown>): boolean {
      return false;
    },

    getRootHostContext(_rootContainer: Container): Record<string, unknown> | null {
      return null;
    },

    getChildHostContext(
      parentHostContext: Record<string, unknown>,
      _type: string,
      _rootContainer: Container,
    ): Record<string, unknown> {
      return parentHostContext;
    },

    getPublicInstance(instance: Instance | TextInstance): Instance {
      return instance as Instance;
    },

    // biome-ignore lint/suspicious/noExplicitAny: react-reconciler API contract
    prepareForCommit(_containerInfo: Container): Record<string, any> | null {
      return null;
    },

    resetAfterCommit(_containerInfo: Container): void {},

    preparePortalMount(_containerInfo: Container): void {},

    scheduleTimeout(fn: (...args: unknown[]) => unknown, delay?: number): number {
      return setTimeout(fn, delay) as unknown as number;
    },

    cancelTimeout(id: number): void {
      clearTimeout(id);
    },

    getCurrentEventPriority(): number {
      return DefaultEventPriority;
    },

    // biome-ignore lint/suspicious/noExplicitAny: react-reconciler API contract
    getInstanceFromNode(_node: any): any {
      return null;
    },

    beforeActiveInstanceBlur(): void {},

    afterActiveInstanceBlur(): void {},

    // biome-ignore lint/suspicious/noExplicitAny: react-reconciler API contract
    prepareScopeUpdate(_scopeInstance: any, _instance: any): void {},

    // biome-ignore lint/suspicious/noExplicitAny: react-reconciler API contract
    getInstanceFromScope(_scopeInstance: any): null | Instance {
      return null;
    },

    detachDeletedInstance(_node: Instance): void {},

    appendChild(parentInstance: Instance, child: Instance | TextInstance): void {
      const childInstance = child as Instance;
      childInstance.parent = parentInstance;
      if ("children" in parentInstance) {
        parentInstance.children.push(childInstance);
      }
      buffer.push({
        type: "AppendChild",
        parent: parentInstance.id,
        child: childInstance.id,
      });
    },

    appendChildToContainer(container: Container, child: Instance | TextInstance): void {
      const childInstance = child as Instance;
      childInstance.parent = null;
      container.children.push(childInstance);
      buffer.push({
        type: "AppendChild",
        parent: container.id,
        child: childInstance.id,
      });
    },

    insertBefore(
      parentInstance: Instance,
      child: Instance | TextInstance,
      beforeChild: Instance | TextInstance,
    ): void {
      const childInstance = child as Instance;
      const beforeInstance = beforeChild as Instance;
      childInstance.parent = parentInstance;
      if ("children" in parentInstance) {
        const index = parentInstance.children.indexOf(beforeInstance);
        if (index !== -1) {
          parentInstance.children.splice(index, 0, childInstance);
        } else {
          parentInstance.children.push(childInstance);
        }
      }
      buffer.push({
        type: "InsertBefore",
        reference: beforeInstance.id,
        child: childInstance.id,
      });
    },

    insertInContainerBefore(
      container: Container,
      child: Instance | TextInstance,
      beforeChild: Instance | TextInstance,
    ): void {
      const childInstance = child as Instance;
      const beforeInstance = beforeChild as Instance;
      childInstance.parent = null;
      const index = container.children.indexOf(beforeInstance);
      if (index !== -1) {
        container.children.splice(index, 0, childInstance);
      } else {
        container.children.push(childInstance);
      }
      buffer.push({
        type: "InsertBefore",
        reference: beforeInstance.id,
        child: childInstance.id,
      });
    },

    removeChild(parentInstance: Instance, child: Instance | TextInstance | Instance): void {
      const childInstance = child as Instance;
      childInstance.parent = null;
      if ("children" in parentInstance) {
        const index = parentInstance.children.indexOf(childInstance);
        if (index !== -1) {
          parentInstance.children.splice(index, 1);
        }
      }
      buffer.push({ type: "RemoveNode", id: childInstance.id });
    },

    removeChildFromContainer(
      container: Container,
      child: Instance | TextInstance | Instance,
    ): void {
      const childInstance = child as Instance;
      childInstance.parent = null;
      const index = container.children.indexOf(childInstance);
      if (index !== -1) {
        container.children.splice(index, 1);
      }
      buffer.push({ type: "RemoveNode", id: childInstance.id });
    },

    commitTextUpdate(textInstance: TextInstance, _oldText: string, newText: string): void {
      textInstance.text = newText;
      // biome-ignore lint/suspicious/noExplicitAny: TextInstance doesn't have id on the type
      const id = (textInstance as any).id;
      if (id) {
        buffer.push({ type: "SetText", id, text: newText });
      }
    },

    commitUpdate(
      instance: Instance,
      updatePayload: Record<string, unknown>,
      _type: string,
      _prevProps: Record<string, unknown>,
      _nextProps: Record<string, unknown>,
      // biome-ignore lint/suspicious/noExplicitAny: react-reconciler OpaqueHandle
      _internalHandle: any,
    ): void {
      Object.assign(instance.props, updatePayload);
      if (updatePayload.style) {
        buffer.push({
          type: "SetStyle",
          id: instance.id,
          style: updatePayload.style as Style,
        });
      }
    },

    prepareUpdate(
      _instance: Instance,
      _type: string,
      _oldProps: Record<string, unknown>,
      newProps: Record<string, unknown>,
      _rootContainer: Container,
      _hostContext: Record<string, unknown>,
    ): Record<string, unknown> | null {
      const { children, style, layout, ...restProps } = newProps;
      return restProps;
    },

    hideInstance(_instance: Instance): void {},

    hideTextInstance(_textInstance: TextInstance): void {},

    unhideInstance(_instance: Instance, _props: Record<string, unknown>): void {},

    unhideTextInstance(_textInstance: TextInstance, _text: string): void {},

    clearContainer(container: Container): void {
      container.children = [];
    },

    supportsMicrotasks: true,
    scheduleMicrotask(fn: () => unknown): void {
      queueMicrotask(fn);
    },
  };

  return Reconciler(hostConfig);
}

export function createContainer(
  reconciler: ReconcilerType,
  buffer: CommandBufferConsumer,
): OpaqueRoot {
  const container: Container = {
    id: generateId(),
    children: [],
    buffer,
  };
  return reconciler.createContainer(
    container,
    0,
    null,
    false,
    null,
    "",
    (error: Error) => {
      console.error(error);
    },
    null,
  );
}

export function updateContainer(
  reconciler: ReconcilerType,
  element: React.ReactNode,
  container: OpaqueRoot,
  callback?: () => void,
): void {
  reconciler.updateContainer(element, container, null, callback);
}
