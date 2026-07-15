// Interactive example browser — the centrepiece, mirroring OpenTUI's
// packages/examples/src/index.ts (the ExampleSelector launcher). Rendered through
// @bettertui/react. Supports category grouping, a live filter input, a run↔menu
// loop, and dark/light theme switching. Keyboard is owned by an internal KeyInput
// (the public useKeyboard hook only fires on DOM events, never in a TTY).
//
// CLI:
//   tsx src/index.tsx              interactive browser
//   tsx src/index.tsx --list       compact catalogue
//   tsx src/index.tsx <slug>       run one example directly

import { render } from "@bettertui/react";
import { Box, Flex, Heading, Input, List, Provider, Separator, Text } from "@bettertui/react";
import { type KeyInput, createKeyInput } from "./lib/keyboard";
import type { ExampleMeta } from "./lib/meta";
import type { ExampleModule } from "./lib/meta";
import { CATEGORY_LABELS, CATEGORY_ORDER } from "./lib/registry";
import { META, exampleBySlug, examplesByCategory, loadExampleModule } from "./lib/registry";
import { disposeActive, mountExample } from "./lib/standalone";
import { exampleThemes } from "./lib/theme";
import type { ExampleThemeNameLiteral } from "./lib/theme";

const INSTRUCTIONS =
  "Tab/Esc switch focus | Type to filter | ↑↓/j/k move | Enter run | / filter | t theme | ctrl+c quit";

type FocusArea = "filter" | "list";
type Screen = "menu" | "example";

interface MenuItem {
  kind: "category" | "example" | "spacer" | "message";
  label: string;
  example?: ExampleMeta;
}

function buildMenuItems(filtered: ExampleMeta[], filterText: string): MenuItem[] {
  if (filtered.length === 0) {
    return [
      {
        kind: "message",
        label: "No matching examples — press Backspace to delete filter characters.",
      },
    ];
  }

  const items: MenuItem[] = [];
  let first = true;
  for (const category of CATEGORY_ORDER) {
    const inCat = filtered.filter((m) => m.category === category);
    if (inCat.length === 0) continue;
    if (!first) items.push({ kind: "spacer", label: "" });
    first = false;
    items.push({ kind: "category", label: CATEGORY_LABELS[category].toUpperCase() });
    for (const meta of inCat) {
      items.push({ kind: "example", label: `  ${meta.title}`, example: meta });
    }
  }
  void filterText;
  return items;
}

function matches(meta: ExampleMeta, filter: string): boolean {
  const hay =
    `${meta.category} ${CATEGORY_LABELS[meta.category]} ${meta.title} ${meta.description} ${meta.tags.join(" ")}`.toLowerCase();
  return hay.includes(filter);
}

class Launcher {
  private keyInput: KeyInput;
  private screen: Screen = "menu";
  private focusArea: FocusArea = "filter";
  private filterText = "";
  private selectedIndex = 0;
  private themeMode: ExampleThemeNameLiteral = "dark";
  private activeModule: ExampleModule | null = null;
  private menuItems: MenuItem[] = [];
  private rerender: () => void = () => {};
  private unsubKey: (() => void) | null = null;

  constructor(keyInput: KeyInput) {
    this.keyInput = keyInput;
    this.recompute();
  }

  attach(notify: () => void): void {
    this.rerender = notify;
    this.unsubKey = this.keyInput.on((event) => this.onKey(event));
  }

  dispose(): void {
    this.unsubKey?.();
    this.unsubKey = null;
    if (this.activeModule) {
      this.activeModule.destroy(this.keyInput);
      this.activeModule = null;
    }
    disposeActive();
  }

  private recompute(): void {
    const filtered =
      this.filterText.trim() === ""
        ? META
        : META.filter((m) => matches(m, this.filterText.toLowerCase().trim()));
    this.menuItems = buildMenuItems(filtered, this.filterText);
    if (this.selectedIndex >= this.menuItems.length) {
      this.selectedIndex = Math.max(0, this.menuItems.length - 1);
    }
  }

  private firstExampleIndex(): number {
    return this.menuItems.findIndex((m) => m.kind === "example");
  }

  private moveSelection(direction: -1 | 1, steps: number): void {
    if (this.menuItems.length === 0) return;
    let idx = this.selectedIndex;
    for (let s = 0; s < steps; s++) {
      let next = idx;
      for (let attempt = 0; attempt < this.menuItems.length; attempt++) {
        next += direction;
        if (next < 0) next = this.menuItems.length - 1;
        if (next >= this.menuItems.length) next = 0;
        if (this.menuItems[next]?.kind === "example") break;
      }
      if (this.menuItems[next]?.kind !== "example") break;
      idx = next;
    }
    this.selectedIndex = idx;
    this.rerender();
  }

