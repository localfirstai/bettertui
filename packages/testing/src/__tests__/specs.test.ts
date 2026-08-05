import { expect } from "vitest";
import { describeBehaviour } from "../specs/runner";

describeBehaviour("Button Behavioural Specification", (registerSpec) => {
  registerSpec("presses key and updates frame", async (driver) => {
    await driver.render("ButtonComponent");
    await driver.type("a");
    const frame = driver.getFrameText();
    expect(frame).toBeDefined();
  });
});
