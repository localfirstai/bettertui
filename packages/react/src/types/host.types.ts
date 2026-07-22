import type { CliRenderer } from "@bettertui/core";

/**
 * A terminal UI node created by the React reconciler.
 * Extends the base structure with the native engine node ID.
 */
export interface BetterTUIInstance {
  /** Stable string ID used by the reconciler. */
  id: string;
  /** Type name of the node, e.g. "box", "text", "input". */
  type: string;
  /** Props last committed by React. */
  props: Record<string, unknown>;
  /** Direct children tracked by the reconciler. */
  children: BetterTUIInstance[];
  /** Parent instance, or null for root-level children. */
  parent: BetterTUIInstance | BetterTUIContainer | null;
  /** Native node ID in the Rust engine. */
  nativeId: number;
}

/**
 * A text-leaf node (content of a <text> element).
 */
export interface BetterTUITextInstance {
  id: string;
  type: "#text";
  text: string;
  parent: BetterTUIInstance | null;
  /** Native node ID in the Rust engine. */
  nativeId: number;
}

/**
 * The container passed to `reconciler.createContainer`.
 * Holds the renderer and the engine root node ID.
 */
export interface BetterTUIContainer {
  /** The CliRenderer this root is bound to. */
  renderer: CliRenderer;
  /** The native engine root node ID. React children are appended here. */
  rootNativeId: number;
}

export type AnyInstance = BetterTUIInstance | BetterTUITextInstance;
