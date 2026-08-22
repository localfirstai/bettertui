import {
  Box,
  type CliRenderer,
  type TabOption,
  TabSelect,
  TabSelectEvents,
  Text,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let rootContainer: Box | null = null;
let tabBar: TabSelect | null = null;
let contentText: Text | null = null;
let renderer: CliRenderer | null = null;

const TAB_DEFS: TabOption[] = [
  { name: "Overview", description: "Project summary and quick-stats", value: "overview" },
  { name: "Files", description: "Browse the project file tree", value: "files" },
  { name: "Terminal", description: "Integrated terminal emulator", value: "terminal" },
  { name: "Extensions", description: "Installed and available extensions", value: "extensions" },
  { name: "Settings", description: "Workspace and editor preferences", value: "settings" },
];

const TAB_CONTENT: Record<string, string> = {
  overview:
    "Project: BetterTUI\nVersion: 0.1.0\nLicense: MIT\n\nA high-performance terminal UI framework.",
  files:
    "src/\n  index.ts\n  components/\n    Box.ts\n    Text.ts\n    Input.ts\npackage.json\ntsconfig.json",
  terminal:
    "$ pnpm build\n✓ core compiled in 1.2s\n✓ examples compiled in 0.8s\n\nBuild succeeded.",
  extensions:
    "Installed:\n  • Biome (lint + format)\n  • Vitest (test runner)\n  • TypeScript LSP\n\nAvailable: 42 extensions",
  settings:
    "Theme: Tokyo Night\nFont: JetBrains Mono 13px\nTabSize: 2\nFormatOnSave: true\nLineNumbers: relative",
};

function showTabContent(value: unknown): void {
  if (!contentText) return;
  const key = String(value);
  contentText.content = TAB_CONTENT[key] ?? "";
}

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  rootContainer = new Box(renderer, {
    id: "tabselect-example-root",
    flexDirection: "column",
    width: "100%",
    height: "100%",
  });
  renderer.root.add(rootContainer);

  // Header
  const header = new Box(renderer, {
    height: 3,
    flexShrink: 0,
    backgroundColor: "#24283b",
    border: ["bottom"],
    borderStyle: "single",
    borderColor: "#414868",
    alignItems: "center",
    paddingX: 2,
  });
  header.add(
    new Text(renderer, { content: t`${bold(fg("#7aa2f7")("TabSelect Example"))}`, fg: "#c0caf5" }),
  );
  rootContainer.add(header);

  // Tab bar
  tabBar = new TabSelect(renderer, {
    id: "tabbar",
    options: TAB_DEFS,
    selectedIndex: 0,
    width: "100%",
    height: 3,
    flexShrink: 0,
    showUnderline: true,
    showDescription: true,
    showScrollArrows: true,
    wrapSelection: false,
    tabPadding: 2,
    tabGap: 0,
    backgroundColor: "#24283b",
    textColor: "#565f89",
    selectedTextColor: "#7aa2f7",
    activeUnderlineColor: "#7aa2f7",
    inactiveUnderlineColor: "#414868",
    descriptionColor: "#414868",
  });
  rootContainer.add(tabBar);

  // Content area
  const contentArea = new Box(renderer, {
    flexGrow: 1,
    border: true,
    borderStyle: "single",
    borderColor: "#414868",
    marginX: 2,
    marginY: 1,
    padding: 1,
    flexDirection: "column",
  });

  contentText = new Text(renderer, {
    flexGrow: 1,
    wrapMode: "word",
    fg: "#c0caf5",
  });
  contentArea.add(contentText);
  rootContainer.add(contentArea);

  // Footer hint
  const footer = new Text(renderer, {
    content: t`${fg("#414868")("← → navigate tabs  ·  Enter confirm  ·  Ctrl+C quit")}`,
    height: 1,
    flexShrink: 0,
    textAlign: "center",
    marginBottom: 1,
    fg: "#414868",
  });
  rootContainer.add(footer);

  // Wire events
  tabBar.on(TabSelectEvents.SELECTION_CHANGED, (_idx, option) => {
    showTabContent((option as TabOption).value);
  });

  tabBar.on(TabSelectEvents.ITEM_SELECTED, (_idx, option) => {
    showTabContent((option as TabOption).value);
  });

  showTabContent("overview");
  tabBar.focus();
}

export function destroy(rendererInstance: CliRenderer): void {
  if (rootContainer) {
    rendererInstance.root.remove(rootContainer);
    rootContainer.destroy();
    rootContainer = null;
  }
  tabBar = null;
  contentText = null;
  renderer = null;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
