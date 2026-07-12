// ─── Validation Utilities ────────────────────────────────

import type { ColorValue, LayoutConstraints, Style } from "@bettertui/shared";

// ─── Color Validation ────────────────────────────────────

const COLOR_REGEX = /^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/;
const RGB_REGEX = /^rgb\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*\)$/;
const RGBA_REGEX = /^rgba\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*[\d.]+\s*\)$/;
const NAMED_COLORS = new Set([
  "black",
  "white",
  "red",
  "green",
  "blue",
  "yellow",
  "cyan",
  "magenta",
  "gray",
  "grey",
  "transparent",
]);

export function isValidColor(color: ColorValue): boolean {
  if (NAMED_COLORS.has(color.toLowerCase())) return true;
  if (COLOR_REGEX.test(color)) return true;
  if (RGB_REGEX.test(color)) return true;
  if (RGBA_REGEX.test(color)) return true;
  return false;
}

// ─── Layout Validation ───────────────────────────────────

export type ValidationError = {
  field: string;
  message: string;
};

export function validateLayoutConstraints(layout: Partial<LayoutConstraints>): ValidationError[] {
  const errors: ValidationError[] = [];

  // Validate numeric values
  const numericFields = [
    "flexGrow",
    "flexShrink",
    "padding",
    "margin",
    "width",
    "height",
    "minWidth",
    "maxWidth",
    "minHeight",
    "maxHeight",
    "top",
    "right",
    "bottom",
    "left",
    "zIndex",
  ] as const;

  for (const field of numericFields) {
    const value = layout[field];
    if (value !== undefined && typeof value === "number") {
      if (Number.isNaN(value) || !Number.isFinite(value)) {
        errors.push({ field, message: `${field} must be a finite number` });
      }
    }
  }

  // Validate percentage values
  const percentageFields = [
    "width",
    "height",
    "minWidth",
    "maxWidth",
    "minHeight",
    "maxHeight",
  ] as const;
  for (const field of percentageFields) {
    const value = layout[field];
    if (typeof value === "string") {
      if (!value.endsWith("%")) {
        errors.push({ field, message: `${field} string value must be a percentage (e.g., "50%")` });
      } else {
        const num = Number.parseFloat(value);
        if (Number.isNaN(num) || num < 0 || num > 100) {
          errors.push({ field, message: `${field} percentage must be between 0% and 100%` });
        }
      }
    }
  }

  // Validate enum values
  if (layout.flexDirection !== undefined) {
    const valid = ["row", "column", "row-reverse", "column-reverse"];
    if (!valid.includes(layout.flexDirection)) {
      errors.push({
        field: "flexDirection",
        message: `flexDirection must be one of: ${valid.join(", ")}`,
      });
    }
  }

  if (layout.justifyContent !== undefined) {
    const valid = [
      "flex-start",
      "center",
      "flex-end",
      "space-between",
      "space-around",
      "space-evenly",
    ];
    if (!valid.includes(layout.justifyContent)) {
      errors.push({
        field: "justifyContent",
        message: `justifyContent must be one of: ${valid.join(", ")}`,
      });
    }
  }

  if (layout.alignItems !== undefined) {
    const valid = ["flex-start", "center", "flex-end", "stretch", "baseline"];
    if (!valid.includes(layout.alignItems)) {
      errors.push({
        field: "alignItems",
        message: `alignItems must be one of: ${valid.join(", ")}`,
      });
    }
  }

  if (layout.alignSelf !== undefined) {
    const valid = ["flex-start", "center", "flex-end", "stretch", "baseline"];
    if (!valid.includes(layout.alignSelf)) {
      errors.push({ field: "alignSelf", message: `alignSelf must be one of: ${valid.join(", ")}` });
    }
  }

  if (layout.position !== undefined) {
    const valid = ["relative", "absolute"];
    if (!valid.includes(layout.position)) {
      errors.push({ field: "position", message: `position must be one of: ${valid.join(", ")}` });
    }
  }

  if (layout.overflow !== undefined) {
    const valid = ["visible", "hidden", "scroll"];
    if (!valid.includes(layout.overflow)) {
      errors.push({ field: "overflow", message: `overflow must be one of: ${valid.join(", ")}` });
    }
  }

  if (layout.flexWrap !== undefined) {
    const valid = ["nowrap", "wrap"];
    if (!valid.includes(layout.flexWrap)) {
      errors.push({ field: "flexWrap", message: `flexWrap must be one of: ${valid.join(", ")}` });
    }
  }

  return errors;
}

// ─── Style Validation ────────────────────────────────────

export function validateStyle(style: Partial<Style>): ValidationError[] {
  const errors: ValidationError[] = [];

  if (style.fg !== undefined && !isValidColor(style.fg)) {
    errors.push({ field: "fg", message: `Invalid foreground color: ${style.fg}` });
  }

  if (style.bg !== undefined && !isValidColor(style.bg)) {
    errors.push({ field: "bg", message: `Invalid background color: ${style.bg}` });
  }

  return errors;
}

// ─── Validation Result ───────────────────────────────────

export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

export function validate(
  layout?: Partial<LayoutConstraints>,
  style?: Partial<Style>,
): ValidationResult {
  const errors: ValidationError[] = [];

  if (layout) {
    errors.push(...validateLayoutConstraints(layout));
  }

  if (style) {
    errors.push(...validateStyle(style));
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}

// ─── Development-mode warnings ───────────────────────────

export function warnIfInvalid(
  layout?: Partial<LayoutConstraints>,
  style?: Partial<Style>,
  componentName?: string,
): void {
  if (process.env.NODE_ENV === "production") return;

  const result = validate(layout, style);
  if (!result.valid) {
    const name = componentName ?? "Component";
    console.warn(`[${name}] Invalid props:`, result.errors);
  }
}
