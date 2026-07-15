import { describe, expect, it, vi } from "vitest";
import { Box, Text } from "../index";
import { renderToStringAsync } from "../testing";

vi.mock("@bettertui/core", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@bettertui/core")>();
  return {
    ...actual,
    createEngine: vi.fn(() => ({
      root: vi.fn(() => "0"),
      processCommands: vi.fn(),
      beginFrame: vi.fn(),
      renderFull: vi.fn(() => ({
        outputData: new TextEncoder().encode("MOCK_ANSI_OUTPUT"),
        width: 80,
        height: 24,
        dirtyRegionCount: 1,
      })),
      commitFrame: vi.fn(),
      resize: vi.fn(),
    })),
  };
});

describe("renderToStringAsync", () => {
  it("renders a simple component tree to a string", async () => {
    const result = await renderToStringAsync(
      <Box flexDirection="row">
        <Text>Hello Testing</Text>
      </Box>,
    );

    expect(result).toBe("MOCK_ANSI_OUTPUT");
  });
});
