import { type CliRenderer, createCliRenderer } from "@bettertui/core";
import {
  Box,
  type BoxOptions,
  Generic,
  type VNode,
  VNodeBox,
  VNodeInput,
  VNodeText,
  delegate,
  instantiate,
  vstyles,
} from "@bettertui/core";
import type { FrameBufferLike } from "@bettertui/core";
import { RGBA, parseColor } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

const textColor = parseColor("#FFFFFF");
const globalbgColor = parseColor("#333333");
const transparent = parseColor("transparent");

const { bold, italic, underline, dim, boldItalic, boldUnderline, color, bgColor, styled } = vstyles;

// This is NOT react and not reactive, it's just a declarative way to compose renderables
// and mount them into a parent container.
function MyRenderable(_props: Record<string, unknown>, children: VNode[] = []) {
  const mouseHandler = (event: unknown) => {
    const e = event as { type?: string };
    console.log("mouseHandler", e.type);
  };

  return VNodeBox(
    { id: "inner" },
    VNodeBox(
      {
        border: true,
        borderStyle: "double",
        padding: 1,
        onMouseDown: mouseHandler,
        flexDirection: "row",
      },
      ...children,
    ),
  );
}

function Button(
  props: {
    title: string;
    onClick: () => void;
    borderColor?: string | RGBA;
  },
  children: VNode[] = [],
) {
  return VNodeBox(
    {
      id: "button",
      border: true,
      onMouseDown: props.onClick,
      borderColor: props.borderColor,
    },
    VNodeText({ content: props.title }),
    ...children,
  );
}

// Custom Rendering Functional Construct
function VNodeButton(
  props: {
    title: string;
    onClick: () => void;
    borderColor?: RGBA;
  },
  children: VNode[] = [],
) {
  return Generic(
    {
      render: (buffer: FrameBufferLike, deltaTime: number, renderable: Box) =>
        demoRenderFn(props, buffer, deltaTime, renderable),
      maxWidth: props.title.length + 4,
      margin: 1,
    },
    VNodeBox(
      {
        id: "button",
        height: 3,
        onMouseDown: props.onClick,
      },
      ...children,
    ),
  );
}

// Custom Rendering - Class Method Example
class MyRoot {
  width: number;

  constructor(private readonly props: { title: string; borderColor?: RGBA }) {
    this.width = Math.max(props.title.length + 4, 12);
  }

  render(buffer: FrameBufferLike, deltaTime: number, renderable: Box) {
    demoRenderFn(this.props, buffer, deltaTime, renderable);
  }
}

function ButtonWithClassRender(
  props: { title: string; onClick: () => void; borderColor?: RGBA; marginLeft?: number },
  children: VNode[] = [],
) {
  const myRoot = new MyRoot(props);
  return Generic(
    {
      render: (buffer: FrameBufferLike, dt: number, r: Box) => myRoot.render(buffer, dt, r),
      maxWidth: props.title.length + 4,
      margin: props.marginLeft ?? 1,
    },
    VNodeBox(
      {
        id: "button",
        height: 3,
        onMouseDown: props.onClick,
      },
      ...children,
    ),
  );
}

// Host Override Example
function MyDelegateToVNodeRenderable(props: Record<string, unknown>, children: VNode[] = []) {
  return delegate(
    `${props.id}_box3`,
    VNodeBox(
      { id: `${props.id}_outer3`, border: true, borderColor: "blue" },
      VNodeBox(
        { id: `${props.id}_inner3`, border: true, borderColor: "magenta" },
        VNodeBox(
          { id: `${props.id}_box3`, flexDirection: "row", border: true, padding: 1 },
          ...children,
        ),
      ),
    ),
  );
}

function MyDelegateToRenderableComponent(
  renderer: CliRenderer,
  _props: Record<string, unknown>,
  children: VNode[] = [],
): Box {
  // Instantiate directly (no delegate needed for Box)
  return instantiate(
    renderer,
    VNodeBox(
      { id: "__outer4", border: true, borderColor: "blue" },
      VNodeBox(
        { id: "__inner4", border: true, borderColor: "magenta" },
        VNodeBox({ id: "__box4", flexDirection: "row", border: true, padding: 1 }, ...children),
      ),
    ),
  );
}

function MyInstancedRenderable(
  renderer: CliRenderer,
  props: Record<string, unknown>,
  children: VNode[] = [],
) {
  return instantiate(renderer, MyDelegateToVNodeRenderable(props, children));
}

