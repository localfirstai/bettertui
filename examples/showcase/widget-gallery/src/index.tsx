import { CommandBuffer, createReconciler } from "@bettertui/core";
import {
  Accordion,
  Badge,
  Blockquote,
  Box,
  Button,
  Calendar,
  Chart,
  Checkbox,
  Code,
  Combobox,
  ContextMenu,
  DataTable,
  Dropdown,
  Flex,
  Grid,
  Heading,
  Input,
  Label,
  List,
  Modal,
  Pane,
  Popover,
  Progress,
  Provider,
  Radio,
  Select,
  Separator,
  Slider,
  Spacer,
  Spinner,
  Stack,
  StatusLine,
  Switch,
  Tabs,
  Text,
  Textarea,
  Toast,
  Tooltip,
  Tree,
  Viewport,
} from "@bettertui/react";
import type { ReactNode } from "react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

interface GalleryState {
  activeTab: number;
  checkboxOn: boolean;
  switchOn: boolean;
  sliderValue: number;
  radioValue: string;
  lastInteraction: string;
}

interface Category {
  name: string;
  letter: string;
  widgetCount: number;
  render: (state: GalleryState) => ReactNode;
}

const categories: Category[] = [
  {
    name: "Layout",
    letter: "A",
    widgetCount: 6,
    render: () => (
      <Flex flexDirection="column" gap={1}>
        <Box padding={1}>
          <Text bold>Box</Text> — padding and border
        </Box>
        <Flex flexDirection="row" gap={1}>
          <Box padding={1}>
            <Text>Row A</Text>
          </Box>
          <Box padding={1}>
            <Text>Row B</Text>
          </Box>
        </Flex>
        <Text dim>Flex — row and column</Text>
        <Grid columns={3} gap={1}>
          <Box padding={1}>
            <Text>Cell 1</Text>
          </Box>
          <Box padding={1}>
            <Text>Cell 2</Text>
          </Box>
          <Box padding={1}>
            <Text>Cell 3</Text>
          </Box>
        </Grid>
        <Text dim>Grid — 3 columns</Text>
        <Stack gap={0}>
          <Text>Stacked 1</Text>
          <Text>Stacked 2</Text>
          <Text>Stacked 3</Text>
        </Stack>
        <Text dim>Stack — stacked items</Text>
        <Flex flexDirection="row" gap={0}>
          <Text>Left</Text>
          <Spacer />
          <Text>Right</Text>
        </Flex>
        <Text dim>Spacer — pushing elements apart</Text>
        <Separator />
        <Text dim>Separator — horizontal line</Text>
      </Flex>
    ),
  },
  {
    name: "Typography",
    letter: "B",
    widgetCount: 5,
    render: () => (
      <Flex flexDirection="column" gap={1}>
        <Text>Text — normal</Text>
        <Text bold>Text — bold</Text>
        <Text dim>Text — dim</Text>
        <Text italic>Text — italic</Text>
        <Text underline>Text — underline</Text>
        <Text strikethrough>Text — strikethrough</Text>
        <Text color="green">Text — colored green</Text>
        <Heading level={1}>Heading level 1</Heading>
        <Heading level={2}>Heading level 2</Heading>
        <Heading level={3}>Heading level 3</Heading>
        <Label htmlFor="demo">Label — form label</Label>
        <Code inline>const x = 42;</Code>
        <Blockquote>A blockquote for quoted text.</Blockquote>
      </Flex>
    ),
  },
  {
    name: "Interactive",
    letter: "C",
    widgetCount: 10,
    render: (state) => (
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" gap={1}>
          <Button variant="default">Default</Button>
          <Button variant="primary">Primary</Button>
          <Button variant="secondary">Secondary</Button>
          <Button variant="danger">Danger</Button>
        </Flex>
        <Text dim>Button — 4 variants</Text>
        <Input placeholder="Type here..." />
        <Text dim>Input — text input with placeholder</Text>
        <Textarea placeholder="Multi-line input..." rows={3} />
        <Text dim>Textarea — multi-line input</Text>
        <Checkbox checked={state.checkboxOn} label="Toggle option" />
        <Text dim>Checkbox — {state.checkboxOn ? "ON" : "OFF"}</Text>
        <Switch checked={state.switchOn} label="Dark mode" />
        <Text dim>Switch — {state.switchOn ? "ON" : "OFF"}</Text>
        <Slider value={state.sliderValue} min={0} max={100} />
        <Text dim>Slider — value: {state.sliderValue}</Text>
        <Flex flexDirection="row" gap={1}>
          <Radio name="color" value="red" checked={state.radioValue === "red"} label="Red" />
          <Radio name="color" value="green" checked={state.radioValue === "green"} label="Green" />
          <Radio name="color" value="blue" checked={state.radioValue === "blue"} label="Blue" />
        </Flex>
        <Text dim>Radio — selected: {state.radioValue}</Text>
        <Select value="option1">
          <option value="option1">Option 1</option>
          <option value="option2">Option 2</option>
          <option value="option3">Option 3</option>
        </Select>
        <Text dim>Select — dropdown concept</Text>
        <Combobox
          value=""
          placeholder="Search..."
          options={[
            { label: "React", value: "react" },
            { label: "Vue", value: "vue" },
            { label: "Svelte", value: "svelte" },
          ]}
        />
        <Text dim>Combobox — searchable dropdown</Text>
        <Tabs tabs={[{ label: "Tab 1" }, { label: "Tab 2" }, { label: "Tab 3" }]} activeIndex={0} />
        <Text dim>Tabs — tab navigation</Text>
      </Flex>
    ),
  },
  {
    name: "Navigation",
    letter: "D",
    widgetCount: 2,
    render: () => (
      <Flex flexDirection="column" gap={1}>
        <Tabs
          tabs={[{ label: "Overview" }, { label: "Settings" }, { label: "About" }]}
          activeIndex={1}
        />
        <Text dim>Tabs — tabbed interface</Text>
        <Accordion title="Click to expand" expanded={false}>
          <Text>This is hidden content revealed by the accordion.</Text>
        </Accordion>
        <Text dim>Accordion — expandable section</Text>
      </Flex>
    ),
  },
  {
    name: "Feedback",
    letter: "E",
    widgetCount: 3,
    render: () => (
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" gap={1}>
          <Badge>default</Badge>
          <Badge variant="primary">primary</Badge>
          <Badge variant="success">success</Badge>
          <Badge variant="warning">warning</Badge>
          <Badge variant="danger">danger</Badge>
          <Badge variant="info">info</Badge>
        </Flex>
        <Text dim>Badge — 6 variants</Text>
        <Progress value={64} />
        <Text dim>Progress — 64%</Text>
        <Flex flexDirection="row" gap={1}>
          <Spinner type="dots" label="Loading" />
          <Spinner type="line" />
          <Spinner type="braille" />
          <Spinner type="arc" />
        </Flex>
        <Text dim>Spinner — 4 types</Text>
      </Flex>
    ),
  },
  {
    name: "Data Display",
    letter: "F",
    widgetCount: 4,
    render: () => (
      <Flex flexDirection="column" gap={1}>
        <List
          items={[
            { id: "1", label: "First item" },
            { id: "2", label: "Second item" },
            { id: "3", label: "Third item" },
          ]}
          selectedId="2"
        />
        <Text dim>List — 3 items, second selected</Text>
        <Tree
          nodes={[
            {
              id: "root",
              label: "src",
              expanded: true,
              children: [
                { id: "components", label: "components/" },
                { id: "hooks", label: "hooks/" },
                { id: "index.ts", label: "index.ts" },
              ],
            },
          ]}
          selectedId="components"
        />
        <Text dim>Tree — nested structure</Text>
        <Table
          columns={[
            { key: "name", header: "Name", width: 20 },
            { key: "version", header: "Version", width: 10 },
          ]}
          data={[
            { name: "react", version: "19.0.0" },
            { name: "typescript", version: "5.7.0" },
          ]}
        />
        <Text dim>Table — basic table</Text>
        <DataTable
          columns={[
            { key: "pkg", header: "Package", width: 20 },
            { key: "size", header: "Size", width: 10 },
            { key: "deps", header: "Deps", width: 8 },
          ]}
          data={[
            { pkg: "@bettertui/core", size: "12 KB", deps: 0 },
            { pkg: "@bettertui/react", size: "8 KB", deps: 2 },
            { pkg: "@bettertui/native", size: "5 KB", deps: 1 },
          ]}
          sortable
          filterable
        />
        <Text dim>DataTable — sortable and filterable</Text>
      </Flex>
    ),
  },
  {
    name: "Overlay",
    letter: "G",
    widgetCount: 5,
    render: () => (
      <Flex flexDirection="column" gap={1}>
        <Tooltip content="This is a tooltip" position="top">
          <Button variant="default">Hover me</Button>
        </Tooltip>
        <Text dim>Tooltip — top, bottom, left, right</Text>
        <Modal title="Confirm Action" closable>
          <Text>Are you sure you want to proceed?</Text>
        </Modal>
        <Text dim>Modal — dialog concept</Text>
        <Popover content={<Text>Popover content</Text>} position="bottom">
          <Button variant="secondary">Open Popover</Button>
        </Popover>
        <Text dim>Popover — positioned content</Text>
        <Dropdown
          items={[
            { label: "Edit", value: "edit" },
            { label: "Duplicate", value: "dup" },
            { label: "Delete", value: "del", disabled: true },
          ]}
        />
        <Text dim>Dropdown — menu with disabled item</Text>
        <ContextMenu
          items={[
            { label: "Copy", value: "copy" },
            { label: "Paste", value: "paste" },
            { separator: true, label: "", value: "" },
            { label: "Select All", value: "select-all" },
          ]}
        />
        <Text dim>ContextMenu — right-click menu</Text>
      </Flex>
    ),
  },
  {
    name: "Status",
    letter: "H",
    widgetCount: 2,
    render: () => (
      <Flex flexDirection="column" gap={1}>
        <Toast message="Operation completed successfully" variant="success" />
        <Text dim>Toast — success notification</Text>
        <StatusLine
          items={[
            { label: "Status", value: "Connected" },
            { label: "Ping", value: "12ms" },
            { label: "Errors", value: "0" },
          ]}
        />
        <Text dim>StatusLine — status bar with items</Text>
      </Flex>
    ),
  },
  {
    name: "Container",
    letter: "I",
    widgetCount: 4,
    render: () => (
      <Flex flexDirection="column" gap={1}>
        <Pane title="Bordered Pane" border>
          <Text>Content inside a bordered pane.</Text>
        </Pane>
        <Text dim>Pane — bordered container with title</Text>
        <Viewport width={60} height={5} scrollY={0}>
          <Text>Scrollable viewport content.</Text>
        </Viewport>
        <Text dim>Viewport — scrollable area</Text>
        <Calendar />
        <Text dim>Calendar — date picker</Text>
        <Chart
          type="bar"
          data={[
            { label: "Mon", value: 30 },
            { label: "Tue", value: 45 },
            { label: "Wed", value: 25 },
            { label: "Thu", value: 60 },
            { label: "Fri", value: 50 },
          ]}
        />
        <Text dim>Chart — bar chart</Text>
      </Flex>
    ),
  },
];

