import type { CliRenderer } from "@bettertui/core";
import type { LayoutConstraints, Style } from "@bettertui/shared";
import { createContext } from "react";
import type { ReactContext } from "react-reconciler";
import { DefaultEventPriority, NoEventPriority } from "react-reconciler/constants";
import type {
  BetterTUIContainer,
  BetterTUIInstance,
  BetterTUITextInstance,
} from "../types/host.types";

// ── Module-level update-priority tracker ─────────────────────────────────────
let _currentUpdatePriority = NoEventPriority;

let _nextId = 0;
function nextId(): string {
  return `btui-${_nextId++}`;
}

// ── Props → engine helpers ────────────────────────────────────────────────────

function applyStyle(renderer: CliRenderer, nativeId: number, props: Record<string, unknown>): void {
  const style: Style = {};
  let hasStyle = false;

  const styleProps: Array<keyof Style> = [
    "fg",
    "bg",
    "bold",
    "italic",
    "underline",
    "dim",
    "strikethrough",
    "inverse",
  ];
  for (const key of styleProps) {
    if (props[key] !== undefined) {
      (style as Record<string, unknown>)[key] = props[key];
      hasStyle = true;
    }
  }

  if (props.style && typeof props.style === "object") {
    Object.assign(style, props.style);
    hasStyle = true;
  }

  if (hasStyle) {
    renderer.setNodeStyle(nativeId, style);
  }
}

function applyLayout(
  renderer: CliRenderer,
  nativeId: number,
  props: Record<string, unknown>,
): void {
  const layout: LayoutConstraints = {};
  let hasLayout = false;

  const layoutKeys: Array<keyof LayoutConstraints> = [
    "flexDirection",
    "flexWrap",
    "justifyContent",
    "alignItems",
    "alignSelf",
    "flexGrow",
    "flexShrink",
    "flexBasis",
    "width",
    "height",
    "minWidth",
    "maxWidth",
    "minHeight",
    "maxHeight",
    "padding",
    "paddingTop",
    "paddingRight",
    "paddingBottom",
    "paddingLeft",
    "margin",
    "marginTop",
    "marginRight",
    "marginBottom",
    "marginLeft",
    "gap",
    "overflow",
    "position",
    "zIndex",
  ];
  for (const key of layoutKeys) {
    if (props[key] !== undefined) {
      (layout as Record<string, unknown>)[key] = props[key];
      hasLayout = true;
    }
  }

  if (props.layout && typeof props.layout === "object") {
    Object.assign(layout, props.layout);
    hasLayout = true;
  }

  if (hasLayout) {
    renderer.setNodeLayout(nativeId, layout);
  }
}

function applyProps(renderer: CliRenderer, nativeId: number, props: Record<string, unknown>): void {
  applyStyle(renderer, nativeId, props);
  applyLayout(renderer, nativeId, props);
}

// ── Host config factory ───────────────────────────────────────────────────────

/**
 * Build a react-reconciler host config object that drives a {@link CliRenderer}.
 * A new config is created per `createRoot` call so the renderer reference is
 * safely captured in closure without cross-root contamination.
 */
