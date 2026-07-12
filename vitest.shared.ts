import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "node",
    include: ["src/**/*.test.{ts,tsx}"],
    globals: false,
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html", "lcov", "clover"],
      include: ["src/**"],
      exclude: [
        "src/**/*.test.ts",
        "src/**/*.test.tsx",
        "src/**/*.bench.ts",
        "src/**/*.d.ts",
        "src/**/__tests__/**",
      ],
    },
    snapshotFormat: {
      indent: 2,
      escapeString: true,
    },
    typecheck: {
      enabled: false,
    },
  },
});
