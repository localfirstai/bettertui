import type { BoxOptions } from "@bettertui/core";
import { Box } from "@bettertui/core";
import { TabSelect, TabSelectEvents } from "@bettertui/core";
import type { CliRenderer, TabOption } from "@bettertui/core";
import type { ColorInput } from "@bettertui/core";
import { RenderableEvents } from "@bettertui/core";
import { parseColor } from "@bettertui/core";

export interface TabObject {
  title: string;
  init(tabGroup: Box): void;
  update?(deltaMs: number, tabGroup: Box): void;
  show?(): void;
  hide?(): void;
}

interface Tab {
  title: string;
  tabObject: TabObject;
  group: Box;
  initialized: boolean;
}

export interface TabControllerOptions extends BoxOptions {
  backgroundColor?: ColorInput;
  textColor?: ColorInput;
  tabBarHeight?: number;
  tabBarBackgroundColor?: ColorInput;
  selectedBackgroundColor?: ColorInput;
  selectedTextColor?: ColorInput;
  selectedDescriptionColor?: ColorInput;
  showDescription?: boolean;
  showUnderline?: boolean;
  showScrollArrows?: boolean;
  tabPadding?: number;
  tabGap?: number;
  minTabWidth?: number;
}

export enum TabControllerEvents {
  TAB_CHANGED = "tabChanged",
}

export class TabController extends Box {
  public tabs: Tab[] = [];
  private currentTabIndex = 0;
  private tabSelectElement: TabSelect;
  private _tabBarHeight: number;
  private _frameCallback: ((deltaMs: number) => Promise<void>) | null = null;
  private _renderer2: CliRenderer;

  constructor(id: string, renderer: CliRenderer, options: TabControllerOptions) {
    super(renderer, {
      ...options,
      id,
      flexDirection: options.flexDirection ?? "column",
      flexGrow: options.flexGrow ?? 1,
      flexShrink: options.flexShrink ?? 1,
    });
    this._renderer2 = renderer;
    this._tabBarHeight = options.tabBarHeight || 4;

    this.tabSelectElement = new TabSelect(renderer, {
      id: `${id}-tabs`,
      width: "100%",
      height: this._tabBarHeight,
      flexGrow: 0,
      flexShrink: 0,
      options: [],
      tabWidth: 0, // Auto-width based on content
      minTabWidth: 10,
      tabPadding: 2,
      tabGap: 2,
      selectedBackgroundColor: options.selectedBackgroundColor || "#333333",
      selectedTextColor: options.selectedTextColor || "#FFFF00",
      textColor: parseColor(options.textColor || "#FFFFFF"),
      descriptionColor: options.selectedDescriptionColor || "#FFFFFF",
      backgroundColor: options.tabBarBackgroundColor || options.backgroundColor || "transparent",
      showDescription: options.showDescription ?? true,
      showUnderline: options.showUnderline ?? true,
      showScrollArrows: options.showScrollArrows ?? false, // Disabled by default for full-width tabs
    });

    this.tabSelectElement.on(TabSelectEvents.SELECTION_CHANGED, (index: number) => {
      this.switchToTab(index);
    });

    this.add(this.tabSelectElement);

    this._frameCallback = async (deltaMs) => {
      this._update(deltaMs);
    };
    this._renderer2.setFrameCallback(this._frameCallback);
  }

  public addTab(tabObject: TabObject): Tab {
    const tabGroup = new Box(this._renderer2, {
      id: `${this._id}-tab-${this.tabs.length}`,
      flexDirection: "column",
      flexGrow: 0,
      flexShrink: 0,
      visible: false,
      width: "100%",
    });

    this.add(tabGroup);

    const tab: Tab = {
      title: tabObject.title,
      tabObject,
      group: tabGroup,
      initialized: false,
    };
    this.tabs.push(tab);

    this.updateTabSelectOptions();
    return tab;
  }

  private updateTabSelectOptions(): void {
    const opts: TabOption[] = this.tabs.map((tab, index) => ({
      name: tab.title,
      description: `Tab ${index + 1}/${this.tabs.length} — ←/→ to switch | Ctrl+C to exit`,
      value: index,
    }));

    this.tabSelectElement.options = opts;

    if (this.tabs.length === 1) {
      const firstTab = this.getCurrentTab();
      firstTab.group.setLayout({ flexGrow: 1, flexShrink: 1 });
      firstTab.group.visible = true;
      this.initializeTab(firstTab);

      if (firstTab.tabObject.show) {
        firstTab.tabObject.show();
      }
    }
  }

  private initializeTab(tab: Tab): void {
    if (!tab.initialized) {
      tab.tabObject.init(tab.group);
      tab.initialized = true;
    }
  }

  public getCurrentTab(): Tab {
    const tab = this.tabs[this.currentTabIndex];
    if (!tab) throw new Error(`No tab at index ${this.currentTabIndex}`);
    return tab;
  }

  public getCurrentTabGroup(): Box {
    return this.getCurrentTab().group;
  }

  public switchToTab(index: number): void {
    if (index < 0 || index >= this.tabs.length) return;
    if (index === this.currentTabIndex) return;

    const currentTab = this.getCurrentTab();
    currentTab.group.visible = false;
    if (currentTab.tabObject.hide) {
      currentTab.tabObject.hide();
    }

    this.currentTabIndex = index;
    this.tabSelectElement.selectedIndex = index;

    const newTab = this.getCurrentTab();
    newTab.group.setLayout({ flexGrow: 1, flexShrink: 1 });
    newTab.group.visible = true;

    this.initializeTab(newTab);

    if (newTab.tabObject.show) {
      newTab.tabObject.show();
    }

    this.emit(TabControllerEvents.TAB_CHANGED, index, newTab);
  }

  public nextTab(): void {
    this.switchToTab((this.currentTabIndex + 1) % this.tabs.length);
  }

  public previousTab(): void {
    this.switchToTab((this.currentTabIndex - 1 + this.tabs.length) % this.tabs.length);
  }

  private _update(deltaMs: number): void {
    const currentTab = this.tabs[this.currentTabIndex];
    if (currentTab?.tabObject.update) {
      currentTab.tabObject.update(deltaMs, currentTab.group);
    }
  }

  public getCurrentTabIndex(): number {
    return this.currentTabIndex;
  }

  public getTabSelectElement(): TabSelect {
    return this.tabSelectElement;
  }

  override focus(): void {
    this.tabSelectElement.focus();
    this.emit(RenderableEvents.FOCUSED);
  }

  override blur(): void {
    this.tabSelectElement.blur();
    this.emit(RenderableEvents.BLURRED);
  }

  public onResize(width: number, height: number): void {
    this.width = width;
    this.height = height;

    this.tabSelectElement.width = width;
    this.tabSelectElement.height = this._tabBarHeight;
  }

  override destroy(): void {
    this.tabSelectElement.blur();

    if (this._frameCallback) {
      this._renderer2.removeFrameCallback(this._frameCallback);
      this._frameCallback = null;
    }

    for (const tab of this.tabs) {
      tab.group.destroy();
    }

    this.tabSelectElement.destroy();

    this.removeAllListeners();
    super.destroy();
  }
}
