import type { LayoutConstraints, Style } from "@bettertui/shared";

export type HostContext = Record<string, unknown>;

export interface Instance {
  id: string;
  type: string;
  props: Record<string, unknown>;
  style: Style;
  layout: LayoutConstraints;
  children: Instance[];
  parent: Instance | null;
}

export interface TextInstance {
  type: "#text";
  text: string;
  parent: Instance | null;
}

export type HostConfig = {
  type: string;
  props: Record<string, unknown>;
  container: Instance;
  instance: Instance;
  textInstance: TextInstance;
  suspenseInstance: Instance;
  hydratableInstance: Instance;
  publicInstance: Instance;
  hostContext: HostContext;
  updatePayload: Record<string, unknown>;
  childSet: Instance[];
  timeoutHandle: number;
  cornerstoneTimeoutHandle: number;
};

export const createReconciler = (): unknown => {
  return null;
};
