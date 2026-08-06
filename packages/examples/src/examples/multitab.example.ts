/**
 * Multi-Tab Demo — Polished showcase of BetterTUI components.
 *
 * Demonstrates: Screen, TabController, flexbox layouts, text styling,
 * border variants, animations, and interactive controls.
 */
import {
  Box,
  type RawKeyEvent,
  Screen,
  Text,
  bold,
  createCliRenderer,
  dim,
  fg,
  italic,
  t,
  underline,
} from "@bettertui/core";
import type { CliRenderer, ThemeMode } from "@bettertui/core";
import { DEFAULT_THEME_MODE, getComponentTheme, getThemeTokens } from "../constants/theme";
import { getBorderFromSides } from "../lib/borderSides";
import { hsvToRgb, rgbaToHex } from "../lib/colorUtils";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";
import { TabController } from "../lib/tabController";

let globalScreen: Screen | null = null;
let globalTabController: TabController | null = null;
let globalKeyboardHandler: ((key: RawKeyEvent) => void) | null = null;
let colorDisplayText: Text | null = null;

interface DemoTheme {
  bg: string;
  surface: string;
  surfaceHighlight: string;
  headerBg: string;
  headerFg: string;
  footerBg: string;
  footerFg: string;
  border: string;
  borderHighlight: string;
  primary: string;
  success: string;
  warning: string;
  danger: string;
  info: string;
  text: string;
  textMuted: string;
}

let theme: DemoTheme = buildTheme(DEFAULT_THEME_MODE);

function buildTheme(mode: ThemeMode): DemoTheme {
  const tokens = getThemeTokens(mode);
  const comp = getComponentTheme(mode);
  return {
    bg: tokens.background,
    surface: tokens.secondary,
    surfaceHighlight: tokens.primary,
    headerBg: tokens.secondary,
    headerFg: tokens.primary,
    footerBg: tokens.muted,
    footerFg: tokens.mutedForeground,
    border: comp.border,
    borderHighlight: tokens.primary,
    primary: tokens.primary,
    success: tokens.success,
    warning: tokens.warning,
    danger: tokens.destructive,
    info: tokens.info,
    text: tokens.foreground,
    textMuted: tokens.mutedForeground,
  };
}

