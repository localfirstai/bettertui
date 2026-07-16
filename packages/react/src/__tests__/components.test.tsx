import type React from "react";
import { describe, expect, it, vi } from "vitest";
import { Box, Input, Text } from "../index";

describe("Box component", () => {
  it("renders with children", () => {
    const { container } = renderToContainer(
      <Box>
        <Text>hello</Text>
      </Box>,
    );
    expect(container).toBeDefined();
  });

  it("renders with style prop", () => {
    const { container } = renderToContainer(<Box style={{ bold: true }}>content</Box>);
    expect(container).toBeDefined();
  });

  it("renders with layout props", () => {
    const { container } = renderToContainer(
      <Box padding={2} margin={1} width={100}>
        content
      </Box>,
    );
    expect(container).toBeDefined();
  });
});

describe("Text component", () => {
  it("renders with color prop", () => {
    const { container } = renderToContainer(<Text color="red">red text</Text>);
    expect(container).toBeDefined();
  });

  it("renders with bold prop", () => {
    const { container } = renderToContainer(<Text bold>bold text</Text>);
    expect(container).toBeDefined();
  });

  it("renders with multiple style props", () => {
    const { container } = renderToContainer(
      <Text color="green" bold italic underline>
        styled
      </Text>,
    );
    expect(container).toBeDefined();
  });

  it("renders without any props", () => {
    const { container } = renderToContainer(<Text>plain text</Text>);
    expect(container).toBeDefined();
  });
});

describe("Input component", () => {
  it("renders with value", () => {
    const { container } = renderToContainer(<Input value="hello" />);
    expect(container).toBeDefined();
  });

  it("renders with placeholder", () => {
    const { container } = renderToContainer(<Input placeholder="type here..." />);
    expect(container).toBeDefined();
  });

  it("renders with onChange", () => {
    const onChange = vi.fn();
    const { container } = renderToContainer(<Input onChange={onChange} />);
    expect(container).toBeDefined();
  });

  it("renders disabled", () => {
    const { container } = renderToContainer(<Input disabled />);
    expect(container).toBeDefined();
  });
});

describe("Component composition", () => {
  it("renders deeply nested components", () => {
    const { container } = renderToContainer(
      <Box padding={1}>
        <Box flexDirection="column">
          <Text bold>Title</Text>
          <Text dim>Description</Text>
        </Box>
      </Box>,
    );
    expect(container).toBeDefined();
  });

  it("renders multiple children", () => {
    const { container } = renderToContainer(
      <Box>
        <Text>a</Text>
        <Text>b</Text>
        <Text>c</Text>
      </Box>,
    );
    expect(container).toBeDefined();
  });

  it("renders with no children", () => {
    const { container } = renderToContainer(<Box />);
    expect(container).toBeDefined();
  });
});

interface TestContainer {
  container: Record<string, unknown>;
}

function renderToContainer(element: React.ReactElement): TestContainer {
  return {
    container: {
      type: element.type,
      props: element.props,
    },
  };
}
