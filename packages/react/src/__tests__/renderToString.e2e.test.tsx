import { describe, expect, it } from "vitest";
import { Box, Text } from "../index";
import { renderToStringAsync } from "../testing";

describe.skip("renderToStringAsync (E2E with native engine)", () => {
  it("produces ANSI output from a component tree", async () => {
    const result = await renderToStringAsync(
      <Box flexDirection="column">
        <Text color="green">Hello Testing</Text>
      </Box>,
      { width: 40, height: 10 },
    );

    expect(result.length).toBeGreaterThan(0);
    const escapeChar = String.fromCharCode(27);
    expect(result).toContain(`${escapeChar}[`);
  });
});
