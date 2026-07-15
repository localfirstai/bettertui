import type { TerminalCapabilities } from "../platform/binding";

export interface TerminalCapabilitiesOptions {
  trueColor?: boolean;
  kittyKeyboard?: boolean;
  csiU?: boolean;
  bracketedPaste?: boolean;
  focusEvents?: boolean;
  mouse?: boolean;
  osc52?: boolean;
  osc8?: boolean;
  sync?: boolean;
  sgrPixel?: boolean;
  underlineColor?: boolean;
  strikethrough?: boolean;
  cursorStyle?: boolean;
  alternateScroll?: boolean;
  inlineImages?: boolean;
  sixel?: boolean;
  columns?: number;
  rows?: number;
  brand?: string;
}

export function createTerminalCapabilities(
  options: TerminalCapabilitiesOptions = {},
): TerminalCapabilities {
  return {
    brand: options.brand ?? "Test",
    true_color: options.trueColor ?? true,
    kitty_keyboard: options.kittyKeyboard ?? false,
    csi_u: options.csiU ?? false,
    bracketed_paste: options.bracketedPaste ?? true,
    focus_events: options.focusEvents ?? true,
    mouse: options.mouse ?? true,
    osc52: options.osc52 ?? false,
    osc8: options.osc8 ?? false,
    sync: options.sync ?? true,
    sgr_pixel: options.sgrPixel ?? false,
    underline_color: options.underlineColor ?? true,
    strikethrough: options.strikethrough ?? true,
    cursor_style: options.cursorStyle ?? true,
    alternate_scroll: options.alternateScroll ?? true,
    inline_images: options.inlineImages ?? false,
    sixel: options.sixel ?? false,
    columns: options.columns ?? 80,
    rows: options.rows ?? 24,
  };
}

export function createMinimalTerminalCapabilities(): TerminalCapabilities {
  return createTerminalCapabilities({
    trueColor: false,
    kittyKeyboard: false,
    csiU: false,
    bracketedPaste: false,
    focusEvents: false,
    mouse: false,
    osc52: false,
    osc8: false,
    sync: false,
    sgrPixel: false,
    underlineColor: false,
    strikethrough: false,
    cursorStyle: false,
    alternateScroll: false,
    inlineImages: false,
    sixel: false,
  });
}

export function createFullTerminalCapabilities(): TerminalCapabilities {
  return createTerminalCapabilities({
    trueColor: true,
    kittyKeyboard: true,
    csiU: true,
    bracketedPaste: true,
    focusEvents: true,
    mouse: true,
    osc52: true,
    osc8: true,
    sync: true,
    sgrPixel: true,
    underlineColor: true,
    strikethrough: true,
    cursorStyle: true,
    alternateScroll: true,
    inlineImages: true,
    sixel: true,
  });
}

export function createKittyTerminalCapabilities(): TerminalCapabilities {
  return createTerminalCapabilities({
    brand: "Kitty",
    trueColor: true,
    kittyKeyboard: true,
    csiU: true,
    bracketedPaste: true,
    focusEvents: true,
    mouse: true,
    osc52: true,
    osc8: true,
    sync: true,
    sgrPixel: true,
    underlineColor: true,
    strikethrough: true,
    cursorStyle: true,
    alternateScroll: true,
    inlineImages: true,
    sixel: false,
  });
}

export function createITerm2TerminalCapabilities(): TerminalCapabilities {
  return createTerminalCapabilities({
    brand: "iTerm2",
    trueColor: true,
    kittyKeyboard: false,
    csiU: true,
    bracketedPaste: true,
    focusEvents: true,
    mouse: true,
    osc52: true,
    osc8: true,
    sync: true,
    sgrPixel: false,
    underlineColor: true,
    strikethrough: true,
    cursorStyle: true,
    alternateScroll: true,
    inlineImages: true,
    sixel: false,
  });
}
