/**
 * VNode composition system for BetterTUI.
 * Provides a declarative API for building UI trees.
 */

import type { CliRenderer } from "../platform/cliRenderer";
import { Box as BoxClass, type BoxOptions } from "../renderables/Box";
import { Input as InputClass, type InputOptions } from "../renderables/Input";
import { Select as SelectClass, type SelectOptions } from "../renderables/Select";
import {
  ASCIIFont as ASCIIFontClass,
  type ASCIIFontOptions,
  Code as CodeClass,
  type CodeOptions,
  FrameBuffer as FrameBufferClass,
  type FrameBufferLike,
  type FrameBufferOptions,
} from "../renderables/Stubs";
import { TabSelect as TabSelectClass, type TabSelectOptions } from "../renderables/TabSelect";
import { Text as TextClass, type TextOptions } from "../renderables/Text";

/** A VNode (virtual node) — a lazy description of a renderable. */
export interface VNode {
  _type: string | (new (renderer: CliRenderer, options: Record<string, unknown>) => BoxClass);
  _props: Record<string, unknown>;
  _children: VNode[];
}

/**
 * Create a VNode.
 */
export function h(
  type: string | (new (renderer: CliRenderer, options: Record<string, unknown>) => BoxClass),
  props?: Record<string, unknown> | null,
  ...children: (VNode | string | null | undefined)[]
): VNode {
  return {
    _type: type,
    _props: props ?? {},
    _children: children
      .filter(Boolean)
      .map((c) => (typeof c === "string" ? h("Text", { content: c }) : (c as VNode))),
  };
}

/**
 * Instantiate a VNode tree into real renderables.
 */
export function instantiate(ctx: CliRenderer, vnode: VNode): BoxClass {
  const { _type: type, _props: props, _children: children } = vnode;

  let instance: BoxClass;

  if (typeof type === "function") {
    // Custom component class
    instance = new (
      type as new (
        renderer: CliRenderer,
        options: Record<string, unknown>,
      ) => BoxClass
    )(ctx, props);
  } else {
    // Built-in type
    switch (type) {
      case "Text":
        instance = new TextClass(ctx, props as TextOptions);
        break;
      case "Input":
        instance = new InputClass(ctx, props as InputOptions);
        break;
      case "Select":
        instance = new SelectClass(ctx, props as SelectOptions);
        break;
      case "TabSelect":
        instance = new TabSelectClass(ctx, props as TabSelectOptions);
        break;
      case "Code":
        instance = new CodeClass(ctx, props as CodeOptions);
        break;
      case "FrameBuffer":
      case "Generic": {
        // Generic: uses a render function prop
        const renderFn = props.render as FrameBufferOptions["drawFn"];
        instance = new FrameBufferClass(ctx, {
          ...props,
          drawFn: renderFn,
        } as FrameBufferOptions);
        break;
      }
      case "ASCIIFont":
        instance = new ASCIIFontClass(ctx, props as ASCIIFontOptions);
        break;
      default:
        instance = new BoxClass(ctx, props as BoxOptions);
    }
  }

  // Instantiate and attach children
  for (const child of children) {
    const childInstance = instantiate(ctx, child);
    instance.add(childInstance);
  }

  return instance;
}

/**
 * Redirect add/remove/focus calls to a named child renderable.
 */
export function delegate(targets: string | string[], vnode: VNode): VNode {
  return {
    ...vnode,
    _props: {
      ...vnode._props,
      __delegateTargets: Array.isArray(targets) ? targets : [targets],
    },
  };
}

/** Maybe create a renderable from a VNode or return existing renderable. */
export function maybeMakeRenderable(ctx: CliRenderer, input: VNode | BoxClass): BoxClass {
  if (input instanceof BoxClass) return input;
  return instantiate(ctx, input as VNode);
}

// ── Functional VNode constructors ─────────────────────────────────────────────

export function BoxVNode(props?: BoxOptions, ...children: VNode[]): VNode {
  return { _type: "Box", _props: (props ?? {}) as Record<string, unknown>, _children: children };
}

export function TextVNode(props?: TextOptions, ...children: (VNode | string)[]): VNode {
  const processedChildren = children.map((c) =>
    typeof c === "string" ? h("Text", { content: c }) : c,
  );
  return {
    _type: "Text",
    _props: (props ?? {}) as Record<string, unknown>,
    _children: processedChildren,
  };
}

export function InputVNode(props?: InputOptions, ...children: VNode[]): VNode {
  return { _type: "Input", _props: (props ?? {}) as Record<string, unknown>, _children: children };
}

export function SelectVNode(props?: SelectOptions, ...children: VNode[]): VNode {
  return { _type: "Select", _props: (props ?? {}) as Record<string, unknown>, _children: children };
}

export function TabSelectVNode(props?: TabSelectOptions, ...children: VNode[]): VNode {
  return {
    _type: "TabSelect",
    _props: (props ?? {}) as Record<string, unknown>,
    _children: children,
  };
}

export function CodeVNode(props?: CodeOptions, ...children: VNode[]): VNode {
  return { _type: "Code", _props: (props ?? {}) as Record<string, unknown>, _children: children };
}

export {
  BoxVNode as Box,
  TextVNode as Text,
  InputVNode as Input,
  SelectVNode as Select,
  TabSelectVNode as TabSelect,
  CodeVNode as Code,
  GenericVNode as Generic,
};

export function GenericVNode(
  props?: BoxOptions & { render?: (buffer: FrameBufferLike, dt: number, r: BoxClass) => void },
  ...children: VNode[]
): VNode {
  return {
    _type: "Generic",
    _props: (props ?? {}) as Record<string, unknown>,
    _children: children,
  };
}

export function ScrollBox(props?: BoxOptions, ...children: VNode[]): VNode {
  return {
    _type: "ScrollBox",
    _props: (props ?? {}) as Record<string, unknown>,
    _children: children,
  };
}

export function ASCIIFont(props?: ASCIIFontOptions, ...children: VNode[]): VNode {
  return {
    _type: "ASCIIFont",
    _props: (props ?? {}) as Record<string, unknown>,
    _children: children,
  };
}

// ── vstyles: vnode-compatible text styling ────────────────────────────────────

function _vstyleText(text: string, fg?: string, _attrs?: number, bg?: string): VNode {
  return h("Text", { content: text, fg, bg });
}

export const vstyles = {
  bold: (text: string) => _vstyleText(text, undefined, 1),
  italic: (text: string) => _vstyleText(text, undefined, 4),
  underline: (text: string) => _vstyleText(text, undefined, 8),
  dim: (text: string) => _vstyleText(text, undefined, 2),
  color: (color: string, ...children: (string | VNode)[]) => {
    const textChildren = children.map((c) =>
      typeof c === "string" ? h("Text", { content: c }) : c,
    );
    return h("Text", { fg: color }, ...textChildren);
  },
  bgColor: (color: string, ...children: (string | VNode)[]) => {
    const textChildren = children.map((c) =>
      typeof c === "string" ? h("Text", { content: c }) : c,
    );
    return h("Text", { bg: color }, ...textChildren);
  },
  fg: (color: string) => (text: string) => _vstyleText(text, color),
  bg: (color: string) => (text: string) => _vstyleText(text, undefined, 0, color),
  styled: (attrs: Record<string, unknown>, text: string) => h("Text", { content: text, ...attrs }),
  boldItalic: (text: string) => _vstyleText(text, undefined, 5),
  boldUnderline: (text: string) => _vstyleText(text, undefined, 9),
};
