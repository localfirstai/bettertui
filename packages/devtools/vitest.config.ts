import { defineProject, mergeConfig } from "vitest/config";
import configShared from "../../vitest.shared";

export default mergeConfig(
  configShared,
  defineProject({
    test: {
      name: "devtools",
      coverage: {
        exclude: ["**/types.ts", "**/__tests__/**"],
      },
    },
  }),
);
