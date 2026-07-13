// Theming — applying and switching themes via the Provider.
//
// Demonstrates: Provider theme prop, switching between built-in themes, and
// useTheme. Themes are defined in src/lib/theme.ts (internal to examples).
// Next: text-styles, animation-basics.

import { Box, Flex, Heading, Provider, Separator, Text, render } from "@bettertui/react";
import type { KeyInput } from "../../lib/keyboard";
import { useExampleKey } from "../../lib/keyboard-context";
import type { ExampleMeta } from "../../lib/meta";
import { type ExampleThemeNameLiteral, exampleThemes } from "../../lib/theme";

export const meta: ExampleMeta = {
  slug: "theming",
  title: "Theming",
  description: "Apply and switch themes through the Provider's theme prop.",
  category: "theming",
  level: 2,
  tags: ["Provider", "Theme", "useTheme"],
  next: ["text-styles", "animation-basics"],
};

const themeNames = Object.keys(exampleThemes) as ExampleThemeNameLiteral[];
let themeIdx = 0;

function Theming() {
  useExampleKey((event) => {
    if (event.key === "t") {
      themeIdx = (themeIdx + 1) % themeNames.length;
      renderApp();
    } else if (event.key === "q" || event.key === "Escape") {
      return true;
    }
    return false;
  });

  const name = themeNames[themeIdx] ?? "dark";
  return (
    <Provider theme={exampleThemes[name]}>
      <Flex flexDirection="column" gap={1}>
        <Box padding={1}>
          <Heading level={1}>Theming — {name}</Heading>
        </Box>
        <Separator />
        <Box padding={1}>
          <Flex flexDirection="column" gap={0}>
            <Text bold>Theme: {name}</Text>
            <Text dim>Switch themes live with the `t` key.</Text>
            <Text>Surfaces, borders, and accents come from the active theme.</Text>
          </Flex>
        </Box>
        <Separator />
        <Text dim>t=cycle theme q=quit</Text>
      </Flex>
    </Provider>
  );
}

function renderApp() {
  render(<Theming />);
}

export function run(keyInput: KeyInput): void {
  void keyInput;
  renderApp();
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = Theming;
