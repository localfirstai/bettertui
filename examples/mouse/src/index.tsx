import { CommandBuffer, createReconciler } from "@bettertui/core";
import {
  Badge,
  Button,
  Checkbox,
  Flex,
  Heading,
  Provider,
  Radio,
  Separator,
  Slider,
  Spacer,
  StatusLine,
  Switch,
  Text,
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface MouseState {
  lastClick: string;
  checkboxOn: boolean;
  switchOn: boolean;
  sliderValue: number;
  radioValue: string;
  clickCount: number;
}

function MouseDemo({ state }: { state: MouseState }) {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Mouse & Interactive Demo</Heading>
          <Spacer />
          <Badge variant="info">{state.clickCount} clicks</Badge>
        </Flex>

        <Separator />

        <Heading level={3}>Buttons</Heading>
        <Flex flexDirection="row" gap={1}>
          <Button
            onPress={() => {
              state.clickCount++;
              state.lastClick = "Button A";
            }}
          >
            Button A
          </Button>
          <Button
            variant="secondary"
            onPress={() => {
              state.clickCount++;
              state.lastClick = "Button B";
            }}
          >
            Button B
          </Button>
          <Button
            variant="danger"
            onPress={() => {
              state.clickCount++;
              state.lastClick = "Danger";
            }}
          >
            Danger
          </Button>
        </Flex>

        <Separator />

        <Heading level={3}>Toggles</Heading>
        <Flex flexDirection="column" gap={0}>
          <Flex flexDirection="row" gap={1}>
            <Checkbox
              checked={state.checkboxOn}
              onChange={() => {
                state.checkboxOn = !state.checkboxOn;
              }}
            />
            <Text>Checkbox: {state.checkboxOn ? "ON" : "OFF"}</Text>
          </Flex>

          <Flex flexDirection="row" gap={1}>
            <Switch
              checked={state.switchOn}
              onChange={() => {
                state.switchOn = !state.switchOn;
              }}
            />
            <Text>Switch: {state.switchOn ? "ON" : "OFF"}</Text>
          </Flex>
        </Flex>

        <Separator />

        <Heading level={3}>Slider</Heading>
        <Flex flexDirection="column" gap={0}>
          <Slider
            value={state.sliderValue}
            min={0}
            max={100}
            onChange={(v) => {
              state.sliderValue = v;
            }}
          />
          <Text>Value: {state.sliderValue}</Text>
        </Flex>

        <Separator />

        <Heading level={3}>Radio Group</Heading>
        <Flex flexDirection="column" gap={0}>
          {["red", "green", "blue"].map((color) => (
            <Flex key={color} flexDirection="row" gap={1}>
              <Radio
                checked={state.radioValue === color}
                onChange={() => {
                  state.radioValue = color;
                }}
              />
              <Text>{color}</Text>
            </Flex>
          ))}
        </Flex>

        <Separator />

        <StatusLine
          items={[
            { label: "Last", value: state.lastClick || "none" },
            { label: "Theme", value: state.radioValue },
            { label: "Slider", value: `${state.sliderValue}` },
            { label: "Controls", value: "Tab to move focus, q=quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

const state: MouseState = {
  lastClick: "",
  checkboxOn: false,
  switchOn: false,
  sliderValue: 50,
  radioValue: "green",
  clickCount: 0,
};

function renderApp() {
  const element = <MouseDemo state={state} />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Mouse & Interactive Demo");
console.log("Tab to move focus, Enter/Space to activate, q to quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "q") {
    process.exit(0);
  }
});
