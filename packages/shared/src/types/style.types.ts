/** A CSS-like color value: named color, hex, rgb(), or rgba(). */
export type ColorValue = string;

/** Visual style properties for a UI node. */
export interface Style {
  /** Foreground (text) color */
  fg?: ColorValue;
  /** Background color */
  bg?: ColorValue;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  dim?: boolean;
  strikethrough?: boolean;
  /** Swap foreground and background colors */
  inverse?: boolean;
  /** Text alignment */
  textAlign?: "left" | "center" | "right" | "justify";
}

/** All possible border visual styles. */
export type BorderStyleKind = "none" | "solid" | "dashed" | "dotted" | "double";

/** Border visual configuration. */
export interface BorderStyle {
  style: BorderStyleKind;
  /** Border foreground color */
  fg?: ColorValue;
}