  private setFocus(area: FocusArea): void {
    this.focusArea = area;
    this.rerender();
  }

  private clearFilter(): void {
    this.filterText = "";
    this.recompute();
    this.selectedIndex = Math.max(0, this.firstExampleIndex());
    this.rerender();
  }

  private async runSelected(meta: ExampleMeta): Promise<void> {
    const module = await loadExampleModule(meta.slug);
    this.activeModule = module;
    this.screen = "example";
    // mountExample renders the example into the same root via render().
    // Do NOT call this.rerender() here — it would overwrite the example
    // with the launcher's thin "Press Escape" frame on the same tick.
    mountExample(module.Example, this.keyInput);
  }

  private returnToMenu(): void {
    if (this.activeModule) {
      this.activeModule.destroy(this.keyInput);
      this.activeModule = null;
    }
    disposeActive();
    this.screen = "menu";
    this.clearFilter();
    this.setFocus("filter");
  }

  private onKey(event: {
    key: string;
    ctrl: boolean;
    shift: boolean;
    alt: boolean;
  }): void {
    if (event.ctrl && event.key === "c") {
      this.dispose();
      process.exit(0);
    }

    if (this.screen === "example") {
      if (event.key === "Escape" || event.key === "q") {
        this.returnToMenu();
      }
      return;
    }

    // In menu.
    if (event.key === "Tab" || event.key === "Escape") {
      this.setFocus(this.focusArea === "filter" ? "list" : "filter");
      return;
    }

    if (this.focusArea === "list") {
      if (event.key === "ArrowUp" || event.key === "k") {
        this.moveSelection(-1, event.shift ? 5 : 1);
        return;
      }
      if (event.key === "ArrowDown" || event.key === "j") {
        this.moveSelection(1, event.shift ? 5 : 1);
        return;
      }
      if (event.key === "/" && !event.ctrl && !event.shift) {
        this.setFocus("filter");
        return;
      }
      if (event.key === "Enter") {
        const item = this.menuItems[this.selectedIndex];
        if (item?.kind === "example" && item.example) {
          void this.runSelected(item.example);
        }
        return;
      }
    }

    if (this.focusArea === "filter") {
      if (event.key === "ArrowUp") {
        this.moveSelection(-1, 1);
        return;
      }
      if (event.key === "ArrowDown") {
        this.moveSelection(1, 1);
        return;
      }
      if (event.key === "Enter" && !event.shift) {
        const item = this.menuItems[this.selectedIndex];
        if (item?.kind === "example" && item.example) {
          void this.runSelected(item.example);
        }
        return;
      }
      if (event.key === "Backspace") {
        this.filterText = this.filterText.slice(0, -1);
        this.recompute();
        if (this.firstExampleIndex() >= 0) this.selectedIndex = this.firstExampleIndex();
        this.rerender();
        return;
      }
      if (event.key === "Escape") {
        this.clearFilter();
        return;
      }
      // Printable characters feed the filter.
      if (!event.ctrl && !event.alt && event.key.length === 1 && event.key >= " ") {
        this.handleFilterInput(this.filterText + event.key);
        return;
      }
    }

    if (!event.ctrl && event.key === "t") {
      this.themeMode = this.themeMode === "dark" ? "light" : "dark";
      this.rerender();
    }
  }

  private handleFilterInput(value: string): void {
    this.filterText = value;
    this.recompute();
    if (this.firstExampleIndex() >= 0) this.selectedIndex = this.firstExampleIndex();
    this.rerender();
  }

