import { defineConfig } from "astro/config";

// Static site deployed to performance.bettertui.com.
// Consumes benchmark JSON produced by `bun run bench`.
export default defineConfig({
  output: "static",
  site: "https://performance.bettertui.com",
  build: { outDir: "dist" },
});
