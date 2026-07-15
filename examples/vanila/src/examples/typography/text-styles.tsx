// Text styles — bold, italic, dim, underline, and colour treatment.
//
// Demonstrates: Text modifiers for legible, expressive typography. Keep contrast
// high so content reads on both dark and light themes.
// Next: flex-layout, theming.

import { Box, Flex, Heading, Provider, Separator, Text } from "@bettertui/react";
import { KeyInput, isMainModule } from "~/lib/keyboard";
import { useExampleKey } from "~/lib/keyboard-context";
import type { ExampleMeta } from "~/lib/meta";
import { mountExample } from "~/lib/standalone";

export const meta: ExampleMeta = {
  slug: "text-styles",
  title: "Text Styles",
  description: "Bold, italic, dim, underline, and colour treatment for legible typography.",
  category: "typography",
  level: 1,
  tags: ["Text", "bold", "dim", "color", "underline"],
  next: ["flex-layout", "theming"],
};

function TextStyles() {
  useExampleKey((event) => {
    if (event.key === "q" || event.key === "Escape") return true;
    return false;
  });

  return (
    <Provider>
      <Flex flexDirection="column" gap={1} padding={1}>
        <Heading level={1}>Text Styles</Heading>
        <Separator />
        <Box padding={1}>
          <Flex flexDirection="column" gap={0}>
            <Text bold>Bold text</Text>
            <Text italic>Italic text</Text>
            <Text dim>Dimmed / muted text</Text>
            <Text underline>Underlined text</Text>
            <Text bold color="#50c8a0">
              Accent-coloured bold text
            </Text>
            <Text color="#dc5050">Error-coloured text</Text>
            <Text style={{ inverse: true }}>Inverse (highlight) text</Text>
          </Flex>
        </Box>
        <Separator />
        <Text dim>Press q to quit</Text>
      </Flex>
    </Provider>
  );
}

export function run(keyInput: KeyInput): void {
  mountExample(TextStyles, keyInput);
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = TextStyles;

if (isMainModule()) {
  const ki = new KeyInput();
  ki.start();
  ki.on((event) => {
    if ((event.key === "q" || event.key === "Escape") && !event.ctrl) {
      destroy(ki);
      ki.stop();
      process.exit(0);
    }
  });
  run(ki);
}
