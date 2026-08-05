#!/usr/bin/env bun
import { Box, type CliRenderer, Text, bold, createCliRenderer, fg, t } from "@bettertui/core";
import { ScrollBox } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let scrollBox: ScrollBox | null = null;
let statusText: Text | null = null;
let hoveredItem: string | null = null;

export function run(renderer: CliRenderer): void {
  renderer.setBackgroundColor("#1a1b26");

  const mainContainer = new Box(renderer, {
    id: "main-container",
    flexGrow: 1,
    maxHeight: "100%",
    maxWidth: "100%",
    flexDirection: "column",
    backgroundColor: "#1a1b26",
  });

  const header = new Box(renderer, {
    id: "header",
    width: "100%",
    height: 3,
    backgroundColor: "#24283b",
    paddingLeft: 1,
    flexShrink: 0,
  });

  const title = new Text(renderer, {
    content: t`${bold(fg("#7aa2f7")("ScrollBox Mouse Hit Test"))} - Scroll and hover items to test hit detection`,
  });
  header.add(title);

  statusText = new Text(renderer, {
    content: t`${fg("#565f89")("Hovered:")} ${fg("#c0caf5")("none")}`,
  });
  header.add(statusText);

  scrollBox = new ScrollBox(renderer, {
    id: "scroll-box",
    rootOptions: {
      backgroundColor: "#24283b",
      border: true,
    },
    contentOptions: {
      backgroundColor: "#16161e",
    },
  });

  for (let i = 0; i < 50; i++) {
    const item = new Box(renderer, {
      id: `item-${i}`,
      width: "100%",
      height: 2,
      backgroundColor: i % 2 === 0 ? "#292e42" : "#2f3449",
      paddingLeft: 1,
      onMouseOver: () => {
        hoveredItem = `item-${i}`;
        updateStatus();
      },
      onMouseOut: () => {
        if (hoveredItem === `item-${i}`) {
          hoveredItem = null;
          updateStatus();
        }
      },
      onClick: () => {
        console.log(`Clicked item-${i}`);
      },
    });

    const text = new Text(renderer, {
      content: t`${fg("#7aa2f7")(`[${i.toString().padStart(2, "0")}]`)} ${fg("#c0caf5")(`Item ${i} - Hover over me to test hit detection`)}`,
    });
    item.add(text);
    scrollBox.add(item);
  }

  mainContainer.add(header);
  mainContainer.add(scrollBox);
  renderer.root.add(mainContainer);

  scrollBox.focus();

  function updateStatus() {
    if (statusText) {
      const hovered = hoveredItem || "none";
      statusText.content = t`${fg("#565f89")("Hovered:")} ${fg("#9ece6a")(hovered)}`;
    }
  }
}

export function destroy(renderer: CliRenderer): void {
  for (const child of renderer.root.getChildren()) {
    renderer.root.remove(child);
    child.destroyRecursively();
  }
  scrollBox = null;
  statusText = null;
  hoveredItem = null;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
