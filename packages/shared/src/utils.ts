import {
  COLOR_REGEX,
  NAMED_COLORS,
  RGBA_REGEX,
  RGB_REGEX,
  VALID_ALIGN_ITEMS,
  VALID_ALIGN_SELVES,
  VALID_FLEX_DIRECTIONS,
  VALID_FLEX_WRAPS,
  VALID_JUSTIFY_CONTENTS,
  VALID_OVERFLOWS,
  VALID_POSITIONS,
} from "./consts";
import type {
  ColorValue,
  LayoutConstraints,
  Style,
  Theme,
  ValidationError,
  ValidationResult,
} from "./types";

/**
 * Deep-merge a partial theme into a base theme.
 * Only the provided keys in each subsection are overridden;
 * missing keys fall through to the base.
 *
 * @param base - The fallback theme (usually DEFAULT_THEME).
 * @param overrides - Partial theme values to merge in.
 * @returns A new Theme with overrides applied.
 */
export function mergeTheme(base: Theme, overrides: Partial<Theme>): Theme {
  return {
    ...base,
    ...overrides,
    colors: { ...base.colors, ...overrides.colors },
    spacing: { ...base.spacing, ...overrides.spacing },
    borders: { ...base.borders, ...overrides.borders },
  };
}

/**
 * Check whether a string is a valid CSS-like color value.
 * Supports named colors, hex (#RGB/#RRGGBB/#RRGGBBAA), rgb(), and rgba().
 *
 * @param color - The color string to validate.
 * @returns True if the color is recognized as valid.
 */
export function isValidColor(color: ColorValue): boolean {
  if (NAMED_COLORS.has(color.toLowerCase())) return true;
  if (COLOR_REGEX.test(color)) return true;
  if (RGB_REGEX.test(color)) return true;
  if (RGBA_REGEX.test(color)) return true;
  return false;
}

/**
 * Validate layout constraint values.
 * Checks numeric fields for finiteness, percentage strings for valid range,
 * and enum fields for allowed values.
 *
 * @param layout - A partial LayoutConstraints object to validate.
 * @returns An array of validation errors (empty if valid).
 */
export function validateLayoutConstraints(layout: Partial<LayoutConstraints>): ValidationError[] {
  const errors: ValidationError[] = [];

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

  if (
    layout.flexDirection !== undefined &&
    !VALID_FLEX_DIRECTIONS.includes(layout.flexDirection as (typeof VALID_FLEX_DIRECTIONS)[number])
  ) {
    errors.push({
      field: "flexDirection",
      message: `flexDirection must be one of: ${VALID_FLEX_DIRECTIONS.join(", ")}`,
    });
  }

  if (
    layout.justifyContent !== undefined &&
    !VALID_JUSTIFY_CONTENTS.includes(
      layout.justifyContent as (typeof VALID_JUSTIFY_CONTENTS)[number],
    )
  ) {
    errors.push({
      field: "justifyContent",
      message: `justifyContent must be one of: ${VALID_JUSTIFY_CONTENTS.join(", ")}`,
    });
  }

  if (
    layout.alignItems !== undefined &&
    !VALID_ALIGN_ITEMS.includes(layout.alignItems as (typeof VALID_ALIGN_ITEMS)[number])
  ) {
    errors.push({
      field: "alignItems",
      message: `alignItems must be one of: ${VALID_ALIGN_ITEMS.join(", ")}`,
    });
  }

  if (
    layout.alignSelf !== undefined &&
    !VALID_ALIGN_SELVES.includes(layout.alignSelf as (typeof VALID_ALIGN_SELVES)[number])
  ) {
    errors.push({
      field: "alignSelf",
      message: `alignSelf must be one of: ${VALID_ALIGN_SELVES.join(", ")}`,
    });
  }

  if (
    layout.position !== undefined &&
    !VALID_POSITIONS.includes(layout.position as (typeof VALID_POSITIONS)[number])
  ) {
    errors.push({
      field: "position",
      message: `position must be one of: ${VALID_POSITIONS.join(", ")}`,
    });
  }

  if (
    layout.overflow !== undefined &&
    !VALID_OVERFLOWS.includes(layout.overflow as (typeof VALID_OVERFLOWS)[number])
  ) {
    errors.push({
      field: "overflow",
      message: `overflow must be one of: ${VALID_OVERFLOWS.join(", ")}`,
    });
  }

  if (
    layout.flexWrap !== undefined &&
    !VALID_FLEX_WRAPS.includes(layout.flexWrap as (typeof VALID_FLEX_WRAPS)[number])
  ) {
    errors.push({
      field: "flexWrap",
      message: `flexWrap must be one of: ${VALID_FLEX_WRAPS.join(", ")}`,
    });
  }

  return errors;
}

/**
 * Validate style property values.
 * Currently checks foreground and background colors.
 *
 * @param style - A partial Style object to validate.
 * @returns An array of validation errors (empty if valid).
 */
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

/**
 * Run both layout and style validation, returning an aggregated result.
 *
 * @param layout - Optional partial LayoutConstraints to validate.
 * @param style - Optional partial Style to validate.
 * @returns A ValidationResult with combined errors.
 */
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

/**
 * Validate props and log a warning to the console if invalid.
 * No-op in production builds.
 *
 * @param layout - Optional partial LayoutConstraints to check.
 * @param style - Optional partial Style to check.
 * @param componentName - Optional component name for the warning message.
 */
export function warnIfInvalid(
  layout?: Partial<LayoutConstraints>,
  style?: Partial<Style>,
  componentName?: string,
): void {
  if (process.env["NODE_ENV"] === "production") return;

  const result = validate(layout, style);
  if (!result.valid) {
    const name = componentName ?? "Component";
    console.warn(`[${name}] Invalid props:`, result.errors);
  }
}

let nextId = 0;

/**
 * Generate a unique identifier string.
 * Each call increments an internal counter and returns the next value.
 *
 * @returns A monotonically increasing unique ID string.
 */
export function generateId(): string {
  return `${nextId++}`;
}
