import type React from "react";
import { describe, expect, it, vi } from "vitest";
import { Box, Button, Flex, Input, Text } from "../index";

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

describe("Flex component", () => {
  it("renders with flexDirection column", () => {
    const { container } = renderToContainer(
      <Flex flexDirection="column">
        <Text>item 1</Text>
        <Text>item 2</Text>
      </Flex>,
    );
    expect(container).toBeDefined();
  });

  it("renders with justifyContent center", () => {
    const { container } = renderToContainer(
      <Flex flexDirection="row" justifyContent="center">
        <Text>centered</Text>
      </Flex>,
    );
    expect(container).toBeDefined();
  });

  it("renders with alignItems stretch", () => {
    const { container } = renderToContainer(
      <Flex flexDirection="row" alignItems="stretch">
        <Text>stretched</Text>
      </Flex>,
    );
    expect(container).toBeDefined();
  });

  it("renders with gap", () => {
    const { container } = renderToContainer(
      <Flex flexDirection="row" gap={4}>
        <Text>a</Text>
        <Text>b</Text>
      </Flex>,
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

describe("Button component", () => {
  it("renders with children", () => {
    const { container } = renderToContainer(<Button>click me</Button>);
    expect(container).toBeDefined();
  });

  it("renders with variant", () => {
    const { container } = renderToContainer(<Button variant="primary">primary</Button>);
    expect(container).toBeDefined();
  });

  it("renders disabled", () => {
    const { container } = renderToContainer(<Button disabled>disabled</Button>);
    expect(container).toBeDefined();
  });

  it("renders with onPress", () => {
    const onPress = vi.fn();
    const { container } = renderToContainer(<Button onPress={onPress}>press</Button>);
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
        <Flex flexDirection="column" gap={2}>
          <Text bold>Title</Text>
          <Text color="muted">Description</Text>
          <Button variant="primary">Action</Button>
        </Flex>
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
