import {
  Box,
  type CliRenderer,
  Select,
  SelectEvents,
  type SelectOption,
  Text,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let rootContainer: Box | null = null;
let selectEl: Select | null = null;
let detailsText: Text | null = null;
let renderer: CliRenderer | null = null;

const menuOptions: SelectOption[] = [
  { name: "New Project", description: "Create a new project from a template", value: "new" },
  { name: "Open Project", description: "Open an existing project from disk", value: "open" },
  { name: "Clone Repo", description: "Clone a remote Git repository", value: "clone" },
  { name: "Recent Files", description: "Quickly jump to recently opened files", value: "recent" },
  { name: "Settings", description: "Edit editor and workspace preferences", value: "settings" },
  { name: "Extensions", description: "Browse and install extensions", value: "ext" },
  {
    name: "Keyboard Shortcuts",
    description: "View and customise keyboard bindings",
    value: "keys",
  },
  { name: "Command Palette", description: "Run any command by name", value: "cmd" },
  { name: "About", description: "Version info and release notes", value: "about" },
  { name: "Quit", description: "Exit the application", value: "quit" },
];

function updateDetails(option: SelectOption | null): void {
  if (!detailsText) return;
  if (!option) {
    detailsText.content = "";
    return;
  }
  detailsText.content = t`${bold(fg("#7aa2f7")(option.name as string))}
${fg("#a9b1d6")(option.description as string)}
${fg("#414868")(`value: ${String(option.value)}`)}`;
}

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  rootContainer = new Box(renderer, {
    id: "select-example-root",
    flexDirection: "column",
    width: "100%",
    height: "100%",
    padding: 1,
    gap: 1,
  });
  renderer.root.add(rootContainer);

  // Header
  rootContainer.add(
    new Text(renderer, {
      content: t`${bold(fg("#7aa2f7")("Select Component Example"))}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  // Main layout: list + detail pane side-by-side
  const body = new Box(renderer, {
    flexDirection: "row",
    gap: 2,
    flexGrow: 1,
    flexShrink: 1,
    flexBasis: 0,
    minHeight: 0,
  });

  // List panel
  const listPanel = new Box(renderer, {
    width: 35,
    flexShrink: 0,
    border: true,
    borderStyle: "round",
    borderColor: "#414868",
    focusedBorderColor: "#7aa2f7",
    title: " Menu ",
    titleAlignment: "center",
    flexGrow: 0,
    overflow: "hidden",
  });

  selectEl = new Select(renderer, {
    id: "select-example",
    options: menuOptions,
    selectedIndex: 0,
    width: "100%",
    showDescription: false,
    showScrollIndicator: true,
    wrapSelection: false,
    fastScrollStep: 3,
    textColor: "#a9b1d6",
    selectedBackgroundColor: "#2a2e43",
    selectedTextColor: "#c0caf5",
    focusedTextColor: "#c0caf5",
    backgroundColor: "transparent",
    focusedBackgroundColor: "transparent",
  });

  listPanel.add(selectEl);
  body.add(listPanel);

  // Detail pane
  const detailPanel = new Box(renderer, {
    flexGrow: 1,
    border: true,
    borderStyle: "round",
    borderColor: "#414868",
    title: " Details ",
    titleAlignment: "center",
    padding: 1,
    flexDirection: "column",
    gap: 1,
  });

  detailsText = new Text(renderer, {
    flexGrow: 1,
    wrapMode: "word",
    fg: "#c0caf5",
  });
  detailPanel.add(detailsText);

  const hint = new Text(renderer, {
    content: t`${fg("#414868")("↑↓ navigate  ·  Enter confirm  ·  Esc return")}`,
    height: 1,
    flexShrink: 0,
  });
  detailPanel.add(hint);

  body.add(detailPanel);
  rootContainer.add(body);

  // Wire events
  selectEl.on(SelectEvents.SELECTION_CHANGED, (_index, option) => {
    updateDetails(option as SelectOption);
  });

  selectEl.on(SelectEvents.ITEM_SELECTED, (_index, option) => {
    const opt = option as SelectOption;
    if (opt.value === "quit") process.exit(0);
    updateDetails(opt);
  });

  updateDetails(menuOptions[0] ?? null);
  selectEl.focus();
}

export function destroy(rendererInstance: CliRenderer): void {
  if (rootContainer) {
    rendererInstance.root.remove(rootContainer);
    rootContainer.destroy();
    rootContainer = null;
  }
  selectEl = null;
  detailsText = null;
  renderer = null;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