function LabeledInput(props: { id: string; label: string; placeholder: string }) {
  return delegate(
    `${props.id}-input`,
    VNodeBox(
      { flexDirection: "row", id: `${props.id}-labeled-outer` },
      VNodeText({ content: `${props.label} ` }),
      VNodeInput({
        id: `${props.id}-input`,
        placeholder: props.placeholder,
        width: 20,
        backgroundColor: "white",
        textColor: "black",
        cursorColor: "blue",
        focusedBackgroundColor: "orange",
      }),
    ),
  );
}

function BaseBox(props: BoxOptions, children: VNode[] = []) {
  return VNodeBox(
    {
      id: "base-box",
      border: true,
      borderColor: "blue",
      backgroundColor: "orange",
      ...props,
    },
    ...children,
  );
}

function ExtendedBaseBox(props: BoxOptions, children: VNode[] = []) {
  return BaseBox(
    {
      id: "extended-base-box",
      ...props,
    },
    children,
  );
}

export function run(renderer: CliRenderer) {
  renderer.start();
  const mainGroup = new Box(renderer, {
    id: "main-group",
  });
  renderer.root.add(mainGroup);

  // BaseBox example
  mainGroup.add(
    instantiate(
      renderer,
      ExtendedBaseBox({
        width: 20,
        height: 10,
        position: "absolute",
        left: 55,
        top: 10,
        zIndex: 1000,
      }),
    ),
  );

  // Proxied VNode example
  const tree = instantiate(
    renderer,
    MyRenderable({ id: "demo-root" }, [
      VNodeBox(
        { id: "child-1", width: 20, height: 3, border: true, marginBottom: 1 },
        VNodeText({ content: "Hello" }),
      ),
      VNodeBox(
        { id: "child-2", width: 24, height: 3, border: true },
        VNodeText({ content: "VNode world" }),
      ),
    ]),
  );
  tree.backgroundColor = RGBA.fromInts(0, 155, 155, 100);

  mainGroup.add(tree);

  const inputInstance = instantiate(
    renderer,
    LabeledInput({
      id: "labeled-input",
      label: "Label:",
      placeholder: "Enter your text...",
    }),
  );
  inputInstance.focus();
  mainGroup.add(inputInstance);

  //
  // VNode delegated version
  const instance1 = instantiate(
    renderer,
    MyDelegateToVNodeRenderable({ id: "delegated-demo-root" }, [
      VNodeBox(
        { id: "child-1", width: 20, height: 3, border: true, marginBottom: 1 },
        VNodeText({ content: "Hello delegated 1" }),
      ),
      VNodeBox(
        { id: "child-2", width: 24, height: 3, border: true },
        VNodeText({ content: "VNode world delegated 1" }),
      ),
    ]),
  );
  instance1.backgroundColor = RGBA.fromInts(155, 0, 155, 100);

  mainGroup.add(instance1);

  //
  // Instaced Delegated version
  const instance = MyInstancedRenderable(renderer, { id: "demo-root" }, [
    VNodeBox(
      { id: "child-1", width: 20, height: 3, border: true, marginBottom: 1 },
      VNodeText({ content: "Hello 2" }),
    ),
    VNodeBox(
      { id: "child-2", width: 24, height: 3, border: true },
      VNodeText({ content: "VNode world 2" }),
    ),
  ]);

  mainGroup.add(instance);

  // Delegated to __box3, would otherwise end up in the top-level group!
  instance.add(
    instantiate(
      renderer,
      VNodeBox(
        { id: "child-3", width: 24, height: 3, border: true },
        VNodeText({ content: "VNode world 3" }),
      ),
    ),
  );
  instance.add(
    instantiate(
      renderer,
      Button({ title: "Click me", onClick: () => console.log("clicked"), borderColor: "red" }),
    ),
  );

  //
  // Renderable delegated version
  const renderableInstance = MyDelegateToRenderableComponent(renderer, { id: "demo-root" }, [
    VNodeBox(
      { id: "child-1", width: 20, height: 3, border: true, marginBottom: 1 },
      VNodeText({ content: "Hello 4" }),
    ),
    VNodeBox(
      { id: "child-2", width: 24, height: 3, border: true },
      VNodeText({ content: "VNode world 4" }),
    ),
  ]);
  mainGroup.add(renderableInstance);

  // Would otherwise end up in the top-level group!
  renderableInstance.add(
    instantiate(
      renderer,
      Button({ title: "Click me too!", onClick: () => console.log("clicked"), borderColor: "red" }),
    ),
  );

  //
  // Add animated VNode button
  mainGroup.add(
    instantiate(
      renderer,
      VNodeButton({
        title: "Animated VNode",
        onClick: () => console.log("vnode 1 clicked"),
        borderColor: RGBA.fromInts(0, 0, 255, 255),
      }),
    ),
  );
  mainGroup.add(
    instantiate(
      renderer,
      VNodeButton({
        title: "Same VNode, different props",
        onClick: () => console.log("vnode 2 clicked"),
        borderColor: RGBA.fromInts(255, 0, 255, 255),
      }),
    ),
  );

  //
  // Add button with class render function
  mainGroup.add(
    instantiate(
      renderer,
      ButtonWithClassRender({
        marginLeft: 1,
        title: "ClassRender",
        onClick: () => console.log("clicked"),
        borderColor: RGBA.fromInts(0, 0, 255, 255),
      }),
    ),
  );

  mainGroup.add(
    instantiate(
      renderer,
      VNodeBox(
        { flexDirection: "column", marginTop: 2 },
        // Basic styles
        bold("Bold Text"),
        italic("Italic Text"),
        underline("Underlined Text"),
        dim("Dim Text"),

        // Combined styles
        boldItalic("Bold and Italic"),
        boldUnderline("Bold and Underlined"),
        // italicUnderline is not in vstyles; use italic("Italic and Underlined")
        italic("Italic and Underlined"),

        // Colors
        color("#ff6b6b", "Red Text"),
        bgColor("#4ecdc4", "Text with Background"),

        // Custom styling
        styled({ content: "Custom Styled" }, "Custom Styled"),

        // Stacked styles
        color("#ffffff", bold("hello"), " world"),
        color("#ff6b6b", bold("Bold Red"), " normal"),
        color("#4ecdc4", "Green Italic", " normal again"),
      ),
    ),
  );
}