function WidgetGallery(state: GalleryState) {
  const category = categories[state.activeTab];
  if (!category) return null;
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Widget Gallery</Heading>
          <Spacer />
          <Badge variant="primary">{categories.length} categories</Badge>
        </Flex>

        <Separator />

        <Flex flexDirection="row" gap={1}>
          {categories.map((cat, i) => (
            <Badge key={cat.letter} variant={i === state.activeTab ? "primary" : "default"}>
              {cat.letter}. {cat.name} ({cat.widgetCount})
            </Badge>
          ))}
        </Flex>

        <Separator />

        <Flex flexDirection="row" alignItems="center">
          <Heading level={3}>
            {category.letter}. {category.name}
          </Heading>
          <Spacer />
          <Badge>{category.widgetCount} widgets</Badge>
        </Flex>

        <Box padding={1}>{category.render(state)}</Box>

        <Separator />

        <StatusLine
          items={[
            { label: "Gallery", value: "Tab/Shift+Tab" },
            { label: "Category", value: `${state.activeTab + 1}/${categories.length}` },
            { label: "Widgets", value: `${category.widgetCount}` },
            { label: "Quit", value: "q" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

const state: GalleryState = {
  activeTab: 0,
  checkboxOn: false,
  switchOn: false,
  sliderValue: 50,
  radioValue: "green",
  lastInteraction: "none",
};

function renderApp() {
  const element = <WidgetGallery {...state} />;
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Widget Gallery");
console.log("Tab/Shift+Tab: switch category, q: quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();

  if (key === "q") {
    process.exit(0);
  } else if (key === "\t") {
    state.activeTab = (state.activeTab + 1) % categories.length;
    state.lastInteraction = `Tab → ${categories[state.activeTab]?.name ?? ""}`;
    renderApp();
  } else if (key === "\u001b[Z") {
    state.activeTab = (state.activeTab - 1 + categories.length) % categories.length;
    state.lastInteraction = `Shift+Tab → ${categories[state.activeTab]?.name ?? ""}`;
    renderApp();
  } else if (key === "c") {
    state.checkboxOn = !state.checkboxOn;
    state.lastInteraction = `Checkbox → ${state.checkboxOn}`;
    renderApp();
  } else if (key === "s") {
    state.switchOn = !state.switchOn;
    state.lastInteraction = `Switch → ${state.switchOn}`;
    renderApp();
  } else if (key === "+" || key === "=") {
    state.sliderValue = Math.min(100, state.sliderValue + 10);
    state.lastInteraction = `Slider → ${state.sliderValue}`;
    renderApp();
  } else if (key === "-") {
    state.sliderValue = Math.max(0, state.sliderValue - 10);
    state.lastInteraction = `Slider → ${state.sliderValue}`;
    renderApp();
  } else if (key === "1") {
    state.radioValue = "red";
    state.lastInteraction = "Radio → red";
    renderApp();
  } else if (key === "2") {
    state.radioValue = "green";
    state.lastInteraction = "Radio → green";
    renderApp();
  } else if (key === "3") {
    state.radioValue = "blue";
    state.lastInteraction = "Radio → blue";
    renderApp();
  }
});
