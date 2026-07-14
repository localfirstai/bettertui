import { describe, expect, it } from "vitest";
import { Flex, Text } from "../index";
import { renderToStringAsync } from "../testing";

describe.skip("renderToStringAsync (E2E with native engine)", () => {
  it("produces ANSI output from a component tree", async () => {
    const result = await renderToStringAsync(
      <Flex flexDirection="column">
        <Text color="green">Hello Testing</Text>
      </Flex>,
      { width: 40, height: 10 },
    );

    expect(result.length).toBeGreaterThan(0);
    // Check for ANSI escape sequence (ESC[) - escape char followed by bracket
    const escapeChar = String.fromCharCode(27);
    expect(result).toContain(`${escapeChar}[`);
  });
});