  view() {
    if (this.screen === "example") {
      // The example mounts its own UI via mountExample; render a thin frame with
      // the return hint so the screen isn't blank before the example paints.
      return (
        <Box height="100%" style={{ fg: exampleThemes[this.themeMode].colors.textDim }}>
          <Text dim>Press Escape or q to return to the menu.</Text>
        </Box>
      );
    }

    const theme = exampleThemes[this.themeMode];
    const focusFilter = this.focusArea === "filter";
    const focusList = this.focusArea === "list";

    const listItems = this.menuItems.map((item, index) => {
      if (item.kind === "category") {
        return { id: `cat-${index}`, label: item.label, disabled: true };
      }
      if (item.kind === "spacer" || item.kind === "message") {
        return { id: `sp-${index}`, label: item.label, disabled: true };
      }
      const selected = index === this.selectedIndex && focusList;
      return {
        id: item.example?.slug ?? `ex-${index}`,
        label: (selected ? "▶ " : "  ") + item.label,
        disabled: false,
      };
    });

    return (
      <Provider theme={theme}>
        <Flex flexDirection="column" gap={1} padding={1}>
          <Heading level={1}>BetterTUI Examples</Heading>
          <Box
            style={{
              border: focusFilter
                ? { fg: theme.colors.borderFocused }
                : { fg: theme.colors.border },
              padding: { top: 0, right: 1, bottom: 0, left: 1 },
            }}
          >
            <Input
              value={this.filterText}
              placeholder="Filter examples..."
              onChange={(v: string) => this.handleFilterInput(v)}
            />
          </Box>
          <Box
            flexGrow={1}
            style={{
              border: focusList ? { fg: theme.colors.borderFocused } : { fg: theme.colors.border },
              padding: { top: 0, right: 1, bottom: 0, left: 1 },
            }}
          >
            <List
              items={listItems}
              selectedId={listItems[this.selectedIndex]?.id ?? ""}
              onSelect={(id: string) => {
                const idx = listItems.findIndex((i) => i.id === id);
                if (idx >= 0) {
                  this.selectedIndex = idx;
                  const item = this.menuItems[idx];
                  if (item?.kind === "example" && item.example) {
                    void this.runSelected(item.example);
                  }
                }
              }}
            />
          </Box>
          <Separator />
          <Text dim>{INSTRUCTIONS}</Text>
          <Text dim>Theme: {this.themeMode}</Text>
        </Flex>
      </Provider>
    );
  }
}

function listExamples(): void {
  console.log(`BetterTUI Examples (${META.length})\n`);
  const byCat = examplesByCategory();
  for (const category of CATEGORY_ORDER) {
    const inCat = byCat.get(category);
    if (!inCat || inCat.length === 0) continue;
    console.log(`${CATEGORY_LABELS[category]}`);
    for (const e of inCat) {
      console.log(`  ${e.slug.padEnd(26)} ${e.title}  [L${e.level}]`);
    }
    console.log("");
  }
}

async function runOne(slug: string): Promise<void> {
  const meta = exampleBySlug(slug);
  if (!meta) {
    console.error(`Unknown example: ${slug}`);
    console.error(`Run with --list to see ${META.length} examples.`);
    process.exit(1);
  }
  const keyInput = createKeyInput();
  keyInput.start();
  const module = await loadExampleModule(meta.slug);
  const unsub = keyInput.on((event) => {
    if ((event.key === "q" || event.key === "Escape") && !event.ctrl) {
      module.destroy(keyInput);
      keyInput.stop();
      process.exit(0);
    }
  });
  module.run(keyInput);
  void unsub;
}

function printMenu(): void {
  console.log("BetterTUI Example Browser\n");
  console.log(`Browse ${META.length} examples. Run one directly:\n`);
  const byCat = examplesByCategory();
  for (const category of CATEGORY_ORDER) {
    const inCat = byCat.get(category);
    if (!inCat || inCat.length === 0) continue;
    console.log(`${CATEGORY_LABELS[category]}`);
    for (const e of inCat) {
      console.log(`  tsx src/index.tsx ${e.slug}`);
    }
  }
  console.log("\nOr: tsx src/index.tsx --list   (compact catalogue)");
}

const arg = process.argv[2];
if (arg === "--list") {
  listExamples();
} else if (arg) {
  void runOne(arg);
} else if (process.stdin.isTTY) {
  const keyInput = createKeyInput();
  keyInput.start();
  const launcher = new Launcher(keyInput);
  const handle = render(<LauncherView launcher={launcher} />);
  launcher.attach(() => {
    const next = render(<LauncherView launcher={launcher} />);
    void next;
  });
  process.on("SIGINT", () => {
    launcher.dispose();
    handle.dispose();
    keyInput.stop();
    process.exit(0);
  });
} else {
  printMenu();
}

function LauncherView({ launcher }: { launcher: Launcher }): React.ReactNode {
  return launcher.view();
}

export { examples, exampleBySlug, examplesByCategory } from "./lib/registry";
export { CATEGORY_ORDER, CATEGORY_LABELS } from "./lib/registry";
