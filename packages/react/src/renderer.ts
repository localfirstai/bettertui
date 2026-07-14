import type { Command, CommandBufferConsumer, Instance, TextInstance } from "@bettertui/core";
import { generateId } from "@bettertui/core";
import type { LayoutConstraints, Style } from "@bettertui/shared";
import { createContext } from "react";
import Reconciler from "react-reconciler";
import { DefaultEventPriority, NoEventPriority } from "react-reconciler/constants";

// Module-level update priority state (required by react-reconciler@0.31+)
let currentUpdatePriority = NoEventPriority;

export interface Container {
  id: string;
  children: Array<Instance | TextInstance>;
  buffer: CommandBufferConsumer;
  onCommit?: () => void;
}

// ─── Layout Props → Command Mapping ──────────────────────────────────────────

const LAYOUT_PROPS = new Set([
  "flexDirection",
  "justifyContent",
  "alignItems",
  "alignSelf",
  "flexGrow",
  "flexShrink",
  "flexBasis",
  "flexWrap",
  "position",
  "padding",
  "paddingX",
  "paddingY",
  "paddingTop",
  "paddingRight",
  "paddingBottom",
  "paddingLeft",
  "margin",
  "marginX",
  "marginY",
  "marginTop",
  "marginRight",
  "marginBottom",
  "marginLeft",
  "gap",
  "width",
  "height",
  "minWidth",
  "minHeight",
  "maxWidth",
  "maxHeight",
  "top",
  "right",
  "bottom",
  "left",
  "overflow",
  "opacity",
  "zIndex",
]);

const STYLE_PROPS = new Set([
  "color",
  "bgColor",
  "bold",
  "italic",
  "underline",
  "dim",
  "strikethrough",
  "inverse",
  "hidden",
  "blink",
]);

function extractLayoutCommands(id: string, props: Record<string, unknown>): Command[] {
  const commands: Command[] = [];

  if (props["flexDirection"] !== undefined) {
    commands.push({ type: "SetFlexDirection", id, direction: props["flexDirection"] });
  }
  if (props["justifyContent"] !== undefined) {
    commands.push({ type: "SetJustifyContent", id, value: props["justifyContent"] });
  }
  if (props["alignItems"] !== undefined) {
    commands.push({ type: "SetAlignItems", id, value: props["alignItems"] });
  }
  if (props["alignSelf"] !== undefined) {
    commands.push({ type: "SetAlignSelf", id, value: props["alignSelf"] });
  }
  if (props["flexGrow"] !== undefined) {
    commands.push({ type: "SetFlexGrow", id, value: props["flexGrow"] });
  }
  if (props["flexShrink"] !== undefined) {
    commands.push({ type: "SetFlexShrink", id, value: props["flexShrink"] });
  }
  if (props["flexBasis"] !== undefined) {
    commands.push({ type: "SetFlexBasis", id, value: props["flexBasis"] });
  }
  if (props["position"] !== undefined) {
    commands.push({ type: "SetPosition", id, value: props["position"] });
  }
  if (props["width"] !== undefined) {
    commands.push({ type: "SetWidth", id, value: props["width"] });
  }
  if (props["height"] !== undefined) {
    commands.push({ type: "SetHeight", id, value: props["height"] });
  }
  if (props["minWidth"] !== undefined) {
    commands.push({ type: "SetMinWidth", id, value: props["minWidth"] });
  }
  if (props["minHeight"] !== undefined) {
    commands.push({ type: "SetMinHeight", id, value: props["minHeight"] });
  }
  if (props["maxWidth"] !== undefined) {
    commands.push({ type: "SetMaxWidth", id, value: props["maxWidth"] });
  }
  if (props["maxHeight"] !== undefined) {
    commands.push({ type: "SetMaxHeight", id, value: props["maxHeight"] });
  }
  if (props["overflow"] !== undefined) {
    commands.push({ type: "SetOverflow", id, value: props["overflow"] });
  }
  if (props["opacity"] !== undefined) {
    commands.push({ type: "SetOpacity", id, value: props["opacity"] });
  }
  if (props["zIndex"] !== undefined) {
    commands.push({ type: "SetZIndex", id, value: props["zIndex"] });
  }

  // Padding shortcuts
  if (props["padding"] !== undefined) {
    const p = props["padding"];
    commands.push({ type: "SetPadding", id, value: typeof p === "number" ? { all: p } : p });
  }
  if (props["paddingX"] !== undefined) {
    commands.push({
      type: "SetPadding",
      id,
      value: { horizontal: props["paddingX"] },
    });
  }
  if (props["paddingY"] !== undefined) {
    commands.push({ type: "SetPadding", id, value: { vertical: props["paddingY"] } });
  }
  if (props["paddingTop"] !== undefined) {
    commands.push({ type: "SetPadding", id, value: { top: props["paddingTop"] } });
  }
  if (props["paddingRight"] !== undefined) {
    commands.push({ type: "SetPadding", id, value: { right: props["paddingRight"] } });
  }
  if (props["paddingBottom"] !== undefined) {
    commands.push({
      type: "SetPadding",
      id,
      value: { bottom: props["paddingBottom"] },
    });
  }
  if (props["paddingLeft"] !== undefined) {
    commands.push({ type: "SetPadding", id, value: { left: props["paddingLeft"] } });
  }

  // Margin shortcuts
  if (props["margin"] !== undefined) {
    const m = props["margin"];
    commands.push({ type: "SetMargin", id, value: typeof m === "number" ? { all: m } : m });
  }
  if (props["marginX"] !== undefined) {
    commands.push({ type: "SetMargin", id, value: { horizontal: props["marginX"] } });
  }
  if (props["marginY"] !== undefined) {
    commands.push({ type: "SetMargin", id, value: { vertical: props["marginY"] } });
  }
  if (props["marginTop"] !== undefined) {
    commands.push({ type: "SetMargin", id, value: { top: props["marginTop"] } });
  }
  if (props["marginRight"] !== undefined) {
    commands.push({ type: "SetMargin", id, value: { right: props["marginRight"] } });
  }
  if (props["marginBottom"] !== undefined) {
    commands.push({ type: "SetMargin", id, value: { bottom: props["marginBottom"] } });
  }
  if (props["marginLeft"] !== undefined) {
    commands.push({ type: "SetMargin", id, value: { left: props["marginLeft"] } });
  }

  // Gap
  if (props["gap"] !== undefined) {
    commands.push({ type: "SetGap", id, value: { width: props["gap"], height: props["gap"] } });
  }

  // Inset (top/right/bottom/left for absolute positioning)
  if (
    props["top"] !== undefined ||
    props["right"] !== undefined ||
    props["bottom"] !== undefined ||
    props["left"] !== undefined
  ) {
    commands.push({
      type: "SetInset",
      id,
      value: {
        top: props["top"],
        right: props["right"],
        bottom: props["bottom"],
        left: props["left"],
      },
    });
  }

  return commands;
}

