import type { TerminalCapabilities } from "./types";

export interface CapabilityInspectorOptions {
  onCapabilitiesDetected?: ((caps: TerminalCapabilities) => void) | undefined;
}

const DEFAULT_CAPABILITIES: TerminalCapabilities = {
  trueColor: false,
  kittyKeyboard: false,
  mouseSupport: false,
  osc52: false,
  osc8: false,
  pixelSupport: false,
  alternateScreen: false,
  terminalBrand: "unknown",
  terminalSize: { columns: 80, rows: 24 },
  syncUpdate: false,
  bracketedPaste: false,
  focusEvents: false,
  strikethrough: false,
  underlineColor: false,
  cursorStyle: false,
  hyperlinks: false,
  inlineImages: false,
  sixel: false,
};

export class CapabilityInspector {
  private capabilities: TerminalCapabilities = { ...DEFAULT_CAPABILITIES };
  private onCapabilitiesDetected: ((caps: TerminalCapabilities) => void) | undefined;

  constructor(options: CapabilityInspectorOptions = {}) {
    this.onCapabilitiesDetected = options.onCapabilitiesDetected;
  }

  update(capabilities: Partial<TerminalCapabilities>): void {
    Object.assign(this.capabilities, capabilities);
    this.onCapabilitiesDetected?.(this.capabilities);
  }

  updateFromNative(capabilitiesJson: string): void {
    try {
      const parsed = JSON.parse(capabilitiesJson) as Partial<TerminalCapabilities>;
      this.update(parsed);
    } catch {
      // Malformed capabilities JSON, ignore
    }
  }

  get(): TerminalCapabilities {
    return { ...this.capabilities };
  }

  has(capability: keyof TerminalCapabilities): boolean {
    const value = this.capabilities[capability];
    if (typeof value === "boolean") return value;
    if (typeof value === "string") return value !== "unknown" && value !== "";
    if (typeof value === "object" && value !== null) return true;
    /* c8 ignore next 2 — return false is unreachable: all capabilities are boolean, string, or object */
    return false;
  }

  getSummary(): string[] {
    const features: string[] = [];
    if (this.capabilities.trueColor) features.push("trueColor");
    if (this.capabilities.kittyKeyboard) features.push("kittyKeyboard");
    if (this.capabilities.mouseSupport) features.push("mouse");
    if (this.capabilities.osc52) features.push("osc52");
    if (this.capabilities.osc8) features.push("osc8");
    if (this.capabilities.pixelSupport) features.push("pixel");
    if (this.capabilities.alternateScreen) features.push("altScreen");
    if (this.capabilities.syncUpdate) features.push("sync");
    if (this.capabilities.bracketedPaste) features.push("bracketedPaste");
    if (this.capabilities.focusEvents) features.push("focusEvents");
    if (this.capabilities.strikethrough) features.push("strikethrough");
    if (this.capabilities.underlineColor) features.push("underlineColor");
    if (this.capabilities.cursorStyle) features.push("cursorStyle");
    if (this.capabilities.hyperlinks) features.push("hyperlinks");
    if (this.capabilities.inlineImages) features.push("inlineImages");
    if (this.capabilities.sixel) features.push("sixel");
    return features;
  }

  clear(): void {
    this.capabilities = { ...DEFAULT_CAPABILITIES };
  }
}
