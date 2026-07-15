import { dlopen, suffix } from "node:ffi";

type FfiFn = (...args: unknown[]) => unknown;

export type NativeLib<T> = {
  [K in keyof T]: FfiFn;
};

export function loadLibrary<T>(
  path: string,
  symbols: Record<string, { arguments: readonly string[]; returns: string }>,
): NativeLib<T> {
  const lib = dlopen(path, symbols);
  return lib.functions as NativeLib<T>;
}

export function getSuffix(): string {
  return suffix;
}
