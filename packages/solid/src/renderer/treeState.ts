/**
 * Shadow tree state for the Solid universal renderer.
 *
 * solid-js/universal requires `getParentNode`, `getFirstChild`, and
 * `getNextSibling` for its reactive diffing engine. Calling into the Rust
 * engine for every such query would be expensive and require JSON parsing, so
 * we maintain a lightweight JS-side shadow of the native node tree.
 *
 * The shadow is kept in sync by every operation that mutates the tree
 * (`insertNode`, `removeNode`). It never diverges from the native tree
 * because @bettertui/solid is the only writer.
 */

/** A sentinel value returned when there is no such sibling / child / parent. */
export const NO_NODE = -1 as const;

export interface TreeState {
  /** Insert `child` into `parent`'s children list immediately before `anchor`.
   *  When `anchor` is `NO_NODE` the child is appended. */
  insertChild(parent: number, child: number, anchor: number): void;
  /** Remove `node` from its parent's children list and delete all metadata. */
  removeNode(node: number): void;
  /** Return the parent of `node`, or `NO_NODE`. */
  getParent(node: number): number;
  /** Return the first child of `node`, or `NO_NODE`. */
  getFirstChild(node: number): number;
  /** Return the next sibling of `node`, or `NO_NODE`. */
  getNextSibling(node: number): number;
  /** Mark a native id as a text node (created by `createTextNode`). */
  markTextNode(node: number): void;
  /** Returns true when `node` was created as a text node. */
  isTextNode(node: number): boolean;
}

export function createTreeState(): TreeState {
  const parentOf = new Map<number, number>();
  const childrenOf = new Map<number, number[]>();
  const textNodes = new Set<number>();

  function children(node: number): number[] {
    let c = childrenOf.get(node);
    if (!c) {
      c = [];
      childrenOf.set(node, c);
    }
    return c;
  }

  return {
    insertChild(parent: number, child: number, anchor: number): void {
      // Remove child from its current parent if it already has one.
      const oldParent = parentOf.get(child);
      if (oldParent !== undefined) {
        const siblings = children(oldParent);
        const idx = siblings.indexOf(child);
        if (idx !== -1) siblings.splice(idx, 1);
      }

      parentOf.set(child, parent);
      const siblings = children(parent);

      if (anchor === NO_NODE) {
        siblings.push(child);
      } else {
        const anchorIdx = siblings.indexOf(anchor);
        if (anchorIdx === -1) {
          siblings.push(child);
        } else {
          siblings.splice(anchorIdx, 0, child);
        }
      }
    },

    removeNode(node: number): void {
      const parent = parentOf.get(node);
      if (parent !== undefined) {
        const siblings = children(parent);
        const idx = siblings.indexOf(node);
        if (idx !== -1) siblings.splice(idx, 1);
      }
      parentOf.delete(node);
      childrenOf.delete(node);
      textNodes.delete(node);
    },

    getParent(node: number): number {
      return parentOf.get(node) ?? NO_NODE;
    },

    getFirstChild(node: number): number {
      return children(node)[0] ?? NO_NODE;
    },

    getNextSibling(node: number): number {
      const parent = parentOf.get(node);
      if (parent === undefined) return NO_NODE;
      const siblings = children(parent);
      const idx = siblings.indexOf(node);
      if (idx === -1 || idx + 1 >= siblings.length) return NO_NODE;
      return siblings[idx + 1] ?? NO_NODE;
    },

    markTextNode(node: number): void {
      textNodes.add(node);
    },

    isTextNode(node: number): boolean {
      return textNodes.has(node);
    },
  };
}
