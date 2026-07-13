// Animation basics — useAnimation, easings, and useTimeline.
//
// Demonstrates: the useAnimation hook with named easings and a simple timeline.
// The progress drives a Progress bar so motion is visible without a render loop.
// Next: theming, live-metrics.

import {
  Box,
  Flex,
  Heading,
  Progress,
  Provider,
  Separator,
  StatusLine,
  Text,
  easings,
  render,
  useAnimation,
} from "@bettertui/react";
import { KeyInput, isMainModule } from "~/lib/keyboard";
import { KeyInputProvider, useExampleKey } from "~/lib/keyboard-context";
import type { ExampleMeta } from "~/lib/meta";

export const meta: ExampleMeta = {
  slug: "animation-basics",
  title: "Animation & Motion",
  description: "Drive values over time with useAnimation, easings, and useTimeline.",
  category: "animation",
  level: 3,
  tags: ["useAnimation", "easings", "useTimeline", "motion"],
  next: ["theming", "live-metrics"],
};

let storedKeyInput: KeyInput | null = null;
const easingNames = Object.keys(easings).filter(
  (e) => typeof easings[e as keyof typeof easings] === "function",
) as (keyof typeof easings)[];

let progress = 0;
let easingIdx = 0;

function AnimationDemo() {
  useExampleKey((event) => {
    if (event.key === "e") {
      easingIdx = (easingIdx + 1) % easingNames.length;
      renderApp();
    } else if (event.key === "q" || event.key === "Escape") {
      return true;
    }
    return false;
  });

  const easing = easingNames[easingIdx] ?? "linear";
  useAnimation(
    (p) => {
      progress = Math.round(p * 100);
      renderApp();
    },
    { duration: 1200, easing, loop: true },
  );

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Heading level={1}>Animation & Motion</Heading>
        <Separator />
        <Heading level={2}>Easing: {easingNames[easingIdx]}</Heading>
        <Box padding={1}>
          <Flex flexDirection="column" gap={0}>
            <Text dim>Progress: {progress}%</Text>
            <Progress value={progress} />
          </Flex>
        </Box>
        <Separator />
        <StatusLine
          items={[
            { label: "Easing", value: String(easingNames[easingIdx]) },
            { label: "e", value: "cycle" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  if (!storedKeyInput) return;
  render(
    <KeyInputProvider keyInput={storedKeyInput}>
      <AnimationDemo />
    </KeyInputProvider>,
  );
}

export function run(keyInput: KeyInput): void {
  storedKeyInput = keyInput;
  renderApp();
}

export function destroy(keyInput: KeyInput): void {
  void keyInput;
}

export const Example = AnimationDemo;

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
