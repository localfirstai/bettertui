import fs from "node:fs";
import path from "node:path";
import { expect } from "vitest";
import type { TargetElement } from "../screen/screen";
import type { CellAttributes } from "../terminal/cell-matrix";
import type { TestTerminal } from "../terminal/test-terminal";

export function setupMatchers(): void {
  expect.extend({
    toMatchGoldenFrame(receivedTerminal: TestTerminal, goldenName: string) {
      const frame = receivedTerminal.captureFrame();
      const goldensDir = path.join(process.cwd(), "__goldens__");
      const goldenPath = path.join(goldensDir, `${goldenName}.ansi`);

      if (!fs.existsSync(goldenPath)) {
        if (process.env.UPDATE_GOLDENS) {
          fs.mkdirSync(goldensDir, { recursive: true });
          fs.writeFileSync(goldenPath, frame.textFrame, "utf-8");
          return { pass: true, message: () => `Created golden frame: ${goldenName}` };
        }
        return {
          pass: false,
          message: () =>
            `Golden frame file missing at ${goldenPath}. Re-run with UPDATE_GOLDENS=1 to create.`,
        };
      }

      const expectedText = fs.readFileSync(goldenPath, "utf-8");
      const pass = frame.textFrame === expectedText;

      return {
        pass,
        message: () =>
          pass
            ? `Frame matches golden ${goldenName}`
            : `Frame mismatch for golden frame "${goldenName}":\n\nExpected:\n${expectedText}\n\nReceived:\n${frame.textFrame}`,
      };
    },

    toBeFocused(target: TargetElement | TestTerminal) {
      if ("getFocusedNodeId" in target) {
        const pass = target.getFocusedNodeId() !== null;
        return {
          pass,
          message: () =>
            pass ? "Terminal has focused element" : "Expected terminal to have a focused element",
        };
      }

      // Target element assertion
      const pass = target !== null && target.text !== undefined;
      return {
        pass,
        message: () =>
          pass
            ? `Element "${target.text}" is focused`
            : `Expected element "${target.text}" to be focused`,
      };
    },

    toHaveCell(
      terminal: TestTerminal,
      expected: { x: number; y: number } & Partial<CellAttributes>,
    ) {
      const cell = terminal.getCell(expected.x, expected.y);
      if (!cell) {
        return {
          pass: false,
          message: () => `Cell at (${expected.x}, ${expected.y}) is out of bounds`,
        };
      }

      let pass = true;
      if (expected.char !== undefined && cell.char !== expected.char) pass = false;
      if (expected.bold !== undefined && cell.bold !== expected.bold) pass = false;

      return {
        pass,
        message: () =>
          pass
            ? `Cell at (${expected.x}, ${expected.y}) matches expected properties`
            : `Cell mismatch at (${expected.x}, ${expected.y}): expected ${JSON.stringify(expected)}, got ${JSON.stringify(cell)}`,
      };
    },
  });
}
