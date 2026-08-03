/**
 * Serialises a TypeScript {@link LayoutConstraints} object into the flat JSON
 * payload expected by the Rust engine's `NativeEngine.setLayout` call.
 *
 * Extracted from cliRenderer.ts for modularity and testability.
 * The function is a pure transformation — it has no side effects.
 */

import type { LayoutConstraints } from "@bettertui/shared";

/**
 * When an explicit numeric `width` or `height` is set on a
 * node, `flexShrink` defaults to `0` rather than the CSS default of `1`.
 * This prevents fixed-size children from being squeezed by their flex
 * container, which is almost always the desired behaviour in terminal UIs.
 *
 * If the caller has explicitly set `flexShrink`, that value wins.
 */
function resolveFlexShrink(layout: LayoutConstraints): number | undefined {
  if (layout.flexShrink !== undefined) return layout.flexShrink;
  const hasExplicitWidth = typeof layout.width === "number";
  const hasExplicitHeight = typeof layout.height === "number";
  if (hasExplicitWidth || hasExplicitHeight) return 0;
  return undefined; // let the engine use its default (1)
}

export function layoutToEngineJson(layout: LayoutConstraints): Record<string, unknown> {
  const j: Record<string, unknown> = {};

  if (layout.flexDirection !== undefined) j.direction = layout.flexDirection;
  if (layout.flexWrap !== undefined) j.flex_wrap = layout.flexWrap;
  if (layout.justifyContent !== undefined) j.justify = layout.justifyContent;
  if (layout.alignItems !== undefined) j.align = layout.alignItems;
  if (layout.alignSelf !== undefined) j.align_self = layout.alignSelf;
  if (layout.alignContent !== undefined) j.align_content = layout.alignContent;
  if (layout.flexGrow !== undefined) j.flex_grow = layout.flexGrow;

  const flexShrink = resolveFlexShrink(layout);
  if (flexShrink !== undefined) j.flex_shrink = flexShrink;

  if (layout.flexBasis !== undefined) j.flex_basis = String(layout.flexBasis);
  if (layout.display !== undefined) j.display = layout.display;

  if (layout.width !== undefined) j.width = String(layout.width);
  if (layout.height !== undefined) j.height = String(layout.height);
  if (layout.minWidth !== undefined) j.min_width = String(layout.minWidth);
  if (layout.minHeight !== undefined) j.min_height = String(layout.minHeight);
  if (layout.maxWidth !== undefined) j.max_width = String(layout.maxWidth);
  if (layout.maxHeight !== undefined) j.max_height = String(layout.maxHeight);

  if (layout.position !== undefined) j.position = layout.position;
  if (layout.top !== undefined) j.top = layout.top;
  if (layout.right !== undefined) j.right = layout.right;
  if (layout.bottom !== undefined) j.bottom = layout.bottom;
  if (layout.left !== undefined) j.left = layout.left;
  if (layout.zIndex !== undefined) j.z_index = layout.zIndex;
  if (layout.overflow !== undefined) j.overflow = layout.overflow;

  // Inset shorthand — explicit per-edge values win (set above) so only fill
  // remaining edges from the shorthand object.
  if (layout.inset !== undefined) {
    if (j.top === undefined && layout.inset.top !== undefined) j.top = layout.inset.top;
    if (j.right === undefined && layout.inset.right !== undefined) j.right = layout.inset.right;
    if (j.bottom === undefined && layout.inset.bottom !== undefined) j.bottom = layout.inset.bottom;
    if (j.left === undefined && layout.inset.left !== undefined) j.left = layout.inset.left;
  }

  // Padding
  const pt =
    layout.paddingTop ??
    (typeof layout.padding === "number" ? layout.padding : layout.padding?.top);
  const pr =
    layout.paddingRight ??
    (typeof layout.padding === "number" ? layout.padding : layout.padding?.right);
  const pb =
    layout.paddingBottom ??
    (typeof layout.padding === "number" ? layout.padding : layout.padding?.bottom);
  const pl =
    layout.paddingLeft ??
    (typeof layout.padding === "number" ? layout.padding : layout.padding?.left);
  if (pt !== undefined) j.padding_top = pt;
  if (pr !== undefined) j.padding_right = pr;
  if (pb !== undefined) j.padding_bottom = pb;
  if (pl !== undefined) j.padding_left = pl;

  // Margin
  const mt =
    layout.marginTop ?? (typeof layout.margin === "number" ? layout.margin : layout.margin?.top);
  const mr =
    layout.marginRight ??
    (typeof layout.margin === "number" ? layout.margin : layout.margin?.right);
  const mb =
    layout.marginBottom ??
    (typeof layout.margin === "number" ? layout.margin : layout.margin?.bottom);
  const ml =
    layout.marginLeft ?? (typeof layout.margin === "number" ? layout.margin : layout.margin?.left);
  if (mt !== undefined) j.margin_top = mt;
  if (mr !== undefined) j.margin_right = mr;
  if (mb !== undefined) j.margin_bottom = mb;
  if (ml !== undefined) j.margin_left = ml;

  // Gap
  const gapVal = layout.gap;
  if (gapVal !== undefined) {
    if (typeof gapVal === "number") {
      j.gap_row = gapVal;
      j.gap_column = gapVal;
    } else {
      if (gapVal.row !== undefined) j.gap_row = gapVal.row;
      if (gapVal.column !== undefined) j.gap_column = gapVal.column;
    }
  }

  // Border layout contribution (applies when box-sizing: border-box is active,
  // which is the default). Typically 0 or 1 cell per bordered side.
  const bt = layout.borderTop;
  const br = layout.borderRight;
  const bb = layout.borderBottom;
  const bl = layout.borderLeft;
  if (bt !== undefined) j.border_top = bt;
  if (br !== undefined) j.border_right = br;
  if (bb !== undefined) j.border_bottom = bb;
  if (bl !== undefined) j.border_left = bl;

  return j;
}
