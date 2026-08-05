// Polyfills required for react-devtools-core in Node.js/Bun environments
// This file MUST be imported before react-devtools-core

// biome-ignore lint/suspicious/noExplicitAny: polyfill browser globals on globalThis
const g = globalThis as any;

// Polyfill WebSocket if not natively available
if (typeof g.WebSocket === "undefined") {
  try {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const ws = require("ws");
    g.WebSocket = ws.default || ws;
  } catch {
    // ws not installed - fallback gracefully if DevTools is not active
  }
}

// react-devtools-core expects browser-like globals
g.window = g.window || globalThis;
g.self = g.self || globalThis;

// Filter out internal component wrappers for clean DevTools view
g.window.__REACT_DEVTOOLS_COMPONENT_FILTERS__ = [
  {
    type: 2,
    value: "ErrorBoundary",
    isEnabled: true,
    isValid: true,
  },
];
