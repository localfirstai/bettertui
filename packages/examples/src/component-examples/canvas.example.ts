import {
  Box,
  type CliRenderer,
  Screen,
  ScreenEvents,
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

let screen: Screen | null = null;
let renderer: CliRenderer | null = null;
let sideNav: Select | null = null;
let contentTitle: Text | null = null;
let contentBody: Text | null = null;
let footerStatus: Text | null = null;

const NAV_OPTIONS: SelectOption[] = [
  { name: "Dashboard", description: "Overview & metrics", value: "dashboard" },
  { name: "Projects", description: "Manage your projects", value: "projects" },
  { name: "Tasks", description: "Your open tasks", value: "tasks" },
  { name: "Calendar", description: "Schedule & upcoming events", value: "calendar" },
  { name: "Messages", description: "Team communications", value: "messages" },
  { name: "Reports", description: "Analytics and exports", value: "reports" },
  { name: "Settings", description: "Workspace configuration", value: "settings" },
];

const PAGE_CONTENT: Record<string, string> = {
  dashboard: "Welcome back!\n\nYou have 3 open tasks, 2 upcoming meetings,\nand 1 new message.",
  projects:
    "Active projects:\n• BetterTUI (in progress)\n• Docs Site (review)\n• CLI Tools (planning)",
  tasks: "Open tasks:\n☐ Fix layout engine test\n☐ Write component docs\n☐ Publish 0.1.0 release",
  calendar: "Today:\n10:00 Team standup\n14:30 Design review\n\nTomorrow:\n09:00 Sprint planning",
  messages: "Unread (2):\n• Alice: Great work on the layout engine!\n• Bob: Ready for code review?",
  reports: "Last 7 days:\nBuild time:     avg 1.2s\nTest coverage:  87%\nBundle size:    142 KB",
  settings: "Theme: Tokyo Night\nFont: JetBrains Mono\nTabSize: 2\nAutoSave: true",
};

function showPage(value: unknown): void {
  const key = String(value);
  if (contentTitle) {
    const page = NAV_OPTIONS.find((o) => o.value === value);
    contentTitle.content = t`${bold(fg("#7aa2f7")(page?.name ?? key))}`;
  }
  if (contentBody) {
    contentBody.content = PAGE_CONTENT[key] ?? "";
  }
}

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  screen = new Screen(renderer, {
    id: "canvas-example",
    backgroundColor: "#1a1b26",
    header: {
      height: 3,
      backgroundColor: "#24283b",
      border: ["bottom"],
      borderStyle: "single",
      borderColor: "#414868",
      title: " BetterTUI App Shell (Canvas/Screen Example) ",
      titleAlignment: "center",
      alignItems: "center",
      paddingX: 2,
    },
    body: {
      flexDirection: "row",
    },
    footer: {
      height: 1,
      backgroundColor: "#292e42",
      alignItems: "center",
      paddingX: 2,
    },
  });

  // Header: right-aligned version label
  screen.header?.add(new Text(renderer, { content: t`${fg("#565f89")("v0.1.0")}` }));

  // Sidebar nav
  const sidebarBox = new Box(renderer, {
    id: "sidebar",
    width: 22,
    flexShrink: 0,
    border: ["right"],
    borderStyle: "single",
    borderColor: "#414868",
    flexDirection: "column",
    paddingY: 1,
    overflow: "hidden",
  });

  sideNav = new Select(renderer, {
    id: "sidenav",
    options: NAV_OPTIONS,
    width: "100%",
    flexGrow: 1,
    showDescription: false,
    showScrollIndicator: false,
    wrapSelection: false,
    textColor: "#a9b1d6",
    selectedBackgroundColor: "#2a2e43",
    selectedTextColor: "#7aa2f7",
    backgroundColor: "transparent",
    focusedBackgroundColor: "transparent",
  });

  sidebarBox.add(sideNav);
  screen.body.add(sidebarBox);

  // Main content area
  const mainArea = new Box(renderer, {
    flexGrow: 1,
    flexDirection: "column",
    padding: 2,
    gap: 1,
  });

  contentTitle = new Text(renderer, { height: 1, flexShrink: 0, fg: "#7aa2f7" });
  contentBody = new Text(renderer, { flexGrow: 1, wrapMode: "word", fg: "#c0caf5" });

  mainArea.add(contentTitle);
  mainArea.add(contentBody);
  screen.body.add(mainArea);

  // Footer status line
  footerStatus = new Text(renderer, {
    content: t`${fg("#565f89")("↑↓ navigate  ·  Enter select  ·  Ctrl+C quit")}`,
    fg: "#565f89",
  });
  screen.footer?.add(footerStatus);

  // Wire nav events
  sideNav.on(SelectEvents.SELECTION_CHANGED, (_i, opt) => showPage((opt as SelectOption).value));
  sideNav.on(SelectEvents.ITEM_SELECTED, (_i, opt) => showPage((opt as SelectOption).value));

  // Respond to terminal resize
  screen.on(ScreenEvents.RESIZE, ({ width, height }: { width: number; height: number }) => {
    if (footerStatus) {
      footerStatus.content = t`${fg("#565f89")(
        `↑↓ navigate  ·  Ctrl+C quit  ·  terminal: ${width}×${height}`,
      )}`;
    }
  });

  showPage("dashboard");
  sideNav.focus();
}

export function destroy(rendererInstance: CliRenderer): void {
  if (screen) {
    rendererInstance.root.remove(screen.container);
    screen.container.destroy();
    screen = null;
  }
  sideNav = null;
  contentTitle = null;
  contentBody = null;
  footerStatus = null;
  renderer = null;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
