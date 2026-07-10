import type { Theme } from "@bettertui/shared";

export const defaultTheme: Theme = {
  name: "default",
  colors: {
    primary: "#007acc",
    secondary: "#6c757d",
    success: "#28a745",
    warning: "#ffc107",
    danger: "#dc3545",
    background: "#1e1e1e",
    foreground: "#d4d4d4",
    border: "#3c3c3c",
  },
  borders: {
    style: "single",
    fg: "#666666",
  },
};

export function createTheme(overrides: Partial<Theme>): Theme {
  return { ...defaultTheme, ...overrides };
}

export type { Theme };
