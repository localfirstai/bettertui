/** A 2D coordinate in the terminal grid. */
export interface Point {
  /** Column position (0-indexed from left) */
  x: number;
  /** Row position (0-indexed from top) */
  y: number;
}

/** Width and height dimensions. */
export interface Size {
  /** Width in columns */
  width: number;
  /** Height in rows */
  height: number;
}

/** A rectangular region defined by position and size. */
export interface Rect {
  /** Left column offset */
  x: number;
  /** Top row offset */
  y: number;
  /** Width in columns */
  width: number;
  /** Height in rows */
  height: number;
}
