import {
  Box,
  type CliRenderer,
  ScrollBox,
  Text,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let rootContainer: Box | null = null;
let scrollBox: ScrollBox | null = null;
let renderer: CliRenderer | null = null;
let itemCount = 0;

function makeRow(r: CliRenderer, index: number): Box {
  const isEven = index % 2 === 0;
  const row = new Box(r, {
    flexDirection: "row",
    gap: 2,
    paddingX: 1,
    paddingY: 0,
    backgroundColor: isEven ? "#24283b" : "#1a1b26",
  });

  const indexCell = new Text(r, {
    content: String(index + 1).padStart(4, " "),
    width: 4,
    fg: "#565f89",
  });
  const nameCell = new Text(r, { content: `Item ${index + 1}`, width: 18, fg: "#c0caf5" });
  const statusColors = ["#9ece6a", "#e0af68", "#f7768e"];
  const statuses = ["Active", "Pending", "Error"];
  const si = index % 3;
  const statusCell = new Text(r, {
    content: statuses[si] ?? "Active",
    width: 10,
    fg: statusColors[si] ?? "#9ece6a",
  });
  const descCell = new Text(r, { content: `Description for item ${index + 1}`, fg: "#a9b1d6" });

  row.add(indexCell);
  row.add(nameCell);
  row.add(statusCell);
  row.add(descCell);

  return row;
}

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  rootContainer = new Box(renderer, {
    id: "scrollbox-example-root",
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
      content: t`${bold(fg("#7aa2f7")("ScrollBox Component Example"))}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  // Column headers
  const colHeaders = new Box(renderer, {
    flexDirection: "row",
    gap: 2,
    paddingX: 1,
    backgroundColor: "#292e42",
    flexShrink: 0,
    height: 1,
  });
  colHeaders.add(new Text(renderer, { content: "   #", width: 4, fg: "#7aa2f7" }));
  colHeaders.add(new Text(renderer, { content: "Name", width: 18, fg: "#7aa2f7" }));
  colHeaders.add(new Text(renderer, { content: "Status", width: 10, fg: "#7aa2f7" }));
  colHeaders.add(new Text(renderer, { content: "Description", fg: "#7aa2f7" }));
  rootContainer.add(colHeaders);

  // Scrollable list
  scrollBox = new ScrollBox(renderer, {
    id: "main-scroll",
    flexGrow: 1,
    flexShrink: 1,
    flexBasis: 0,
    minHeight: 0,
    scrollY: true,
    border: true,
    borderStyle: "single",
    borderColor: "#414868",
    focusedBorderColor: "#7aa2f7",
  });

  for (let i = 0; i < 50; i++) {
    scrollBox.add(makeRow(renderer, i));
    itemCount++;
  }

  rootContainer.add(scrollBox);

  // Footer controls
  const footer = new Box(renderer, {
    flexDirection: "row",
    gap: 2,
    flexShrink: 0,
    height: 1,
    alignItems: "center",
  });

  footer.add(
    new Text(renderer, {
      content: t`${fg("#414868")("↑↓ scroll  ·  PgUp/PgDn page  ·  Home/End edges  ·  A add row  ·  Ctrl+C quit")}`,
    }),
  );
  rootContainer.add(footer);

  // Add more rows on 'a'
  rendererInstance.keyInput.on("keypress", (key) => {
    if (key.name === "a" && scrollBox && renderer) {
      scrollBox.add(makeRow(renderer, itemCount));
      itemCount++;
    }
  });

  scrollBox.focus();
}

export function destroy(rendererInstance: CliRenderer): void {
  if (rootContainer) {
    rendererInstance.root.remove(rootContainer);
    rootContainer.destroy();
    rootContainer = null;
  }
  scrollBox = null;
  renderer = null;
  itemCount = 0;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
