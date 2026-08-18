/** Determine which edge or corner of a rectangular box a mouse position is on. */
export function getResizeDirection(
  mouseX: number,
  mouseY: number,
  boxLeft: number,
  boxTop: number,
  boxWidth: number,
  boxHeight: number,
): "nw" | "ne" | "sw" | "se" | "n" | "s" | "w" | "e" | null {
  const onLeftBorder = mouseX === boxLeft;
  const onRightBorder = mouseX === boxLeft + boxWidth - 1;
  const onTopBorder = mouseY === boxTop;
  const onBottomBorder = mouseY === boxTop + boxHeight - 1;

  const withinHorizontalBounds = mouseX >= boxLeft && mouseX <= boxLeft + boxWidth - 1;
  const withinVerticalBounds = mouseY >= boxTop && mouseY <= boxTop + boxHeight - 1;

  const left = onLeftBorder && withinVerticalBounds;
  const right = onRightBorder && withinVerticalBounds;
  const top = onTopBorder && withinHorizontalBounds;
  const bottom = onBottomBorder && withinHorizontalBounds;

  if (top && left) return "nw";
  if (top && right) return "ne";
  if (bottom && left) return "sw";
  if (bottom && right) return "se";
  if (top) return "n";
  if (bottom) return "s";
  if (left) return "w";
  if (right) return "e";

  return null;
}
