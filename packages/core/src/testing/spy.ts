export interface Spy {
  (...args: unknown[]): void;
  calls: unknown[][];
  callCount: () => number;
  calledWith: (...expected: unknown[]) => boolean;
  lastCall: () => unknown[] | undefined;
  reset: () => void;
}

export function createSpy(): Spy {
  const calls: unknown[][] = [];
  const spy = (...args: unknown[]): void => {
    calls.push(args);
  };
  spy.calls = calls;
  spy.callCount = () => calls.length;
  spy.calledWith = (...expected: unknown[]): boolean => {
    return calls.some((call) => JSON.stringify(call) === JSON.stringify(expected));
  };
  spy.lastCall = (): unknown[] | undefined =>
    calls.length > 0 ? calls[calls.length - 1] : undefined;
  spy.reset = (): void => {
    calls.length = 0;
  };
  return spy;
}
