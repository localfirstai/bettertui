// Theming — applying and switching themes via the Provider.
//
// Demonstrates: Provider `theme` prop, switching between built-in themes, and
// the useTheme hook. Themes are defined in src/lib/theme.ts (internal to examples).
// Next: text-styles, animation-basics, responsive-layout.

import {
  Box,
  Flex,
  Heading,
  Provider,
  Separator,
  Text,
  render,
  useKeyboard,
  useRuntime,
} from "@bettertui/react";
import type { ExampleMeta } from "./lib/meta";
import { type ExampleThemeName, exampleThemes } from "./lib/theme";

export const meta: ExampleMeta = {
  slug: "theming",
  title: "Theming",
  description: "Apply and switch themes through the Provider's theme prop.",
  category: "theming",
  level: 2,
  tags: ["Provider", "Theme", "useTheme"],
  next: ["text-styles", "animation-basics", "responsive-layout"],
};

const themeNames = Object.keys(exampleThemes) as ExampleThemeName[];
let themeIdx = 0;

function Theming() {
  const runtime = useRuntime();
  const name = themeNames[themeIdx];

  useKeyboard((key) => {
    if (key.key === "t") {
      themeIdx = (themeIdx + 1) % themeNames.length;
      renderApp();
    } else if (key.key === "q") {
      runtime?.runtime.dispose();
      process.exit(0);
    }
    return true;
  });

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

renderApp();
