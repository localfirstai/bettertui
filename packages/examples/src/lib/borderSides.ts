import type { BorderSide } from "@bettertui/core";

/** Convert a sides mask into a border value: true for all sides, false for none, else the active sides. */
export function getBorderFromSides(sides: {
  top: boolean;
  right: boolean;
  bottom: boolean;
  left: boolean;
}): boolean | BorderSide[] {
  const result: BorderSide[] = [];
  if (sides.top) result.push("top");
  if (sides.right) result.push("right");
  if (sides.bottom) result.push("bottom");
  if (sides.left) result.push("left");
  if (result.length === 4) return true;
  if (result.length === 0) return false;
  return result;
}
