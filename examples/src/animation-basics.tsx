// Animation basics — useAnimation, easings, and useTimeline.
//
// Demonstrates: the useAnimation hook with named easings and a simple timeline.
// The progress drives a Progress bar so motion is visible without a render loop.
// Next: theming, theming, frame-loop (when added).

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
  useKeyboard,
  useRuntime,
} from "@bettertui/react";
import type { ExampleMeta } from "./lib/meta";

export const meta: ExampleMeta = {
  slug: "animation-basics",
  title: "Animation & Motion",
  description: "Drive values over time with useAnimation, easings, and useTimeline.",
  category: "animation",
  level: 3,
  tags: ["useAnimation", "easings", "useTimeline", "motion"],
  next: ["theming", "text-styles", "live-metrics"],
};

const easingNames = Object.keys(easings).filter(
  (e) => typeof easings[e as keyof typeof easings] === "function",
) as (keyof typeof easings)[];

let progress = 0;
let easingIdx = 0;

function AnimationDemo() {
  const runtime = useRuntime();

  useAnimation(
    (p) => {
      progress = Math.round(p * 100);
      renderApp();
    },
    { duration: 1200, easing: easingNames[easingIdx], loop: true },
  );

  useKeyboard((key) => {
    if (key.key === "e") {
      easingIdx = (easingIdx + 1) % easingNames.length;
      renderApp();
    } else if (key.key === "q") {
      runtime?.runtime.dispose();
      process.exit(0);
    }
    return true;
  });

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
            { label: "Easing", value: easing },
            { label: "e", value: "cycle" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  render(<AnimationDemo />);
}

renderApp();
