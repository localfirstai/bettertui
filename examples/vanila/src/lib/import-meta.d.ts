// Type declaration for `import.meta.main` so pnpm typecheck passes.
// Bun sets `import.meta.main = true` when the file is the entry point.
// tsdown strips it in the bundle, and it's always false on static import.
interface ImportMeta {
  readonly main: boolean;
}
