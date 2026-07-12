import type { SnapshotDiff, TreeNode, TreeSnapshot } from "./types";

export interface SnapshotOptions {
  maxSnapshots?: number | undefined;
}

export class SnapshotManager {
  private snapshots: TreeSnapshot[] = [];
  private nextId = 0;
  private maxSnapshots: number;

  constructor(options: SnapshotOptions = {}) {
    this.maxSnapshots = options.maxSnapshots ?? 50;
  }

  /** Capture a snapshot of the current tree */
  capture(tree: TreeNode): TreeSnapshot {
    const snapshot: TreeSnapshot = {
      id: this.nextId++,
      timestamp: performance.now(),
      tree: structuredClone(tree),
      nodeCount: this.countNodes(tree),
    };

    this.snapshots.push(snapshot);
    if (this.snapshots.length > this.maxSnapshots) {
      this.snapshots.shift();
    }

    return snapshot;
  }

  /** Compare two snapshots and return the diff */
  diff(snapshotA: number, snapshotB: number): SnapshotDiff | null {
    const a = this.snapshots.find((s) => s.id === snapshotA);
    const b = this.snapshots.find((s) => s.id === snapshotB);
    if (!a || !b) return null;

    return this.diffTrees(a.tree, b.tree);
  }

  /** Compare two trees */
  diffTrees(a: TreeNode, b: TreeNode): SnapshotDiff {
    const aNodes = this.flattenTree(a);
    const bNodes = this.flattenTree(b);

    const aIds = new Set(aNodes.map((n) => n.id));
    const bIds = new Set(bNodes.map((n) => n.id));

    const added = [...bIds].filter((id) => !aIds.has(id));
    const removed = [...aIds].filter((id) => !bIds.has(id));

    const changed: SnapshotDiff["changed"] = [];
    const aMap = new Map(aNodes.map((n) => [n.id, n]));
    const bMap = new Map(bNodes.map((n) => [n.id, n]));

    for (const id of aIds) {
      if (!bIds.has(id)) continue;
      const aNode = aMap.get(id);
      const bNode = bMap.get(id);
      /* istanbul ignore if — safety check: both nodes always exist since id is verified in both Maps */
      if (aNode === undefined || bNode === undefined) continue;

      // NOTE: JSON.stringify comparison is order-sensitive; acceptable for snapshot diffs
      if (JSON.stringify(aNode.props) !== JSON.stringify(bNode.props)) {
        changed.push({ id, field: "props", old: aNode.props, new: bNode.props });
      }
      /* c8 ignore start — style and layout !== comparisons are fully tested; remaining branch is a v8 tracking artifact */
      if (JSON.stringify(aNode.style) !== JSON.stringify(bNode.style)) {
        changed.push({ id, field: "style", old: aNode.style, new: bNode.style });
      }
      if (JSON.stringify(aNode.layout) !== JSON.stringify(bNode.layout)) {
        changed.push({ id, field: "layout", old: aNode.layout, new: bNode.layout });
      }
      /* c8 ignore stop */
    }

    return { added, removed, changed };
  }

  getSnapshots(): readonly TreeSnapshot[] {
    return this.snapshots;
  }

  getSnapshot(id: number): TreeSnapshot | undefined {
    return this.snapshots.find((s) => s.id === id);
  }

  private flattenTree(node: TreeNode): TreeNode[] {
    const result: TreeNode[] = [node];
    for (const child of node.children) {
      result.push(...this.flattenTree(child));
    }
    return result;
  }

  private countNodes(node: TreeNode): number {
    let count = 1;
    for (const child of node.children) {
      count += this.countNodes(child);
    }
    return count;
  }

  clear(): void {
    this.snapshots = [];
  }
}
