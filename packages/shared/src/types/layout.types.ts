/** Direction of the main axis in a flex layout. */
export type FlexDirection = "row" | "column" | "row-reverse" | "column-reverse";

/** Alignment of children along the main axis. */
export type JustifyContent =
  | "flex-start"
  | "center"
  | "flex-end"
  | "space-between"
  | "space-around"
  | "space-evenly";

/** Alignment of children along the cross axis. */
export type AlignItems = "flex-start" | "center" | "flex-end" | "stretch" | "baseline";

/** Alignment of a single child along the cross axis, overriding AlignItems. */
export type AlignSelf = "flex-start" | "center" | "flex-end" | "stretch" | "baseline";

/** Positioning strategy for a layout node. */
export type Position = "relative" | "absolute";

/** A dimension value — number for fixed columns/rows, string for percentage or calc. */
export type Sizing = number | string;

/** Behavior when content overflows the container bounds. */
export type Overflow = "visible" | "hidden" | "scroll";

/** Flex wrap behavior. */
export type FlexWrap = "nowrap" | "wrap";

/** Display mode. */
export type Display = "flex" | "none";

/** Shorthand for all four padding sides. */
export interface Padding {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

/** Shorthand for all four margin sides. */
export interface Margin {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

/** Shorthand for all four inset offsets (used with absolute positioning). */
export interface Inset {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

/** Row and column gap values for flex layouts. */
export interface Gap {
  row?: number;
  column?: number;
}

/** All layout-affecting properties for a UI node. */
export interface LayoutConstraints {
  display?: Display;
  flexDirection?: FlexDirection;
  justifyContent?: JustifyContent;
  alignItems?: AlignItems;
  alignSelf?: AlignSelf;
  flexWrap?: FlexWrap;
  flexGrow?: number;
  flexShrink?: number;
  flexBasis?: Sizing;
  gap?: number | Gap;
  padding?: number | Padding;
  paddingTop?: number;
  paddingRight?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  margin?: number | Margin;
  marginTop?: number;
  marginRight?: number;
  marginBottom?: number;
  marginLeft?: number;
  width?: Sizing;
  height?: Sizing;
  minWidth?: Sizing;
  maxWidth?: Sizing;
  minHeight?: Sizing;
  maxHeight?: Sizing;
  position?: Position;
  inset?: Inset;
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
  zIndex?: number;
  visible?: boolean;
  overflow?: Overflow;
}
