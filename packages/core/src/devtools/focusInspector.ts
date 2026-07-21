import type { FocusSnapshot } from "./devtools.types";

export interface FocusInspectorOptions {
  onFocusChange?: ((snapshot: FocusSnapshot) => void) | undefined;
}

export class FocusInspector {
  private focusedNodeId: string | null = null;
  private previousNodeId: string | null = null;
  private focusableNodes: string[] = [];
  private tabOrder: string[] = [];
  private currentScope: string | null = null;
  private focusHistory: Array<{
    timestamp: number;
    nodeId: string | null;
    type: "focus" | "blur";
  }> = [];
  private onFocusChange: ((snapshot: FocusSnapshot) => void) | undefined;

  constructor(options: FocusInspectorOptions = {}) {
    this.onFocusChange = options.onFocusChange;
  }

  recordFocus(nodeId: string): void {
    this.previousNodeId = this.focusedNodeId;
    this.focusedNodeId = nodeId;
    this.focusHistory.push({ timestamp: performance.now(), nodeId, type: "focus" });
    this.onFocusChange?.(this.getSnapshot());
  }

  recordBlur(nodeId: string): void {
    if (this.focusedNodeId === nodeId) {
      this.previousNodeId = nodeId;
      this.focusedNodeId = null;
    }
    this.focusHistory.push({ timestamp: performance.now(), nodeId, type: "blur" });
    this.onFocusChange?.(this.getSnapshot());
  }

  setFocusableNodes(nodes: string[]): void {
    this.focusableNodes = nodes;
  }

  setTabOrder(order: string[]): void {
    this.tabOrder = order;
  }

  setScope(scope: string | null): void {
    this.currentScope = scope;
  }

  getSnapshot(): FocusSnapshot {
    return {
      focusedNodeId: this.focusedNodeId,
      previousNodeId: this.previousNodeId,
      focusableNodes: [...this.focusableNodes],
      tabOrder: [...this.tabOrder],
      currentScope: this.currentScope,
    };
  }

  getFocusHistory(): Array<{ timestamp: number; nodeId: string | null; type: "focus" | "blur" }> {
    return this.focusHistory;
  }

  getRecentFocusChanges(
    count: number,
  ): Array<{ timestamp: number; nodeId: string | null; type: "focus" | "blur" }> {
    return this.focusHistory.slice(-count);
  }

  isFocused(nodeId: string): boolean {
    return this.focusedNodeId === nodeId;
  }

  clear(): void {
    this.focusedNodeId = null;
    this.previousNodeId = null;
    this.focusableNodes = [];
    this.tabOrder = [];
    this.currentScope = null;
    this.focusHistory = [];
  }
}
