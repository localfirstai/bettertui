import { CommandBuffer, createReconciler } from "@bettertui/core";
import {
  Badge,
  Box,
  Checkbox,
  Flex,
  Heading,
  Input,
  Provider,
  Radio,
  Separator,
  Slider,
  Spacer,
  StatusLine,
  Switch,
  Text,
  Textarea,
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface FormState {
  inputValue: string;
  textareaValue: string;
  checkboxOn: boolean;
  switchOn: boolean;
  sliderValue: number;
  radioValue: string;
}

const state: FormState = {
  inputValue: "Hello",
  textareaValue: "Line 1\nLine 2\nLine 3",
  checkboxOn: false,
  switchOn: false,
  sliderValue: 50,
  radioValue: "option-a",
};

function FormDemo() {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Form Widgets</Heading>
          <Spacer />
          <Badge variant="info">Interactive</Badge>
        </Flex>

        <Separator />

        <Heading level={3}>Text Input</Heading>
        <Box padding={1}>
          <Flex flexDirection="column" gap={0}>
            <Text dimColor>Type something:</Text>
            <Input value={state.inputValue} placeholder="Enter text..." width={40} />
          </Flex>
        </Box>

        <Heading level={3}>Textarea</Heading>
        <Box padding={1}>
          <Flex flexDirection="column" gap={0}>
            <Text dimColor>Multi-line content:</Text>
            <Textarea
              value={state.textareaValue}
              placeholder="Type here..."
              width={50}
              height={4}
            />
          </Flex>
        </Box>

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
          {(["option-a", "option-b", "option-c"] as const).map((opt) => (
            <Flex key={opt} flexDirection="row" gap={1}>
              <Radio
                checked={state.radioValue === opt}
                onChange={() => {
                  state.radioValue = opt;
                }}
              />
              <Text>{opt}</Text>
            </Flex>
          ))}
        </Flex>

        <Separator />

        <StatusLine
          items={[
            { label: "Input", value: state.inputValue || "(empty)" },
            { label: "Slider", value: `${state.sliderValue}` },
            { label: "Radio", value: state.radioValue },
            {
              label: "Controls",
              value: "i=edit t=textarea c=check s=switch +/-=slider 123=radio q=quit",
            },
          ]}
        />
      </Flex>
    </Provider>
  );
}

function renderApp() {
  const element = <FormDemo />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Forms Demo");
console.log("Navigate with keyboard shortcuts, q to quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "i") {
    state.inputValue = state.inputValue === "Hello" ? "World" : "Hello";
    renderApp();
  } else if (key === "t") {
    state.textareaValue = state.textareaValue === "" ? "Line 1\nLine 2\nLine 3" : "";
    renderApp();
  } else if (key === "c") {
    state.checkboxOn = !state.checkboxOn;
    renderApp();
  } else if (key === "s") {
    state.switchOn = !state.switchOn;
    renderApp();
  } else if (key === "+") {
    state.sliderValue = Math.min(state.sliderValue + 5, 100);
    renderApp();
  } else if (key === "-") {
    state.sliderValue = Math.max(state.sliderValue - 5, 0);
    renderApp();
  } else if (key === "1") {
    state.radioValue = "option-a";
    renderApp();
  } else if (key === "2") {
    state.radioValue = "option-b";
    renderApp();
  } else if (key === "3") {
    state.radioValue = "option-c";
    renderApp();
  } else if (key === "q") {
    process.exit(0);
  }
});