export function makeHostConfig(renderer: CliRenderer) {
  return {
    supportsMutation: true as const,
    supportsPersistence: false as const,
    supportsHydration: false as const,
    supportsMicrotasks: true as const,
    scheduleMicrotask: queueMicrotask,

    createInstance(type: string, props: Record<string, unknown>): BetterTUIInstance {
      const nativeId = renderer.createNode(type);
      const instance: BetterTUIInstance = {
        id: nextId(),
        type,
        props,
        children: [],
        parent: null,
        nativeId,
      };
      applyProps(renderer, nativeId, props);
      return instance;
    },

    createTextInstance(text: string): BetterTUITextInstance {
      const nativeId = renderer.createNode("text");
      renderer.setText(nativeId, text);
      return { id: nextId(), type: "#text", text, parent: null, nativeId };
    },

    appendChild(parent: BetterTUIInstance, child: BetterTUIInstance | BetterTUITextInstance): void {
      child.parent = parent;
      if ("children" in parent) parent.children.push(child as BetterTUIInstance);
      renderer.appendChild(parent.nativeId, child.nativeId);
    },

    appendInitialChild(
      parent: BetterTUIInstance,
      child: BetterTUIInstance | BetterTUITextInstance,
    ): void {
      child.parent = parent;
      if ("children" in parent) parent.children.push(child as BetterTUIInstance);
      renderer.appendChild(parent.nativeId, child.nativeId);
    },

    appendChildToContainer(
      container: BetterTUIContainer,
      child: BetterTUIInstance | BetterTUITextInstance,
    ): void {
      renderer.appendChild(container.rootNativeId, child.nativeId);
    },

    removeChild(parent: BetterTUIInstance, child: BetterTUIInstance | BetterTUITextInstance): void {
      child.parent = null;
      if ("children" in parent) {
        const idx = parent.children.indexOf(child as BetterTUIInstance);
        if (idx !== -1) parent.children.splice(idx, 1);
      }
      renderer.removeNode(child.nativeId);
    },

    removeChildFromContainer(
      _container: BetterTUIContainer,
      child: BetterTUIInstance | BetterTUITextInstance,
    ): void {
      renderer.removeNode(child.nativeId);
    },

    insertBefore(
      parent: BetterTUIInstance,
      child: BetterTUIInstance | BetterTUITextInstance,
      before: BetterTUIInstance | BetterTUITextInstance,
    ): void {
      child.parent = parent;
      const beforeIdx = parent.children.indexOf(before as BetterTUIInstance);
      const childIdx = parent.children.indexOf(child as BetterTUIInstance);
      if (childIdx !== -1) parent.children.splice(childIdx, 1);
      const insertAt = beforeIdx === -1 ? parent.children.length : beforeIdx;
      parent.children.splice(insertAt, 0, child as BetterTUIInstance);
      renderer.insertNodeBefore(parent.nativeId, child.nativeId, before.nativeId);
    },

    insertInContainerBefore(
      container: BetterTUIContainer,
      child: BetterTUIInstance | BetterTUITextInstance,
      before: BetterTUIInstance | BetterTUITextInstance,
    ): void {
      renderer.insertNodeBefore(container.rootNativeId, child.nativeId, before.nativeId);
    },

    clearContainer(container: BetterTUIContainer): void {
      for (const childId of [...renderer.getChildrenOf(container.rootNativeId)]) {
        renderer.removeNode(childId);
      }
    },

    prepareForCommit(): null {
      return null;
    },
    resetAfterCommit(): void {
      renderer.render();
    },
    finalizeInitialChildren(): boolean {
      return false;
    },
    commitMount(): void {},

    prepareUpdate(
      _instance: BetterTUIInstance,
      _type: string,
      _oldProps: Record<string, unknown>,
      newProps: Record<string, unknown>,
    ): Record<string, unknown> {
      return newProps;
    },

    commitUpdate(instance: BetterTUIInstance, updatePayload: Record<string, unknown>): void {
      instance.props = { ...instance.props, ...updatePayload };
      applyProps(renderer, instance.nativeId, updatePayload);
    },

    commitTextUpdate(textInstance: BetterTUITextInstance, _oldText: string, newText: string): void {
      textInstance.text = newText;
      renderer.setText(textInstance.nativeId, newText);
    },

    hideInstance(_instance: BetterTUIInstance): void {},
    unhideInstance(instance: BetterTUIInstance, props: Record<string, unknown>): void {
      applyStyle(renderer, instance.nativeId, props);
    },
    hideTextInstance(_textInstance: BetterTUITextInstance): void {},
    unhideTextInstance(textInstance: BetterTUITextInstance, text: string): void {
      renderer.setText(textInstance.nativeId, text);
    },

    getRootHostContext(): Record<string, unknown> {
      return {};
    },
    getChildHostContext(parentCtx: Record<string, unknown>): Record<string, unknown> {
      return parentCtx;
    },
    shouldSetTextContent(): boolean {
      return false;
    },

    scheduleTimeout: setTimeout,
    cancelTimeout: clearTimeout,
    noTimeout: -1 as const,
    isPrimaryRenderer: true as const,
    shouldAttemptEagerTransition(): boolean {
      return true;
    },

    setCurrentUpdatePriority(priority: number): void {
      _currentUpdatePriority = priority;
    },
    getCurrentUpdatePriority(): number {
      return _currentUpdatePriority;
    },
    resolveUpdatePriority(): number {
      return _currentUpdatePriority !== NoEventPriority
        ? _currentUpdatePriority
        : DefaultEventPriority;
    },

    maySuspendCommit(): boolean {
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

    NotPendingTransition: null,
    HostTransitionContext: createContext(null) as unknown as ReactContext<null>,
    resetFormInstance(): void {},
    requestPostPaintCallback(): void {},
    trackSchedulerEvent(): void {},
    resolveEventType(): null {
      return null;
    },
    resolveEventTimeStamp(): number {
      return -1.1;
    },

    detachDeletedInstance(_instance: BetterTUIInstance): void {},
    getPublicInstance(instance: BetterTUIInstance): BetterTUIInstance {
      return instance;
    },
    preparePortalMount(): void {},
    getInstanceFromNode(): null {
      return null;
    },
    beforeActiveInstanceBlur(): void {},
    afterActiveInstanceBlur(): void {},
    prepareScopeUpdate(): void {},
    getInstanceFromScope(): null {
      return null;
    },
  };
}
