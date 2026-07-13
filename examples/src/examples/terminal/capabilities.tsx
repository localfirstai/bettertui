// Terminal capabilities — detect and display the current terminal's feature set.
//
// Demonstrates: detectCapabilities() from @bettertui/core, which reads what the
// native engine discovered about the running terminal. Honest demo — it shows
// exactly what the framework can rely on, nothing faked.
// Next: theming, live-metrics.

import { detectCapabilities } from "@bettertui/core";
import { Box, Flex, Heading, Provider, Separator, StatusLine, Text } from "@bettertui/react";
import { KeyInput, isMainModule } from "~/lib/keyboard";
import { useExampleKey } from "~/lib/keyboard-context";
import type { ExampleMeta } from "~/lib/meta";
import { mountExample } from "~/lib/standalone";

export const meta: ExampleMeta = {
  slug: "capabilities",
  title: "Terminal Capabilities",
  description: "Detect and display the current terminal's feature set via the native engine.",
  category: "terminal",
  level: 2,
  tags: ["detectCapabilities", "kittyKeyboard", "trueColor", "focusEvents"],
  next: ["theming", "live-metrics"],
  requires: ["native engine"],
};

function Capabilities() {
  useExampleKey((event) => {
    if (event.key === "q" || event.key === "Escape") return true;
    return false;
  });

  let caps: Record<string, boolean> = {};
  try {
    caps = detectCapabilities() as unknown as Record<string, boolean>;
  } catch {
    caps = {};
  }

  const rows = Object.entries(caps).map(([name, supported]) => (
    <Flex key={name} flexDirection="row" gap={1}>
      <Text bold>{name}</Text>
      <Text color={supported ? "#50c878" : "#dc5050"}>
        {supported ? "supported" : "unavailable"}
      </Text>
    </Flex>
  ));

  return (
    <Provider>
      <Flex flexDirection="column" gap={1} padding={1}>
        <Heading level={1}>Terminal Capabilities</Heading>
        <Separator />
        <Box padding={1}>
          <Flex flexDirection="column" gap={0}>
            {rows.length > 0 ? (
              rows
            ) : (
              <Text dim>No capability data — native engine may be unavailable.</Text>
            )}
          </Flex>
        </Box>
        <Separator />
        <StatusLine
          items={[
            { label: "Source", value: "detectCapabilities()" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

export function run(keyInput: KeyInput): void {
  mountExample(Capabilities, keyInput);
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = Capabilities;

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
