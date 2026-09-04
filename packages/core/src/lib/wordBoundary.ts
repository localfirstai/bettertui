/**
 * Find the left (backward) word boundary from a given position.
 *
 * Ports the Rust `TextBuffer::word_boundary_left` algorithm exactly:
 * 1. If pos is 0, return 0 immediately.
 * 2. Starting at pos, skip whitespace characters backward.
 * 3. Then skip non-whitespace characters backward.
 * 4. Return the resulting index.
 *
 * This means the cursor lands at the start of the previous word,
 * or at the start of the current word if pos was inside a word
 * preceded by whitespace.
 *
 * @param text - The full text string to navigate.
 * @param pos  - The current cursor position (0-based character index).
 * @returns The character index of the left word boundary.
 */
export function wordBoundaryLeft(text: string, pos: number): number {
  if (pos === 0) {
    return 0;
  }

  const chars = [...text];
  let i = pos;

  // Skip whitespace backward
  while (i > 0 && /\s/.test(chars[i - 1])) {
    i -= 1;
  }

  // Skip non-whitespace backward
  while (i > 0 && !/\s/.test(chars[i - 1])) {
    i -= 1;
  }

  return i;
}

/**
 * Find the right (forward) word boundary from a given position.
 *
 * Ports the Rust `TextBuffer::word_boundary_right` algorithm exactly:
 * 1. Starting at pos, skip whitespace characters forward.
 * 2. Then skip non-whitespace characters forward.
 * 3. Return the resulting index.
 *
 * This means the cursor lands just past the end of the next word,
 * or just past the end of the current word if pos was inside one.
 *
 * @param text - The full text string to navigate.
 * @param pos  - The current cursor position (0-based character index).
 * @returns The character index of the right word boundary.
 */
export function wordBoundaryRight(text: string, pos: number): number {
  const chars = [...text];
  let i = pos;

  // Skip whitespace forward
  while (i < chars.length && /\s/.test(chars[i])) {
    i += 1;
  }

  // Skip non-whitespace forward
  while (i < chars.length && !/\s/.test(chars[i])) {
    i += 1;
  }

  return i;
}
