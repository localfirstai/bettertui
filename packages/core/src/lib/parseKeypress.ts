// Traditional terminal keypress parser (CSI/SS3/meta sequences)
// Handles: arrows, F-keys, Home/End, PageUp/Down, modifier combos, meta keys

import { Buffer } from "node:buffer";
import { kittyNamedSingleStrokeKeys, parseKittyKeyboard } from "./parseKeypressKitty";

const ESC = "\x1b";

const metaKeyCodeRe = new RegExp(`^(?:${ESC})([a-zA-Z0-9])$`);

const fnKeyRe = new RegExp(
  `^(?:${ESC}+)(O|N|\\[|\\[\\[)(?:(\\d+)(?:;(\\d+))?([~^$])|(?:1;)?(\\d+)?([a-zA-Z]))`,
);

const modifyOtherKeysRe = new RegExp(`^${ESC}\\[27;(\\d+);(\\d+)~$`);

const mouseSgrCompleteRe = new RegExp(`^${ESC}\\[<\\d+;\\d+;\\d+[Mm]$`);
const mouseSgrPartialRe = new RegExp(`^${ESC}\\[<[\\d;]*$`);
const mouseSgrPartialNoEscRe = /^\[<\d+;\d+;\d+[Mm]$/;
const mouseSgrPartialNoEsc2Re = /^\[<[\d;]*$/;

const termResponseWindowSizeRe = new RegExp(`^${ESC}\\[\\d+;\\d+;\\d+t$`);
const termResponseCprRe = new RegExp(`^${ESC}\\[\\d+;\\d+R$`);
const termResponseDaRe = new RegExp(`^${ESC}\\[\\?[\\d;]+c$`);
const termResponseModeRe = new RegExp(`^${ESC}\\[\\?[\\d;]+\\$y$`);
const termResponseOscRe = new RegExp(`^${ESC}\\][\\d;].*(${ESC}\\\\|\x07)$`);

const keyName: Record<string, string> = {
  /* xterm/gnome ESC O letter */
  OP: "f1",
  OQ: "f2",
  OR: "f3",
  OS: "f4",
  /* xterm/rxvt ESC [ number ~ */
  "[11~": "f1",
  "[12~": "f2",
  "[13~": "f3",
  "[14~": "f4",
  /* from Cygwin and used in libuv */
  "[[A": "f1",
  "[[B": "f2",
  "[[C": "f3",
  "[[D": "f4",
  "[[E": "f5",
  /* common */
  "[15~": "f5",
  "[17~": "f6",
  "[18~": "f7",
  "[19~": "f8",
  "[20~": "f9",
  "[21~": "f10",
  "[23~": "f11",
  "[24~": "f12",
  "[29~": "menu",
  "[57427~": "clear",
  /* xterm ESC [ letter */
  "[A": "up",
  "[B": "down",
  "[C": "right",
  "[D": "left",
  "[E": "clear",
  "[F": "end",
  "[H": "home",
  "[P": "f1",
  "[Q": "f2",
  "[S": "f4",
  /* xterm/gnome ESC O letter */
  OA: "up",
  OB: "down",
  OC: "right",
  OD: "left",
  OE: "clear",
  OF: "end",
  OH: "home",
  /* VT100 application keypad (SS3) — sent when terminal enables DECKPAM (ESC =).
   * macOS Terminal.app and other xterm-based terminals emit these when running
   * full-screen apps with the alternate screen. */
  OM: "return",
  Oj: "*",
  Ok: "+",
  Ol: ",",
  Om: "-",
  On: ".",
  Oo: "/",
  Op: "0",
  Oq: "1",
  Or: "2",
  Os: "3",
  Ot: "4",
  Ou: "5",
  Ov: "6",
  Ow: "7",
  Ox: "8",
  Oy: "9",
  OX: "=",
  /* xterm/rxvt ESC [ number ~ */
  "[1~": "home",
  "[2~": "insert",
  "[3~": "delete",
  "[4~": "end",
  "[5~": "pageup",
  "[6~": "pagedown",
  /* putty */
  "[[5~": "pageup",
  "[[6~": "pagedown",
  /* rxvt */
  "[7~": "home",
  "[8~": "end",
  /* rxvt keys with modifiers */
  "[a": "up",
  "[b": "down",
  "[c": "right",
  "[d": "left",
  "[e": "clear",
  /* option + arrow keys (old style) */
  f: "right",
  b: "left",
  p: "up",
  n: "down",
  "[2$": "insert",
  "[3$": "delete",
  "[5$": "pageup",
  "[6$": "pagedown",
  "[7$": "home",
  "[8$": "end",
  Oa: "up",
  Ob: "down",
  Oc: "right",
  Od: "left",
  Oe: "clear",
  "[2^": "insert",
  "[3^": "delete",
  "[5^": "pageup",
  "[6^": "pagedown",
  "[7^": "home",
  "[8^": "end",
  /* misc. */
  "[Z": "tab",
};

export const nonAlphanumericKeys = [...Object.values(keyName), "backspace"];

export const terminalNamedSingleStrokeKeys = [
  ...new Set([
    "return",
    "linefeed",
    "tab",
    "escape",
    "space",
    ...nonAlphanumericKeys,
    ...kittyNamedSingleStrokeKeys,
  ]),
];

const isShiftKey = (code: string) => {
  return ["[a", "[b", "[c", "[d", "[e", "[2$", "[3$", "[5$", "[6$", "[7$", "[8$", "[Z"].includes(
    code,
  );
};

const isCtrlKey = (code: string) => {
  return ["Oa", "Ob", "Oc", "Od", "Oe", "[2^", "[3^", "[5^", "[6^", "[7^", "[8^"].includes(code);
};

const getCtrlKeyName = (charCode: number): string | undefined => {
  if (charCode === 0) {
    return "space";
  }

  if (charCode >= 1 && charCode <= 26) {
    return String.fromCharCode(charCode + "a".charCodeAt(0) - 1);
  }

  if (charCode >= 28 && charCode <= 31) {
    return String.fromCharCode(charCode + 64);
  }

  return undefined;
};

export type KeyEventType = "press" | "repeat" | "release";

export interface ParsedKey {
  name: string;
  ctrl: boolean;
  meta: boolean;
  shift: boolean;
  option: boolean;
  sequence: string;
  number: boolean;
  raw: string;
  eventType: KeyEventType;
  source: "raw" | "kitty";
  code?: string;
  super?: boolean;
  hyper?: boolean;
  capsLock?: boolean;
  numLock?: boolean;
  baseCode?: number;
  repeated?: boolean;
}

export type ParseKeypressOptions = {
  useKittyKeyboard?: boolean;
};

// Printable characters for VT100 SS3 application-keypad sequences.
const ss3NumpadPrintable: Record<string, string> = {
  Op: "0",
  Oq: "1",
  Or: "2",
  Os: "3",
  Ot: "4",
  Ou: "5",
  Ov: "6",
  Ow: "7",
  Ox: "8",
  Oy: "9",
  Oj: "*",
  Ok: "+",
  Ol: ",",
  Om: "-",
  On: ".",
  Oo: "/",
  OX: "=",
};

export const parseKeypress = (
  input: Buffer | string = "",
  options: ParseKeypressOptions = {},
): ParsedKey | null => {
  let str: string;

  if (Buffer.isBuffer(input)) {
    const firstByte = input[0];
    if (firstByte !== undefined && firstByte > 127 && input[1] === undefined) {
      const modifiedBuffer = Buffer.from(input);
      modifiedBuffer[0] = firstByte - 128;
      str = `${ESC}${String(modifiedBuffer)}`;
    } else {
      str = String(input);
    }
  } else if (input !== undefined && typeof input !== "string") {
    str = String(input);
  } else {
    str = input || "";
  }

  // Filter out mouse events (SGR and basic)
  if (mouseSgrCompleteRe.test(str)) {
    return null;
  }
  if (mouseSgrPartialNoEscRe.test(str)) {
    return null;
  }
  if (mouseSgrPartialRe.test(str)) {
    return null;
  }
  if (mouseSgrPartialNoEsc2Re.test(str)) {
    return null;
  }
  if (str.startsWith(`${ESC}[M`) && str.length >= 6) {
    return null;
  }

  // Filter out terminal response sequences (not keyboard events)
  if (termResponseWindowSizeRe.test(str)) {
    return null;
  }
  if (termResponseCprRe.test(str)) {
    return null;
  }
  if (termResponseDaRe.test(str)) {
    return null;
  }
  if (termResponseModeRe.test(str)) {
    return null;
  }
  if (str === `${ESC}[I` || str === `${ESC}[O`) {
    return null;
  }
  if (termResponseOscRe.test(str)) {
    return null;
  }
  if (str === `${ESC}[200~` || str === `${ESC}[201~`) {
    return null;
  }

  const key: ParsedKey = {
    name: "",
    ctrl: false,
    meta: false,
    shift: false,
    option: false,
    number: false,
    sequence: str,
    raw: str,
    eventType: "press",
    source: "raw",
  };

  key.sequence = key.sequence || str || key.name;

  const ctrlKeyName = str.length === 1 ? getCtrlKeyName(str.charCodeAt(0)) : undefined;
  const metaCtrlKeyName =
    str.length === 2 && str[0] === ESC ? getCtrlKeyName(str.charCodeAt(1)) : undefined;

  // Check for Kitty keyboard protocol if enabled
  if (options.useKittyKeyboard) {
    const kittyResult = parseKittyKeyboard(str);
    if (kittyResult) {
      return kittyResult;
    }
  }

  // Check for modifyOtherKeys sequences (CSI u protocol variant)
  const modifyOtherKeysMatch = modifyOtherKeysRe.exec(str);
  if (modifyOtherKeysMatch) {
    const modifierStr = modifyOtherKeysMatch[1];
    const charStr = modifyOtherKeysMatch[2];
    if (modifierStr && charStr) {
      const modifier = Number.parseInt(modifierStr, 10) - 1;
      const charCode = Number.parseInt(charStr, 10);

      key.ctrl = (modifier & 4) !== 0;
      key.meta = (modifier & 2) !== 0;
      key.shift = (modifier & 1) !== 0;
      key.option = (modifier & 2) !== 0;
      key.super = (modifier & 8) !== 0;
      key.hyper = (modifier & 16) !== 0;

      if (charCode === 13) {
        key.name = "return";
      } else if (charCode === 27) {
        key.name = "escape";
      } else if (charCode === 9) {
        key.name = "tab";
      } else if (charCode === 32) {
        key.name = "space";
      } else if (charCode === 127 || charCode === 8) {
        key.name = "backspace";
      } else {
        const char = String.fromCharCode(charCode);
        key.name = char;
        key.sequence = char;
        if (charCode >= 48 && charCode <= 57) {
          key.number = true;
        }
      }

      return key;
    }
  }

  if (str === "\r" || str === `${ESC}\r`) {
    key.name = "return";
    key.meta = str.length === 2;
  } else if (str === "\n" || str === `${ESC}\n`) {
    key.name = "linefeed";
    key.meta = str.length === 2;
  } else if (str === "\t") {
    key.name = "tab";
  } else if (str === "\b" || str === `${ESC}\b` || str === "\x7f" || str === `${ESC}\x7f`) {
    key.name = "backspace";
    key.meta = str.charAt(0) === ESC;
  } else if (str === ESC || str === `${ESC}${ESC}`) {
    key.name = "escape";
    key.meta = str.length === 2;
  } else if (str === " " || str === `${ESC} `) {
    key.name = "space";
    key.meta = str.length === 2;
  } else if (ctrlKeyName) {
    key.name = ctrlKeyName;
    key.ctrl = true;
  } else if (str.length === 1 && str >= "0" && str <= "9") {
    key.name = str;
    key.number = true;
  } else if (str.length === 1 && str >= "a" && str <= "z") {
    key.name = str;
  } else if (str.length === 1 && str >= "A" && str <= "Z") {
    key.name = str.toLowerCase();
    key.shift = true;
  } else if (str.length === 1 || (str.length === 2 && (str.codePointAt(0) ?? 0) > 0xffff)) {
    key.name = str;
  } else {
    const metaMatch = metaKeyCodeRe.exec(str);
    if (metaMatch) {
      key.meta = true;
      const char = metaMatch[1];
      if (char) {
        const isUpperCase = /^[A-Z]$/.test(char);

        if (char === "F") {
          key.name = "right";
        } else if (char === "B") {
          key.name = "left";
        } else if (isUpperCase) {
          key.shift = true;
          key.name = char;
        } else {
          key.name = char;
        }
      }
    } else if (metaCtrlKeyName) {
      key.meta = true;
      key.ctrl = true;
      key.name = metaCtrlKeyName;
    } else {
      const fnMatch = fnKeyRe.exec(str);
      if (fnMatch) {
        const segs = [...str];

        if (segs[0] === ESC && segs[1] === ESC) {
          key.option = true;
          key.meta = true;
        }

        const code = [fnMatch[1], fnMatch[2], fnMatch[4], fnMatch[6]].filter(Boolean).join("");

        const modifier = Number.parseInt(fnMatch[3] || fnMatch[5] || "1", 10) - 1;

        key.ctrl = key.ctrl || (modifier & 4) !== 0;
        key.meta = key.meta || (modifier & 2) !== 0;
        key.shift = key.shift || (modifier & 1) !== 0;
        key.option = key.option || (modifier & 2) !== 0;
        key.super = (modifier & 8) !== 0;
        key.hyper = (modifier & 16) !== 0;
        key.code = code;

        const keyNameResult = keyName[code];
        if (keyNameResult) {
          key.name = keyNameResult;
          key.shift = isShiftKey(code) || key.shift;
          key.ctrl = isCtrlKey(code) || key.ctrl;

          const ss3Char = ss3NumpadPrintable[code];
          if (ss3Char !== undefined) {
            key.sequence = ss3Char;
            if (key.name >= "0" && key.name <= "9") {
              key.number = true;
            }
          }
        } else {
          key.name = "";
        }
      } else if (str === `${ESC}[3~`) {
        key.name = "delete";
        key.meta = false;
        key.code = "[3~";
      }
    }
  }

  return key;
};