function extractStyleCommands(id: string, props: Record<string, unknown>): Command[] {
  const commands: Command[] = [];

  if (props["color"] !== undefined) {
    commands.push({ type: "SetForeground", id, color: props["color"] });
  }
  if (props["bgColor"] !== undefined) {
    commands.push({ type: "SetBackground", id, color: props["bgColor"] });
  }
  if (props["bold"] !== undefined) {
    commands.push({ type: "SetBold", id, value: props["bold"] });
  }
  if (props["italic"] !== undefined) {
    commands.push({ type: "SetItalic", id, value: props["italic"] });
  }
  if (props["underline"] !== undefined) {
    commands.push({ type: "SetUnderline", id, value: props["underline"] });
  }
  if (props["dim"] !== undefined) {
    commands.push({ type: "SetDim", id, value: props["dim"] });
  }
  if (props["strikethrough"] !== undefined) {
    commands.push({ type: "SetStrikethrough", id, value: props["strikethrough"] });
  }
  if (props["inverse"] !== undefined) {
    commands.push({ type: "SetInverse", id, value: props["inverse"] });
  }
  if (props["hidden"] !== undefined) {
    commands.push({ type: "SetHidden", id, value: props["hidden"] });
  }

  return commands;
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
  // Host config is inferred (not annotated) because the installed @types/react-reconciler@0.31
  // lags the runtime reconciler; some forward-compat methods (setCurrentUpdatePriority,
  // resolveUpdatePriority, shouldAttemptEagerTransition, NotPendingTransition) are valid at
  // runtime but absent from the type defs. Inference + the Reconciler() call below still
  // type-checks the required surface without rejecting the extra members.
  const hostConfig = {
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

      // Create node in Rust engine
      buffer.push({ type: "CreateNode", id, kind: type });

      // Special handling for Text nodes: if children is a string, set text directly
      // Text nodes in the Rust engine are leaf nodes (not containers)
      // Also set width based on text length for proper layout
      if (type === "Text" && typeof children === "string") {
        buffer.push({ type: "SetText", id, text: children });
        // Set width to text length so layout engine can position it correctly
        buffer.push({ type: "SetWidth", id, value: children.length });
      }

      // Forward style object if provided
      if (Object.keys(instance.style).length > 0) {
        buffer.push({ type: "SetStyle", id, style: instance.style });
      }

      // Extract and forward layout props as individual commands
      const layoutCommands = extractLayoutCommands(id, restProps);
      for (const cmd of layoutCommands) {
        buffer.push(cmd);
      }

      // Extract and forward style props (color, bold, italic, etc.)
      const styleCmds = extractStyleCommands(id, restProps);
      for (const cmd of styleCmds) {
        buffer.push(cmd);
      }

      // Forward remaining props as SetAttribute commands
      for (const [key, value] of Object.entries(restProps)) {
        if (!LAYOUT_PROPS.has(key) && !STYLE_PROPS.has(key) && value !== undefined) {
          buffer.push({
            type: "SetAttribute",
            id,
            key,
            value: typeof value === "string" ? value : JSON.stringify(value),
          });
        }
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

    shouldSetTextContent(type: string, _props: Record<string, unknown>): boolean {
      // Text nodes in the Rust engine are leaf nodes - they can't have children
      // So we tell React to set text content directly instead of creating TextInstances
      return type === "Text";
    },

    getRootHostContext(_rootContainer: Container): Record<string, unknown> {
      return { isInsideText: false };
    },

    getChildHostContext(
      parentHostContext: Record<string, unknown>,
      _type: string,
      _rootContainer: Container,
    ): Record<string, unknown> {
      return { ...parentHostContext };
    },

    getPublicInstance(instance: Instance | TextInstance): Instance {
      return instance as Instance;
    },

    // biome-ignore lint/suspicious/noExplicitAny: react-reconciler API contract
    prepareForCommit(_containerInfo: Container): Record<string, any> | null {
      return null;
    },

    resetAfterCommit(container: Container): void {
      container.onCommit?.();
    },

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
      // Update instance props
      Object.assign(instance.props, updatePayload);

      // Handle style object update
      if (updatePayload["__style"]) {
        instance.style = updatePayload["__style"] as Style;
        buffer.push({ type: "SetStyle", id: instance.id, style: instance.style });
      }

      // Extract and forward layout prop changes
      const layoutCommands = extractLayoutCommands(instance.id, updatePayload);
      for (const cmd of layoutCommands) {
        buffer.push(cmd);
      }

      const styleCommands = extractStyleCommands(instance.id, updatePayload);
      for (const cmd of styleCommands) {
        buffer.push(cmd);
      }

      // Forward remaining changed props as SetAttribute commands
      for (const [key, value] of Object.entries(updatePayload)) {
        if (
          key !== "__style" &&
          !LAYOUT_PROPS.has(key) &&
          !STYLE_PROPS.has(key) &&
          value !== undefined
        ) {
          buffer.push({
            type: "SetAttribute",
            id: instance.id,
            key,
            value: typeof value === "string" ? value : JSON.stringify(value),
          });
        }
      }
    },

    prepareUpdate(
      _instance: Instance,
      _type: string,
      oldProps: Record<string, unknown>,
      newProps: Record<string, unknown>,
      _rootContainer: Container,
      _hostContext: Record<string, unknown>,
    ): Record<string, unknown> | null {
      const { children: _oldChildren, style: _oldStyle, layout: _oldLayout, ...oldRest } = oldProps;
      const { children: _newChildren, style: _newStyle, layout: _newLayout, ...newRest } = newProps;

      // Build diff of changed props
      const diff: Record<string, unknown> = {};
      const allKeys = new Set([...Object.keys(oldRest), ...Object.keys(newRest)]);
      for (const key of allKeys) {
        if (oldRest[key] !== newRest[key]) {
          diff[key] = newRest[key];
        }
      }

      // Include style if changed
      if (_newStyle !== _oldStyle) {
        diff["__style"] = _newStyle;
      }

      return Object.keys(diff).length > 0 ? diff : null;
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

    // ─── Update priority (required by react-reconciler@0.31+) ────────────────
    setCurrentUpdatePriority(newPriority: number): void {
      currentUpdatePriority = newPriority;
    },

    getCurrentUpdatePriority(): number {
      return currentUpdatePriority;
    },

    resolveUpdatePriority(): number {
      if (currentUpdatePriority !== NoEventPriority) {
        return currentUpdatePriority;
      }
      return DefaultEventPriority;
    },

    // ─── Transition config ───────────────────────────────────────────────────
    shouldAttemptEagerTransition(): boolean {
      return false;
    },

    NotPendingTransition: null,

    HostTransitionContext: createContext(null),

    // ─── Suspend config ─────────────────────────────────────────────────────
    maySuspendCommit(): boolean {
      return false;
    },

    maySuspendCommitOnUpdate(): boolean {
      return false;
    },

    maySuspendCommitInSyncRender(): boolean {
      return false;
    },

    preloadInstance(): boolean {
      return true;
    },

    startSuspendingCommit(): void {},

    suspendInstance(): void {},

    waitForCommitToBeReady(): null {
      return null;
    },

    // ─── Misc stubs ─────────────────────────────────────────────────────────
    resetFormInstance(): void {},

    requestPostPaintCallback(): void {},

    trackSchedulerEvent(): void {},

    resolveEventType(): null {
      return null;
    },

    resolveEventTimeStamp(): number {
      return -1;
    },
  };

  return Reconciler(hostConfig);
}

export function createContainer(
  reconciler: ReconcilerType,
  buffer: CommandBufferConsumer,
  options?: { id?: string; onCommit?: () => void },
): OpaqueRoot {
  const container: Container = {
    id: options?.id ?? generateId(),
    children: [],
    buffer,
    ...(options?.onCommit ? { onCommit: options.onCommit } : {}),
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