export function run(renderer: CliRenderer): void {
  theme = buildTheme(renderer.themeMode ?? DEFAULT_THEME_MODE);

  renderer.start();
  renderer.setBackgroundColor(theme.bg);

  globalScreen = new Screen(renderer, {
    id: "multitab-screen",
    backgroundColor: theme.bg,
    header: {
      id: "multitab-header",
      height: 3,
      backgroundColor: theme.headerBg,
      border: true,
      borderStyle: "double",
      borderColor: theme.borderHighlight,
      title: " BetterTUI — Component Showcase ",
      titleAlignment: "center",
      alignItems: "center",
      justifyContent: "flex-start",
      paddingX: 2,
    },
    body: {
      id: "multitab-body",
      flexDirection: "column",
      padding: 0,
    },
    footer: {
      id: "multitab-footer",
      height: 3,
      backgroundColor: theme.footerBg,
      border: true,
      borderStyle: "single",
      borderColor: theme.border,
      alignItems: "center",
      justifyContent: "center",
    },
  });

  const headerLeft = new Text(renderer, {
    id: "multitab-header-left",
    content: t`${dim("←/→")} Navigate Tabs`,
    fg: theme.textMuted,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 1,
    flexShrink: 1,
    textAlign: "left",
  });
  globalScreen.header?.add(headerLeft);

  const headerRight = new Text(renderer, {
    id: "multitab-header-right",
    content: t`${dim("Ctrl+C")} Exit`,
    fg: theme.textMuted,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 0,
    flexShrink: 0,
    textAlign: "right",
  });
  globalScreen.header?.add(headerRight);

  const footerText = new Text(renderer, {
    id: "multitab-footer-text",
    content: t`${fg(theme.primary)("●")} Multi-Tab Demo  ${fg(theme.textMuted)("│")}  ${dim("← →")} navigate  ${fg(theme.textMuted)("│")}  ${dim("Ctrl+C")} exit  ${fg(theme.textMuted)("│ v1.0.0")}`,
    fg: theme.footerFg,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 1,
    flexShrink: 1,
    textAlign: "center",
  });
  globalScreen.footer?.add(footerText);

  globalTabController = new TabController("main-controller", renderer, {
    id: "tab-controller",
    flexGrow: 1,
    flexShrink: 1,
    tabBarHeight: 3,
    tabBarBackgroundColor: theme.surface,
    selectedBackgroundColor: theme.surfaceHighlight,
    selectedTextColor: theme.bg,
    textColor: theme.text,
    selectedDescriptionColor: theme.textMuted,
    tabPadding: 3,
    tabGap: 1,
  });
  globalScreen.body.add(globalTabController as unknown as Box);

  globalTabController.addTab({
    title: "Typography",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      const content = new Box(renderer, {
        id: "text-content",
        flexDirection: "row",
        flexGrow: 1,
        flexShrink: 1,
        padding: 2,
        gap: 2,
      });
      g.add(content);

      // ── Text Styles panel ──────────────────────────────────────────────────
      const attrCard = new Box(renderer, {
        id: "attr-card",
        flexDirection: "column",
        flexGrow: 0,
        flexShrink: 0,
        width: 32,
        border: true,
        borderStyle: "round",
        borderColor: theme.border,
        backgroundColor: theme.surface,
        title: "Text Styles",
        titleAlignment: "center",
        padding: 2,
        gap: 1,
      });
      content.add(attrCard);

      const styles = [
        { label: t`${bold("Bold")}`, desc: "Bold weight" },
        { label: t`${italic("Italic")}`, desc: "Italic style" },
        { label: t`${underline("Underline")}`, desc: "Underlined" },
        { label: t`${dim("Dim")}`, desc: "Reduced opacity" },
        { label: t`${bold(italic("Bold+Italic"))}`, desc: "Combined" },
      ];

      for (const style of styles) {
        const row = new Box(renderer, {
          flexDirection: "row",
          alignItems: "center",
          gap: 2,
        });
        attrCard.add(row);
        row.add(
          new Text(renderer, {
            content: style.label,
            fg: theme.text,
            zIndex: 10,
          }),
        );
        row.add(
          new Text(renderer, {
            content: t`${dim(style.desc)}`,
            fg: theme.textMuted,
            zIndex: 10,
          }),
        );
      }

      // ── Color Palette panel ────────────────────────────────────────────────
      const colorCard = new Box(renderer, {
        id: "color-card",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        border: true,
        borderStyle: "round",
        borderColor: theme.border,
        backgroundColor: theme.surface,
        title: "Color Palette / Gradient",
        titleAlignment: "center",
        padding: 2,
        gap: 1,
      });
      content.add(colorCard);

      // Static hue-spectrum bar
      const colorRow = new Box(renderer, {
        id: "color-spectrum-row",
        flexDirection: "row",
        flexGrow: 0,
        flexShrink: 0,
        height: 1,
        alignItems: "center",
      });
      colorCard.add(colorRow);
      for (let i = 0; i < 48; i++) {
        const hue = (i / 48) * 360;
        colorRow.add(
          new Text(renderer, {
            content: "█",
            fg: rgbaToHex(hsvToRgb(hue, 1, 1)),
            zIndex: 10,
          }),
        );
      }

      // Animated gradient — a single Text node updated every frame via ANSI codes.
      // Using one node avoids per-pixel DOM allocation and the absolute-positioning
      // complexity of the previous wheel approach.
      const gradientBox = new Box(renderer, {
        id: "gradient-box",
        flexGrow: 1,
        flexShrink: 1,
        marginTop: 1,
        border: true,
        borderStyle: "single",
        borderColor: theme.border,
        backgroundColor: theme.bg,
        overflow: "hidden",
      });
      colorCard.add(gradientBox);

      colorDisplayText = new Text(renderer, {
        id: "gradient-text",
        content: "",
        flexGrow: 1,
        flexShrink: 1,
        zIndex: 5,
      });
      gradientBox.add(colorDisplayText);
    },

    update: (_deltaMs: number, _tabGroup: unknown) => {
      if (!colorDisplayText) return;

      const tw = renderer.terminalWidth;
      const th = renderer.terminalHeight;
      // Approximate inner dimensions of gradientBox (border on each side + layout overhead)
      const cols = Math.max(4, tw - 46);
      const rows = Math.max(1, th - 26);

      const hueOffset = ((Date.now() / 1000) * 60) % 360;

      const lines: string[] = [];
      for (let y = 0; y < rows; y++) {
        const sat = 0.3 + 0.7 * (y / Math.max(1, rows - 1));
        let line = "";
        for (let x = 0; x < cols; x++) {
          const hue = ((x / cols) * 360 + hueOffset) % 360;
          const rgba = hsvToRgb(hue, sat, 1);
          const r = Math.round(rgba.r * 255);
          const gr = Math.round(rgba.g * 255);
          const b = Math.round(rgba.b * 255);
          line += `\x1b[38;2;${r};${gr};${b}m█`;
        }
        lines.push(`${line}\x1b[0m`);
      }
      colorDisplayText.content = lines;
    },

    hide: () => {
      if (colorDisplayText) colorDisplayText.content = "";
    },
  });

  globalTabController.addTab({
    title: "Layouts",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      const content = new Box(renderer, {
        id: "layout-content",
        flexDirection: "row",
        flexGrow: 1,
        flexShrink: 1,
        padding: 2,
        gap: 2,
      });
      g.add(content);

      const leftCol = new Box(renderer, {
        id: "layout-left",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        overflow: "hidden",
        gap: 1,
      });
      content.add(leftCol);

      const box1 = new Box(renderer, {
        id: "box1",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: 8,
        backgroundColor: theme.surface,
        borderStyle: "single",
        borderColor: theme.border,
        title: "Single Border",
        titleAlignment: "center",
        border: true,
      });
      leftCol.add(box1);
      box1.add(
        new Text(renderer, {
          content: "Classic",
          fg: theme.text,
          zIndex: 10,
        }),
      );

      const box2 = new Box(renderer, {
        id: "box2",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: 8,
        backgroundColor: theme.surface,
        borderStyle: "double",
        borderColor: theme.primary,
        title: "Double Border",
        titleAlignment: "center",
        border: true,
      });
      leftCol.add(box2);
      box2.add(
        new Text(renderer, {
          content: "Elegant",
          fg: theme.text,
          zIndex: 10,
        }),
      );

      const rightCol = new Box(renderer, {
        id: "layout-right",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        overflow: "hidden",
        gap: 1,
      });
      content.add(rightCol);

      const box3 = new Box(renderer, {
        id: "box3",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: 8,
        backgroundColor: theme.surface,
        borderStyle: "round",
        borderColor: theme.success,
        title: "Rounded",
        titleAlignment: "center",
        border: true,
      });
      rightCol.add(box3);
      box3.add(
        new Text(renderer, {
          content: "Modern",
          fg: theme.text,
          zIndex: 10,
        }),
      );

      const box4 = new Box(renderer, {
        id: "box4",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: 8,
        backgroundColor: theme.surface,
        borderStyle: "thick",
        borderColor: theme.warning,
        title: "Thick",
        titleAlignment: "center",
        border: true,
      });
      rightCol.add(box4);
      box4.add(
        new Text(renderer, {
          content: "Bold",
          fg: theme.text,
          zIndex: 10,
        }),
      );
    },
  });

  let borderPhase = 0;

  globalTabController.addTab({
    title: "Borders",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      const content = new Box(renderer, {
        id: "border-content",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        padding: 2,
        gap: 2,
      });
      g.add(content);

      const grid = new Box(renderer, {
        id: "border-grid",
        flexDirection: "row",
        flexShrink: 1,
        flexWrap: "wrap",
        gap: 2,
      });
      content.add(grid);

      const styles: Array<{
        id: string;
        name: string;
        style: "single" | "double" | "round" | "ascii" | "thick" | "dashed";
        color: string;
      }> = [
        { id: "single", name: "Single", style: "single", color: theme.border },
        { id: "double", name: "Double", style: "double", color: theme.primary },
        { id: "round", name: "Rounded", style: "round", color: theme.success },
        { id: "thick", name: "Thick", style: "thick", color: theme.warning },
        { id: "ascii", name: "ASCII", style: "ascii", color: theme.info },
        { id: "dashed", name: "Dashed", style: "dashed", color: theme.danger },
      ];

      for (const s of styles) {
        const box = new Box(renderer, {
          id: s.id,
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          width: 14,
          height: 6,
          backgroundColor: theme.surface,
          borderStyle: s.style,
          borderColor: s.color,
          title: s.name,
          titleAlignment: "center",
          border: true,
        });
        grid.add(box);
      }

      const animBox = new Box(renderer, {
        id: "border-anim-box",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        flexGrow: 1,
        flexShrink: 1,
        backgroundColor: theme.surface,
        borderStyle: "double",
        borderColor: theme.borderHighlight,
        title: "Animated Borders",
        titleAlignment: "center",
        border: true,
      });
      content.add(animBox);

      animBox.add(
        new Text(renderer, {
          id: "border-phase-text",
          content: "Phase: 1/8",
          fg: theme.textMuted,
          zIndex: 10,
        }),
      );
    },
    update: (_deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const time = Date.now() / 1000;
      const phase = Math.floor(time % 8);

      if (phase !== borderPhase) {
        borderPhase = phase;

        const sides = {
          top: [0, 3, 5, 7].includes(phase),
          right: [1, 3, 6, 7].includes(phase),
          bottom: [2, 3, 5, 7].includes(phase),
          left: [4, 5, 6, 7].includes(phase),
        };

        const box = g.getRenderable("border-anim-box") as Box;
        if (box) {
          box.border = getBorderFromSides(sides);
        }

        const text = g.getRenderable("border-phase-text") as Text;
        if (text) {
          text.content = `Phase: ${phase + 1}/8`;
        }
      }
    },
  });

  let animPos = 3;
  let animDir = 1;

  globalTabController.addTab({
    title: "Motion",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      const content = new Box(renderer, {
        id: "anim-content",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        padding: 2,
        gap: 2,
      });
      g.add(content);

      const stage = new Box(renderer, {
        id: "anim-stage",
        position: "relative",
        flexGrow: 1,
        flexShrink: 1,
        border: true,
        borderStyle: "round",
        borderColor: theme.border,
        backgroundColor: theme.surface,
      });
      content.add(stage);

      const traveler = new Box(renderer, {
        id: "traveler",
        position: "absolute",
        left: 3,
        top: 2,
        width: 16,
        height: 3,
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        backgroundColor: theme.primary,
        borderStyle: "round",
        borderColor: theme.borderHighlight,
        border: true,
      });
      stage.add(traveler);

      traveler.add(
        new Text(renderer, {
          content: t`${bold("Moving Box")}`,
          fg: theme.bg,
          zIndex: 10,
        }),
      );
    },
    update: (deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const stage = g.getRenderable("anim-stage") as Box;
      if (!stage) return;

      const traveler = stage.getRenderable("traveler") as Box;
      if (!traveler) return;

      const delta = Math.min(deltaMs / 1000, 0.1);
      animPos += 20 * animDir * delta;

      const maxPos = Math.max(10, renderer.terminalWidth - 22);
      if (animPos > maxPos) {
        animPos = maxPos;
        animDir = -1;
      } else if (animPos < 2) {
        animPos = 2;
        animDir = 1;
      }

      traveler.setPosition({ left: Math.round(animPos), top: 2 });

      const hue = (Date.now() / 20) % 360;
      traveler.backgroundColor = hsvToRgb(hue, 0.8, 1);
    },
  });

  globalTabController.addTab({
    title: "Alignment",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      const content = new Box(renderer, {
        id: "align-content",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        padding: 2,
        gap: 2,
        alignItems: "center",
        justifyContent: "center",
      });
      g.add(content);

      content.add(
        new Text(renderer, {
          content: t`${fg(theme.textMuted)("Title alignment options for bordered boxes:")}`,
          fg: theme.textMuted,
          zIndex: 10,
        }),
      );

      const row = new Box(renderer, {
        flexDirection: "row",
        gap: 2,
        marginTop: 1,
      });
      content.add(row);

      const alignments: Array<{
        align: "left" | "center" | "right";
        color: string;
      }> = [
        { align: "left", color: theme.danger },
        { align: "center", color: theme.success },
        { align: "right", color: theme.info },
      ];

      for (const a of alignments) {
        const box = new Box(renderer, {
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          width: 20,
          height: 5,
          backgroundColor: theme.surface,
          borderStyle: "round",
          borderColor: a.color,
          title: `${a.align.charAt(0).toUpperCase()}${a.align.slice(1)}`,
          titleAlignment: a.align,
          border: true,
        });
        row.add(box);
      }
    },
  });

  const borderSides = { top: true, right: true, bottom: true, left: true };

  globalTabController.addTab({
    title: "Controls",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      const content = new Box(renderer, {
        id: "ctrl-content",
        flexDirection: "row",
        flexGrow: 1,
        flexShrink: 1,
        padding: 2,
        gap: 2,
      });
      g.add(content);

      const previewCol = new Box(renderer, {
        id: "preview-col",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        gap: 1,
      });
      content.add(previewCol);

      const demoBox = new Box(renderer, {
        id: "demo-box",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        flexGrow: 1,
        backgroundColor: theme.surface,
        borderStyle: "double",
        borderColor: theme.borderHighlight,
        border: true,
      });
      previewCol.add(demoBox);

      demoBox.add(
        new Text(renderer, {
          id: "demo-text",
          content: t`${fg(theme.text)("Border Demo")}`,
          zIndex: 10,
        }),
      );

      const ctrlCol = new Box(renderer, {
        id: "ctrl-col",
        flexDirection: "column",
        flexGrow: 0,
        flexShrink: 0,
        width: 28,
        gap: 1,
        padding: 1,
        border: true,
        borderStyle: "round",
        borderColor: theme.border,
        backgroundColor: theme.surface,
        title: "Keyboard Controls",
        titleAlignment: "center",
      });
      content.add(ctrlCol);

      const controls: Array<[string, string]> = [
        ["t", "Toggle top border"],
        ["r", "Toggle right border"],
        ["b", "Toggle bottom border"],
        ["l", "Toggle left border"],
      ];

      for (const [key, desc] of controls) {
        const line = new Box(renderer, {
          flexDirection: "row",
          gap: 2,
          alignItems: "center",
        });
        ctrlCol.add(line);

        const keyLabel = new Text(renderer, {
          content: t`${bold(fg(theme.primary)(key.toUpperCase()))}`,
          zIndex: 10,
        });
        line.add(keyLabel);

        const description = new Text(renderer, {
          content: desc,
          fg: theme.textMuted,
          zIndex: 10,
        });
        line.add(description);
      }
    },
    update: (_deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const box = g.getRenderable("demo-box") as Box;
      if (box) {
        box.border = getBorderFromSides(borderSides);
      }
    },
  });

  globalTabController.focus();

  globalKeyboardHandler = (key: RawKeyEvent) => {
    if (globalTabController?.getCurrentTab().title === "Controls") {
      if (key.name === "t") borderSides.top = !borderSides.top;
      else if (key.name === "r") borderSides.right = !borderSides.right;
      else if (key.name === "b") borderSides.bottom = !borderSides.bottom;
      else if (key.name === "l") borderSides.left = !borderSides.left;
    }
  };

  renderer.keyInput.on("keypress", globalKeyboardHandler);
}

export function destroy(renderer: CliRenderer): void {
  renderer.clearFrameCallbacks();

  if (globalKeyboardHandler) {
    renderer.keyInput.off("keypress", globalKeyboardHandler);
    globalKeyboardHandler = null;
  }

  if (globalTabController) {
    globalScreen?.body.remove(globalTabController as unknown as Box);
    globalTabController = null;
  }

  if (globalScreen) {
    globalScreen.destroy();
    globalScreen = null;
  }

  colorDisplayText = null;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
