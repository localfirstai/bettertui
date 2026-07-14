import type { LayoutConstraints, Style } from "@bettertui/shared";
import { generateId } from "@bettertui/shared";
import type { Instance, TextInstance } from "./types";

export function createInstance(type: string, props: Record<string, unknown>): Instance {
  const id = generateId();
  const { children, style, layout, ...restProps } = props;

  return {
    id,
    type,
    props: restProps,
    style: (style as Style) || {},
    layout: (layout as LayoutConstraints) || {},
    children: [],
    parent: null,
  };
}

export function createTextInstance(text: string): TextInstance {
  return {
    id: generateId(),
    type: "#text",
    text,
    parent: null,
  };
}

export function appendChild(parent: Instance, child: Instance | TextInstance): void {
  child.parent = parent;
  if ("children" in parent) {
    parent.children.push(child as Instance);
  }
}

export function removeChild(parent: Instance, child: Instance | TextInstance): void {
  child.parent = null;
  if ("children" in parent) {
    const index = parent.children.indexOf(child as Instance);
    if (index !== -1) {
      parent.children.splice(index, 1);
    }
  }
}

export function insertBefore(
  parent: Instance,
  child: Instance | TextInstance,
  reference: Instance | TextInstance,
): void {
  child.parent = parent;
  if ("children" in parent) {
    const index = parent.children.indexOf(reference as Instance);
    if (index !== -1) {
      parent.children.splice(index, 0, child as Instance);
    } else {
      parent.children.push(child as Instance);
    }
  }
}

export function prepareUpdate(
  _instance: Instance,
  _type: string,
  _oldProps: Record<string, unknown>,
  newProps: Record<string, unknown>,
): Record<string, unknown> | null {
  const { children, style, layout, ...restProps } = newProps;
  return restProps;
}

export function commitUpdate(instance: Instance, updatePayload: Record<string, unknown>): void {
  Object.assign(instance.props, updatePayload);
}

export function commitTextUpdate(textInstance: TextInstance, text: string): void {
  textInstance.text = text;
}

export function finalizeInitialChildren(_instance: Instance): boolean {
  return false;
}

export function resetAfterCommit(): void {
  // Flush happens at a higher level
}

export { generateId };
