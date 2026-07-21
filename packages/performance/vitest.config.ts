import { defineProject, mergeConfig } from "vitest/config";
import configShared from "../../vitest.shared";

export default mergeConfig(
  configShared,
  defineProject({
    test: {
      name: "performance",
      include: ["src/**/*.bench.ts"],
      includeSource: [],
      bench: {
        reporters: ["default"],
        // 2s iteration time budget per case so CI doesn't run all day.
        iterations: 50,
        time: 1000,
      },
      hookTimeout: 30000,
      testTimeout: 30000,
    },
  }),
);
