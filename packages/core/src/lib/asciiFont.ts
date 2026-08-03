import block from "./fonts/block.json" with { type: "json" };
import grid from "./fonts/grid.json" with { type: "json" };
import huge from "./fonts/huge.json" with { type: "json" };
import pallet from "./fonts/pallet.json" with { type: "json" };
import shade from "./fonts/shade.json" with { type: "json" };
import slick from "./fonts/slick.json" with { type: "json" };
import tiny from "./fonts/tiny.json" with { type: "json" };
import { type ColorInput, parseColor } from "./rgba";

export type ASCIIFontName =
  | "tiny"
  | "block"
  | "shade"
  | "slick"
  | "huge"
  | "grid"
  | "pallet"
  | string;

type FontSegment = {
  text: string;
  colorIndex: number;
};

type FontDefinition = {
  name: string;
  lines: number;
  letterspace_size: number;
  letterspace: string[];
  colors?: number;
  chars: Record<string, string[]>;
};

type ParsedFontDefinition = {
  name: string;
  lines: number;
  letterspace_size: number;
  letterspace: string[];
  colors: number;
  chars: Record<string, FontSegment[][]>;
};

export const fonts: Record<string, FontDefinition> = {
  tiny: tiny as unknown as FontDefinition,
  block: block as unknown as FontDefinition,
  shade: shade as unknown as FontDefinition,
  slick: slick as unknown as FontDefinition,
  huge: huge as unknown as FontDefinition,
  grid: grid as unknown as FontDefinition,
  pallet: pallet as unknown as FontDefinition,
};

const parsedFonts: Record<string, ParsedFontDefinition> = {};

function parseColorTags(text: string): FontSegment[] {
  const segments: FontSegment[] = [];
  const colorTagRegex = /<c(\d+)>(.*?)<\/c\d+>/g;
  let lastIndex = 0;

  for (const match of text.matchAll(colorTagRegex)) {
    const matchIndex = match.index ?? 0;
    if (matchIndex > lastIndex) {
      const plainText = text.slice(lastIndex, matchIndex);
      if (plainText) {
        segments.push({ text: plainText, colorIndex: 0 });
      }
    }

    const colorStr = match[1];
    const taggedText = match[2] ?? "";
    const colorIndex = colorStr ? Number.parseInt(colorStr, 10) - 1 : 0;
    segments.push({ text: taggedText, colorIndex: Math.max(0, colorIndex) });

    lastIndex = matchIndex + match[0].length;
  }

  if (lastIndex < text.length) {
    const remainingText = text.slice(lastIndex);
    if (remainingText) {
      segments.push({ text: remainingText, colorIndex: 0 });
    }
  }

  return segments;
}

function getParsedFont(fontKey: string): ParsedFontDefinition | null {
  const key = fontKey.toLowerCase();
  const fontDef = fonts[key];
  if (!fontDef) return null;

  let parsed = parsedFonts[key];
  if (!parsed) {
    const parsedChars: Record<string, FontSegment[][]> = {};

    for (const [char, lines] of Object.entries(fontDef.chars)) {
      parsedChars[char] = lines.map((line) => parseColorTags(line));
    }

    parsed = {
      ...fontDef,
      colors: fontDef.colors || 1,
      chars: parsedChars,
    };
    parsedFonts[key] = parsed;
  }

  return parsed;
}

export function measureFontText(text: string, font = "tiny"): { width: number; height: number } {
  const fontDef = getParsedFont(font);
  if (!fontDef) {
    return { width: text.length, height: 1 };
  }

  let currentX = 0;

  for (let i = 0; i < text.length; i++) {
    const char = text[i]?.toUpperCase() ?? "";
    const charDef = fontDef.chars[char];

    if (!charDef) {
      const spaceChar = fontDef.chars[" "];
      if (spaceChar?.[0]) {
        let spaceWidth = 0;
        for (const segment of spaceChar[0]) {
          spaceWidth += segment.text.length;
        }
        currentX += spaceWidth;
      } else {
        currentX += 1;
      }
    } else {
      let charWidth = 0;
      if (charDef[0]) {
        for (const segment of charDef[0]) {
          charWidth += segment.text.length;
        }
      }
      currentX += charWidth;
    }

    if (i < text.length - 1) {
      currentX += fontDef.letterspace_size;
    }
  }

  return {
    width: currentX,
    height: fontDef.lines,
  };
}

export function renderFontToText(
  text: string,
  font = "tiny",
  color?: ColorInput | ColorInput[],
): string {
  const fontDef = getParsedFont(font);
  if (!fontDef) return text;

  const colors = Array.isArray(color) ? color : [color ?? "#FFFFFF"];
  const parsedColors = colors.map((c) => parseColor(c));

  const lineOutputs: string[] = Array.from({ length: fontDef.lines }, () => "");

  for (let i = 0; i < text.length; i++) {
    const char = text[i]?.toUpperCase() ?? "";
    const charDef = fontDef.chars[char];

    if (!charDef) {
      const spaceChar = fontDef.chars[" "];
      let spaceWidth = 0;
      if (spaceChar?.[0]) {
        for (const segment of spaceChar[0]) {
          spaceWidth += segment.text.length;
        }
      } else {
        spaceWidth = 1;
      }
      for (let l = 0; l < fontDef.lines; l++) {
        lineOutputs[l] += " ".repeat(spaceWidth);
      }
    } else {
      for (let l = 0; l < fontDef.lines; l++) {
        const segments = charDef[l] ?? [];
        for (const segment of segments) {
          const c = parsedColors[segment.colorIndex] ??
            parsedColors[0] ?? {
              r: 255,
              g: 255,
              b: 255,
              a: 255,
            };
          lineOutputs[l] += `\x1b[38;2;${c.r};${c.g};${c.b}m${segment.text}\x1b[0m`;
        }
      }
    }

    if (i < text.length - 1) {
      for (let l = 0; l < fontDef.lines; l++) {
        lineOutputs[l] += " ".repeat(fontDef.letterspace_size);
      }
    }
  }

  return lineOutputs.join("\n");
}
