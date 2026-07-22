/**
 * solid-js/universal RendererOptions implementation for @bettertui/solid.
 *
 * Bridges the Solid reactive diffing engine to a @bettertui/core CliRenderer.
 * Each renderer instance gets its own closure so multiple roots are fully
 * isolated (same pattern as @bettertui/react's makeHostConfig).
 */

import type { CliRenderer } from "@bettertui/core";
import type { LayoutConstraints, Style } from "@bettertui/shared";
import { createRenderer } from "solid-js/universal";
import { NO_NODE, createTreeState } from "./treeState";

// ── Prop helpers ──────────────────────────────────────────────────────────────

const STYLE_KEYS = new Set([
  "fg",
  "bg",
  "bold",
  "italic",
  "underline",
  "dim",
  "strikethrough",
  "inverse",
]);

const LAYOUT_KEYS = new Set([
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
]);

function applyStyleProp(renderer: CliRenderer, id: number, name: string, value: unknown): void {
  if (name === "style" && value && typeof value === "object") {
    renderer.setNodeStyle(id, value as Style);
    return;
  }
  if (STYLE_KEYS.has(name) && value !== undefined) {
    renderer.setNodeStyle(id, { [name]: value } as unknown as Style);
  }
}

function applyLayoutProp(renderer: CliRenderer, id: number, name: string, value: unknown): void {
  if (name === "layout" && value && typeof value === "object") {
    renderer.setNodeLayout(id, value as LayoutConstraints);
    return;
  }
  if (LAYOUT_KEYS.has(name) && value !== undefined) {
    renderer.setNodeLayout(id, { [name]: value } as unknown as LayoutConstraints);
  }
}

// ── Content props (non-style, non-layout) ─────────────────────────────────────

function applyContentProp(renderer: CliRenderer, id: number, name: string, value: unknown): void {
  // Text content
  if ((name === "content" || name === "text" || name === "value") && typeof value === "string") {
    renderer.setText(id, value);
    return;
  }
  // Callbacks and event handlers are stored on the JS-side widget objects;
  // the native engine does not need to know about them. Nothing to do here
  // until @bettertui/solid ships a full widget-event plumbing layer.
}

// ── Universal renderer factory ────────────────────────────────────────────────

/**
 * Create a `solid-js/universal` renderer bound to a specific `CliRenderer`.
 * Call this once per root; the returned `render()` function starts a reactive
 * tree and returns a cleanup/dispose function.
 */
export function makeUniversalRenderer(renderer: CliRenderer) {
  const tree = createTreeState();

  const {
    render,
    effect,
    memo,
    createComponent,
    createElement,
    createTextNode,
    insertNode,
    insert,
    spread,
    setProp,
    mergeProps,
    use,
  } = createRenderer<number>({
    // ── Node creation ─────────────────────────────────────────────────────

    createElement(type: string): number {
      return renderer.createNode(type);
    },

    createTextNode(value: string): number {
      const id = renderer.createNode("text");
      renderer.setText(id, value);
      tree.markTextNode(id);
      return id;
    },

    // ── Text update ───────────────────────────────────────────────────────

    replaceText(id: number, value: string): void {
      renderer.setText(id, value);
    },

    // ── Node classification ───────────────────────────────────────────────

    isTextNode(id: number): boolean {
      return tree.isTextNode(id);
    },

    // ── Prop application ─────────────────────────────────────────────────

    setProperty(id: number, name: string, value: unknown): void {
      applyStyleProp(renderer, id, name, value);
      applyLayoutProp(renderer, id, name, value);
      applyContentProp(renderer, id, name, value);
    },

    // ── Tree mutations ───────────────────────────────────────────────────

    /**
     * Insert `node` into `parent` immediately before `anchor`.
     * When `anchor` is undefined (solid passes undefined for append), append.
     */
    insertNode(parent: number, node: number, anchor: number | undefined): void {
      const before = anchor ?? NO_NODE;
      tree.insertChild(parent, node, before);
      if (before === NO_NODE) {
        renderer.appendChild(parent, node);
      } else {
        renderer.insertNodeBefore(parent, node, before);
      }
    },

    removeNode(id: number): void {
      tree.removeNode(id);
      renderer.removeNode(id);
    },

    // ── Tree traversal (required by solid-js/universal's diffing) ────────

    getParentNode(id: number): number {
      return tree.getParent(id);
    },

    getFirstChild(id: number): number {
      return tree.getFirstChild(id);
    },

    getNextSibling(id: number): number {
      return tree.getNextSibling(id);
    },
  });

  return {
    render,
    effect,
    memo,
    createComponent,
    createElement,
    createTextNode,
    insertNode,
    insert,
    spread,
    setProp,
    mergeProps,
    use,
  };
}
