/**
 * JSX runtime for @bettertui/solid.
 *
 * Re-exports `solid-js`'s JSX factory functions so that TypeScript + the Solid
 * Babel transform (`babel-preset-solid` with `generate: "universal"`) can
 * resolve `@bettertui/solid/jsx-runtime` when compiling user `.tsx` files.
 *
 * Set in the user's `tsconfig.json`:
 * ```json
 * { "compilerOptions": { "jsx": "preserve", "jsxImportSource": "@bettertui/solid" } }
 * ```
 */

// Side-effect: registers BetterTUI intrinsic elements onto the solid-js JSX namespace.
import "./types/jsx.types";

export {
  createComponent as jsx,
  createComponent as jsxs,
  createComponent as jsxDEV,
} from "solid-js";

// solid-js does not export Fragment natively — export a null sentinel so
// bundlers that look for Fragment in the JSX runtime don't crash.
export const Fragment = null;

export type { JSX } from "solid-js";
