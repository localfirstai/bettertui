import { describe, expect, it } from "vitest";

import { wordBoundaryLeft, wordBoundaryRight } from "../lib/wordBoundary";

describe("wordBoundaryLeft", () => {
  it("returns 0 for pos=0", () => {
    expect(wordBoundaryLeft("hello world", 0)).toBe(0);
  });

  it("skips whitespace then non-whitespace backward from end", () => {
    // "hello world" at pos 11 (end) → skip no whitespace, skip "world" → 6
    expect(wordBoundaryLeft("hello world", 11)).toBe(6);
  });

  it("jumps to start of current word from middle of word", () => {
    // "hello world" at pos 8 (inside "world") → skip no whitespace, skip "wo" → 6
    expect(wordBoundaryLeft("hello world", 8)).toBe(6);
  });

  it("handles multiple spaces between words", () => {
    // "one  two" at pos 5 → skip two spaces (pos 5,4 → chars[4]=' ', chars[3]=' '),
    // then skip "one" → 0
    // Wait: pos=5 is 't'. chars[4]=' ', so no whitespace to skip.
    // chars[4]=' ' — skip whitespace: i goes 5→4→3. Then skip non-whitespace "one": 3→0.
    expect(wordBoundaryLeft("one  two", 5)).toBe(0);
  });

  it("skips whitespace cluster then previous word", () => {
    // "one  two" at pos 3 → chars[2]='e' (non-ws), skip no whitespace,
    // skip "one" backward → 0
    expect(wordBoundaryLeft("one  two", 3)).toBe(0);
  });

  it("returns 0 for empty string", () => {
    expect(wordBoundaryLeft("", 0)).toBe(0);
  });
});

describe("wordBoundaryRight", () => {
  it("skips to end of first word from start", () => {
    // "hello world" at pos 0 → skip no whitespace, skip "hello" → 5
    expect(wordBoundaryRight("hello world", 0)).toBe(5);
  });

  it("skips whitespace then to end of next word", () => {
    // "hello world" at pos 5 → skip ' ' (whitespace), skip "world" → 11
    expect(wordBoundaryRight("hello world", 5)).toBe(11);
  });

  it("returns text length when already at end", () => {
    expect(wordBoundaryRight("hello world", 11)).toBe(11);
  });

  it("handles multiple spaces between words", () => {
    // "one  two" at pos 3 → skip '  ' (two spaces), skip "two" → 8
    expect(wordBoundaryRight("one  two", 3)).toBe(8);
  });

  it("returns 0 for empty string", () => {
    expect(wordBoundaryRight("", 0)).toBe(0);
  });
});

describe("integration: back-and-forth navigation", () => {
  it("navigates left then right to consistent word boundaries", () => {
    const text = "hello world";
    // Start at end (11), go left → 6 (start of "world")
    const left1 = wordBoundaryLeft(text, text.length);
    expect(left1).toBe(6);
    // Go right from 6 → 11 (end of "world")
    const right1 = wordBoundaryRight(text, left1);
    expect(right1).toBe(11);
  });

  it("walks left word by word through 'one two'", () => {
    const text = "one two";
    // From end (7) → boundary_left → 4 (start of "two")
    const left1 = wordBoundaryLeft(text, 7);
    expect(left1).toBe(4);
    // From 4 → boundary_left → 0 (start of "one")
    const left2 = wordBoundaryLeft(text, left1);
    expect(left2).toBe(0);
  });
});
