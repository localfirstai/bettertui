const singletonCacheSymbol = Symbol.for("@bettertui/core/singleton");

/**
 * Ensures a value is initialized once per process,
 * persists across hot reloads, and is type-safe.
 */
export function singleton<T>(key: string, factory: () => T): T {
  // biome-ignore lint/suspicious/noExplicitAny: globalThis symbol cache bag
  const g = globalThis as any;
  if (!g[singletonCacheSymbol]) {
    g[singletonCacheSymbol] = {};
  }
  const bag = g[singletonCacheSymbol];
  if (!(key in bag)) {
    bag[key] = factory();
  }
  return bag[key] as T;
}

export function getSingleton<T>(key: string): T | undefined {
  // biome-ignore lint/suspicious/noExplicitAny: globalThis symbol cache bag
  const g = globalThis as any;
  const bag = g[singletonCacheSymbol];
  return bag?.[key] as T | undefined;
}

export function destroySingleton(key: string): void {
  // biome-ignore lint/suspicious/noExplicitAny: globalThis symbol cache bag
  const g = globalThis as any;
  const bag = g[singletonCacheSymbol];
  if (bag && key in bag) {
    delete bag[key];
  }
}

export function hasSingleton(key: string): boolean {
  // biome-ignore lint/suspicious/noExplicitAny: globalThis symbol cache bag
  const g = globalThis as any;
  const bag = g[singletonCacheSymbol];
  return Boolean(bag && key in bag);
}
