import type { TreeNode } from "./devtools.types";

export interface TreeInspectorOptions {
  onTreeUpdate?: ((root: TreeNode | null) => void) | undefined;
}

export class TreeInspector {
  private root: TreeNode | null = null;
  private nodeIndex = new Map<string, TreeNode>();
  private dirtyNodes = new Set<string>();
  private onTreeUpdate: ((root: TreeNode | null) => void) | undefined;

  constructor(options: TreeInspectorOptions = {}) {
    this.onTreeUpdate = options.onTreeUpdate;
  }

  /** Build a tree from a flat list of node descriptors */
  buildTree(
    nodes: Array<{
      id: string;
      type: string;
      parent?: string;
      props?: Record<string, unknown>;
      style?: Record<string, unknown>;
      layout?: { x: number; y: number; width: number; height: number };
      dirty?: boolean;
      visible?: boolean;
      zIndex?: number;
    }>,
  ): TreeNode {
    this.nodeIndex.clear();
    this.dirtyNodes.clear();

    // Create all nodes
    const nodeMap = new Map<string, TreeNode>();
    for (const n of nodes) {
      const treeNode: TreeNode = {
        id: n.id,
        type: n.type,
        props: n.props ?? {},
        style: n.style,
        layout: n.layout,
        children: [],
        parent: n.parent,
        dirty: n.dirty,
        visible: n.visible,
        zIndex: n.zIndex,
      };
      nodeMap.set(n.id, treeNode);
      this.nodeIndex.set(n.id, treeNode);
      if (n.dirty) this.dirtyNodes.add(n.id);
    }

    // Wire parent-child relationships
    let root: TreeNode | null = null;
    for (const treeNode of nodeMap.values()) {
      if (treeNode.parent) {
        const parentNode = nodeMap.get(treeNode.parent);
        if (parentNode) {
          parentNode.children.push(treeNode);
        }
      } else {
        root = treeNode;
      }
    }

    this.root = root;
    this.onTreeUpdate?.(root);
    return root ?? { id: "empty", type: "Empty", props: {}, children: [] };
  }

  /** Update a single node's properties */
  updateNode(id: string, updates: Partial<Omit<TreeNode, "id" | "children">>): void {
    const node = this.nodeIndex.get(id);
    if (!node) return;

    if (updates.props !== undefined) node.props = updates.props;
    if (updates.style !== undefined) node.style = updates.style;
    if (updates.layout !== undefined) node.layout = updates.layout;
    if (updates.dirty !== undefined) {
      node.dirty = updates.dirty;
      if (updates.dirty) {
        this.dirtyNodes.add(id);
      } else {
        this.dirtyNodes.delete(id);
      }
    }
    if (updates.visible !== undefined) node.visible = updates.visible;
    if (updates.zIndex !== undefined) node.zIndex = updates.zIndex;
  }

  /** Mark a node as dirty */
  markDirty(id: string): void {
    this.dirtyNodes.add(id);
    const node = this.nodeIndex.get(id);
    if (node) node.dirty = true;
  }

  /** Clear dirty state for all nodes */
  clearDirty(): void {
    for (const id of this.dirtyNodes) {
      const node = this.nodeIndex.get(id);
      if (node) node.dirty = false;
    }
    this.dirtyNodes.clear();
  }

  getNode(id: string): TreeNode | undefined {
    return this.nodeIndex.get(id);
  }

  getRoot(): TreeNode | null {
    return this.root;
  }

  getDirtyNodes(): TreeNode[] {
    return [...this.dirtyNodes]
      .map((id) => this.nodeIndex.get(id))
      .filter((n): n is TreeNode => n !== undefined);
  }

  /** Find nodes matching a predicate */
  findNodes(predicate: (node: TreeNode) => boolean): TreeNode[] {
    const results: TreeNode[] = [];
    const walk = (node: TreeNode) => {
      if (predicate(node)) results.push(node);
      for (const child of node.children) {
        walk(child);
      }
    };
    if (this.root) walk(this.root);
    return results;
  }

  /** Get the path from root to a given node */
  getPath(nodeId: string): TreeNode[] {
    const path: TreeNode[] = [];
    let current = this.nodeIndex.get(nodeId);
    while (current) {
      path.unshift(current);
      current = current.parent ? this.nodeIndex.get(current.parent) : undefined;
    }
    return path;
  }

  /** Count total nodes in the tree */
  countNodes(): number {
    return this.nodeIndex.size;
  }

  /** Get all nodes as a flat array */
  getAllNodes(): TreeNode[] {
    return [...this.nodeIndex.values()];
  }

  clear(): void {
    this.root = null;
    this.nodeIndex.clear();
    this.dirtyNodes.clear();
  }
}