export function destroy(renderer: CliRenderer) {
  renderer.root.getRenderable("main-group")?.destroyRecursively();
  renderer.requestRender();
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
  renderer.start();
}

function demoRenderFn(
  props: { title: string; borderColor?: RGBA },
  buffer: FrameBufferLike,
  _deltaTime: number,
  renderable: Box,
) {
  const x = renderable.x;
  const y = renderable.y;
  const w = typeof renderable.width === "number" ? renderable.width : 0;
  const h = typeof renderable.height === "number" ? renderable.height : 0;

  const borderColor = props.borderColor ?? RGBA.fromInts(255, 255, 0, 255);

  // Draw a simple animated button with pulsing border
  const timeInSeconds = Date.now() / 1000;
  const pulse = Math.sin(timeInSeconds * 4) * 0.5 + 0.5; // Fast pulsing, 0-1 oscillation

  const pulsingBorderColor = RGBA.fromValues(
    borderColor.r * (0.1 + pulse * 0.9),
    borderColor.g * (0.1 + pulse * 0.9),
    borderColor.b * (0.1 + pulse * 0.9),
    borderColor.a,
  );

  const bgPulse = Math.sin(timeInSeconds * 2 + Math.PI / 2) * 0.4 + 0.6; // Different frequency and phase
  const pulsingBgColor = RGBA.fromValues(
    globalbgColor.r * bgPulse,
    globalbgColor.g * bgPulse,
    globalbgColor.b * bgPulse,
    globalbgColor.a,
  );

  for (let row = 0; row < h; row++) {
    for (let col = 0; col < w; col++) {
      const isTop = row === 0;
      const isBottom = row === h - 1;
      const isLeft = col === 0;
      const isRight = col === w - 1;
      const isBorder = isTop || isBottom || isLeft || isRight;

      if (isBorder) {
        buffer.setCell(x + col, y + row, "█", pulsingBorderColor, pulsingBgColor);
      } else {
        buffer.setCell(x + col, y + row, " ", textColor, pulsingBgColor);
      }
    }
  }

  const titlePulse = Math.sin(timeInSeconds * 6) * 0.5 + 0.5; // Even faster text pulse
  const textScale = 0.3 + titlePulse * 0.7;
  const pulsingTextColor = RGBA.fromValues(
    textColor.r * textScale,
    textColor.g * textScale,
    textColor.b * textScale,
    textColor.a,
  );

  const titleX = x + Math.floor((w - props.title.length) / 2);
  const titleY = y + Math.floor(h / 2);
  if (titleY >= y && titleY < y + h) {
    buffer.drawText(props.title, titleX, titleY, pulsingTextColor, transparent);
  }
}
