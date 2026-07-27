/**
 * VNode composition system for BetterTUI.
 * Provides a declarative API for building UI trees.
 */

import type { CliRenderer } from "../platform/cliRenderer";
import { type BoxOptions, BoxRenderable } from "../renderables/Box";
import { InputRenderable, type InputRenderableOptions } from "../renderables/Input";
import { SelectRenderable, type SelectRenderableOptions } from "../renderables/Select";
import { type CodeOptions, CodeRenderable } from "../renderables/Stubs";
import {
  type FrameBufferLike,
  type FrameBufferOptions,
  FrameBufferRenderable,
} from "../renderables/Stubs";
import { type ASCIIFontOptions, ASCIIFontRenderable } from "../renderables/Stubs";
import { TabSelectRenderable, type TabSelectRenderableOptions } from "../renderables/TabSelect";
import { type TextOptions, TextRenderable } from "../renderables/Text";

/** A VNode (virtual node) — a lazy description of a renderable. */
export interface VNode {
  _type: string | (new (renderer: CliRenderer, options: Record<string, unknown>) => BoxRenderable);
  _props: Record<string, unknown>;
  _children: VNode[];
}

/**
 * Create a VNode.
 */
export function h(
  type: string | (new (renderer: CliRenderer, options: Record<string, unknown>) => BoxRenderable),
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
export function instantiate(ctx: CliRenderer, vnode: VNode): BoxRenderable {
  const { _type: type, _props: props, _children: children } = vnode;

  let instance: BoxRenderable;

  if (typeof type === "function") {
    // Custom component class
    instance = new (
      type as new (
        renderer: CliRenderer,
        options: Record<string, unknown>,
      ) => BoxRenderable
    )(ctx, props);
  } else {
    // Built-in type
    switch (type) {
      case "Text":
        instance = new TextRenderable(ctx, props as TextOptions);
        break;
      case "Input":
        instance = new InputRenderable(ctx, props as InputRenderableOptions);
        break;
      case "Select":
        instance = new SelectRenderable(ctx, props as SelectRenderableOptions);
        break;
      case "TabSelect":
        instance = new TabSelectRenderable(ctx, props as TabSelectRenderableOptions);
        break;
      case "Code":
        instance = new CodeRenderable(ctx, props as CodeOptions);
        break;
      case "FrameBuffer":
      case "Generic": {
        // Generic: uses a render function prop
        const renderFn = props.render as FrameBufferOptions["drawFn"];
        instance = new FrameBufferRenderable(ctx, {
          ...props,
          drawFn: renderFn,
        } as FrameBufferOptions);
        break;
      }
      case "ASCIIFont":
        instance = new ASCIIFontRenderable(ctx, props as ASCIIFontOptions);
        break;
      default:
        instance = new BoxRenderable(ctx, props as BoxOptions);
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
export function maybeMakeRenderable(ctx: CliRenderer, input: VNode | BoxRenderable): BoxRenderable {
  if (input instanceof BoxRenderable) return input;
  return instantiate(ctx, input as VNode);
}

// ── Functional VNode constructors ─────────────────────────────────────────────

export function Box(props?: BoxOptions, ...children: VNode[]): VNode {
  return { _type: "Box", _props: (props ?? {}) as Record<string, unknown>, _children: children };
}

export function Text(props?: TextOptions, ...children: (VNode | string)[]): VNode {
  const processedChildren = children.map((c) =>
    typeof c === "string" ? h("Text", { content: c }) : c,
  );
  return {
    _type: "Text",
    _props: (props ?? {}) as Record<string, unknown>,
    _children: processedChildren,
  };
}

export function Input(props?: InputRenderableOptions, ...children: VNode[]): VNode {
  return { _type: "Input", _props: (props ?? {}) as Record<string, unknown>, _children: children };
}

export function Select(props?: SelectRenderableOptions, ...children: VNode[]): VNode {
  return { _type: "Select", _props: (props ?? {}) as Record<string, unknown>, _children: children };
}

export function TabSelect(props?: TabSelectRenderableOptions, ...children: VNode[]): VNode {
  return {
    _type: "TabSelect",
    _props: (props ?? {}) as Record<string, unknown>,
    _children: children,
  };
}

export function Code(props?: CodeOptions, ...children: VNode[]): VNode {
  return { _type: "Code", _props: (props ?? {}) as Record<string, unknown>, _children: children };
}

export function Generic(
  props?: BoxOptions & { render?: (buffer: FrameBufferLike, dt: number, r: BoxRenderable) => void },
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
