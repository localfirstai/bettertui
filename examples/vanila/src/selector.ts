/**
 * Example Selector - Interactive menu for browsing examples
 * Following OpenTUI's ExampleSelector pattern exactly
 */

import { CATEGORY_LABELS, getExampleSections } from "./examples";
import type { Example, ExampleCategory } from "./lib/types";
import type { CliRenderer, KeyEvent } from "./renderer";

type FocusArea = "filter" | "list";

interface MenuOption {
  kind: "category" | "example" | "spacer";
  label: string;
  example?: Example;
  category?: ExampleCategory;
}

export class ExampleSelector {
  private renderer: CliRenderer;
  private focusArea: FocusArea = "list";
  private selectedIndex = 0;
  private filterText = "";
  private menuOptions: MenuOption[] = [];
  private themeMode: "dark" | "light" = "dark";
  private running = false;

  constructor(renderer: CliRenderer) {
    this.renderer = renderer;
    this.rebuildMenu();
  }

  async run(): Promise<void> {
    this.renderer.start();
    this.setupKeyBindings();
    this.render();

    this.running = true;
    while (this.running) {
      await new Promise((resolve) => setTimeout(resolve, 50));
    }
  }

  private setupKeyBindings(): void {
    this.renderer.addKeyBinding("global", "quit", "ctrl+c", "quit", "Exit", 100);
    this.renderer.addKeyBinding("global", "quit_esc", "escape", "quit", "Exit", 99);

    this.renderer.keyHandler.on("keypress", (key: KeyEvent) => this.handleKey(key));
  }

  private handleKey(key: KeyEvent): void {
    const cmd = this.renderer.handleKey(key.sequence);

    if (cmd === "quit") {
      this.running = false;
      return;
    }

    if (key.name === "tab") {
      this.focusArea = this.focusArea === "filter" ? "list" : "filter";
      this.render();
      return;
    }

    if (this.focusArea === "filter") {
      this.handleFilterKey(key);
    } else {
      this.handleListKey(key);
    }
  }

  private handleFilterKey(key: KeyEvent): void {
    if (key.name === "backspace") {
      this.filterText = this.filterText.slice(0, -1);
      this.rebuildMenu();
      this.selectedIndex = this.findFirstExampleIndex();
      this.render();
      return;
    }

    if (key.name === "up" || key.name === "k") {
      this.moveSelection(-1);
      return;
    }

    if (key.name === "down" || key.name === "j") {
      this.moveSelection(1);
      return;
    }

    if (key.name === "enter") {
      this.runSelectedExample();
      return;
    }

    if (key.name.length === 1 && !key.ctrl && !key.alt) {
      this.filterText += key.name;
      this.rebuildMenu();
      this.selectedIndex = this.findFirstExampleIndex();
      this.render();
    }
  }

  private handleListKey(key: KeyEvent): void {
    if (key.name === "up" || key.name === "k") {
      this.moveSelection(key.shift ? -5 : -1);
      return;
    }

    if (key.name === "down" || key.name === "j") {
      this.moveSelection(key.shift ? 5 : 1);
      return;
    }

    if (key.name === "enter") {
      this.runSelectedExample();
      return;
    }

    if (key.name === "/" || key.name === "f") {
      this.focusArea = "filter";
      this.render();
      return;
    }

    if (key.name === "t") {
      this.toggleTheme();
      return;
    }
  }

  private moveSelection(delta: number): void {
    const count = this.menuOptions.length;
    if (count === 0) return;

    let newIndex = this.selectedIndex;
    const step = delta > 0 ? 1 : -1;
    const steps = Math.abs(delta);

    for (let i = 0; i < steps; i++) {
      for (let attempt = 0; attempt < count; attempt++) {
        newIndex = (newIndex + step + count) % count;
        if (this.menuOptions[newIndex]?.kind === "example") break;
      }
    }

    if (this.menuOptions[newIndex]?.kind === "example") {
      this.selectedIndex = newIndex;
      this.render();
    }
  }

  private findFirstExampleIndex(): number {
    for (let i = 0; i < this.menuOptions.length; i++) {
      if (this.menuOptions[i]?.kind === "example") return i;
    }
    return 0;
  }

  private rebuildMenu(): void {
    const options: MenuOption[] = [];
    const sections = getExampleSections();

    for (const section of sections) {
      const filteredExamples = section.examples.filter((ex) => this.matchesFilter(ex));

      if (filteredExamples.length === 0) continue;

      if (options.length > 0) {
        options.push({ kind: "spacer", label: "" });
      }

      options.push({
        kind: "category",
        label: CATEGORY_LABELS[section.category].toUpperCase(),
        category: section.category,
      });

      for (const ex of filteredExamples) {
        options.push({ kind: "example", label: ex.name, example: ex });
      }
    }

    this.menuOptions = options;
  }

  private matchesFilter(example: Example): boolean {
    if (!this.filterText) return true;
    const search = this.filterText.toLowerCase();
    return (
      example.name.toLowerCase().includes(search) ||
      example.description.toLowerCase().includes(search) ||
      example.slug.toLowerCase().includes(search)
    );
  }

  private async runSelectedExample(): Promise<void> {
    const option = this.menuOptions[this.selectedIndex];
    if (option?.kind !== "example" || !option.example) return;

    try {
      await option.example.run?.(this.renderer);
    } catch (error) {
      console.error("Example failed:", error);
    }

    option.example.destroy?.(this.renderer);

    this.renderer.clearTree();
    this.renderer.clearScreen();
    this.render();
  }

  private toggleTheme(): void {
    this.themeMode = this.themeMode === "dark" ? "light" : "dark";
    this.render();
  }

  private render(): void {
    this.renderer.clearScreen();

    const lines: string[] = [];

    lines.push("\x1b[1;34m  BetterTUI Examples\x1b[0m");
    lines.push("");

    const filterDisplay = this.filterText || "Type to filter...";
    const filterFocused = this.focusArea === "filter";
    const filterPrefix = filterFocused ? ">" : " ";
    lines.push(`  ${filterPrefix} [\x1b[36m${filterDisplay}\x1b[0m]`);
    lines.push("");

    for (let i = 0; i < this.menuOptions.length; i++) {
      const opt = this.menuOptions[i];

      if (!opt) continue;

      if (opt.kind === "spacer") {
        lines.push("");
        continue;
      }

      if (opt.kind === "category") {
        lines.push(`  \x1b[1;33m${opt.label}\x1b[0m`);
        continue;
      }

      const selected = i === this.selectedIndex && this.focusArea === "list";
      const prefix = selected ? "  → " : "    ";
      const name = selected ? `\x1b[1;32m${opt.label}\x1b[0m` : opt.label;
      const desc = selected ? "" : ` \x1b[90m${opt.example?.description ?? ""}\x1b[0m`;

      lines.push(`${prefix}${name}${desc}`);
    }

    lines.push("");
    lines.push(
      "  \x1b[90mTab: switch focus | ↑/↓: navigate | Enter: run | /: filter | t: theme | Ctrl+C: quit\x1b[0m",
    );
    lines.push(`  \x1b[90mTheme: ${this.themeMode}\x1b[0m`);

    process.stdout.write(`\x1b[1;1H${lines.join("\n")}`);
  }
}
