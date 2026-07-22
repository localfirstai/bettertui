import type {
  AlignItems,
  AlignSelf,
  ColorValue,
  FlexDirection,
  Gap,
  Inset,
  JustifyContent,
  LayoutConstraints,
  Margin,
  Overflow,
  Padding,
  Position,
  Sizing,
  Style,
} from "@bettertui/shared";

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
  id: string;
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

export type Command =
  | { type: "CreateNode"; id: string; kind: string }
  | { type: "RemoveNode"; id: string }
  | { type: "AppendChild"; parent: string; child: string }
  | { type: "InsertBefore"; reference: string; child: string }
  | { type: "MoveNode"; node: string; newParent: string }
  | { type: "ReplaceNode"; old: string; new: string }
  | { type: "DetachNode"; id: string }
  | { type: "SetText"; id: string; text: string }
  | { type: "SetStyle"; id: string; style: Style }
  | { type: "SetLayout"; id: string; layout: LayoutConstraints }
  | { type: "SetAttribute"; id: string; key: string; value: string }
  | { type: "RemoveAttribute"; id: string; key: string }
  | { type: "BeginFrame"; frameId: number }
  | { type: "CommitFrame"; frameId: number }
  | { type: "Invalidate"; id: string }
  | { type: "Shutdown" }
  | { type: "SetFlexDirection"; id: string; direction: FlexDirection }
  | { type: "SetJustifyContent"; id: string; value: JustifyContent }
  | { type: "SetAlignItems"; id: string; value: AlignItems }
  | { type: "SetAlignSelf"; id: string; value: AlignSelf }
  | { type: "SetFlexGrow"; id: string; value: number }
  | { type: "SetFlexShrink"; id: string; value: number }
  | { type: "SetFlexBasis"; id: string; value: Sizing }
  | { type: "SetPosition"; id: string; value: Position }
  | { type: "SetWidth"; id: string; value: Sizing }
  | { type: "SetHeight"; id: string; value: Sizing }
  | { type: "SetMinWidth"; id: string; value: Sizing }
  | { type: "SetMaxWidth"; id: string; value: Sizing }
  | { type: "SetMinHeight"; id: string; value: Sizing }
  | { type: "SetMaxHeight"; id: string; value: Sizing }
  | { type: "SetOverflow"; id: string; value: Overflow }
  | { type: "SetOpacity"; id: string; value: number }
  | { type: "SetZIndex"; id: string; value: number }
  | { type: "SetPadding"; id: string; value: Padding }
  | { type: "SetMargin"; id: string; value: Margin }
  | { type: "SetGap"; id: string; value: Gap }
  | { type: "SetInset"; id: string; value: Inset }
  | { type: "SetForeground"; id: string; color: ColorValue }
  | { type: "SetBackground"; id: string; color: ColorValue }
  | { type: "SetBold"; id: string; value: boolean }
  | { type: "SetItalic"; id: string; value: boolean }
  | { type: "SetUnderline"; id: string; value: boolean }
  | { type: "SetDim"; id: string; value: boolean }
  | { type: "SetStrikethrough"; id: string; value: boolean }
  | { type: "SetInverse"; id: string; value: boolean }
  | { type: "SetHidden"; id: string; value: boolean }
  | { type: "SetBlink"; id: string; value: boolean }
  | {
      type: "SetScreenMode";
      mode: "alternate-screen" | "main-screen" | "split-footer";
      footerHeight?: number;
    };

export interface CommandBufferConsumer {
  push(command: Command): void;
}
