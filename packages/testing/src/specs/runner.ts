import { describe, it } from "vitest";
import { BetterTUIDriver, type FrameworkTestingDriver } from "./drivers";

export type SpecTestFn = (driver: FrameworkTestingDriver) => Promise<void> | void;

export interface BehaviourSpec {
  name: string;
  test: SpecTestFn;
}

export function describeBehaviour(
  specName: string,
  specFn: (registerSpec: (name: string, fn: SpecTestFn) => void) => void,
): void {
  const specs: BehaviourSpec[] = [];

  specFn((name, fn) => {
    specs.push({ name, test: fn });
  });

  describe(`Behavioral Spec: ${specName}`, () => {
    for (const spec of specs) {
      it(`BetterTUI Driver - ${spec.name}`, async () => {
        const driver = new BetterTUIDriver();
        try {
          await spec.test(driver);
        } finally {
          await driver.unmount();
        }
      });
    }
  });
}
