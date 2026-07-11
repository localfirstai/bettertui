import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    bench: {
      reporters: ["default"],
    },
  },
});
