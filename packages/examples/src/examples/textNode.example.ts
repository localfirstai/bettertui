#!/usr/bin/env bun

import {
  Box,
  type CliRenderer,
  Text,
  bold,
  createCliRenderer,
  cyan,
  green,
  t,
  underline,
  yellow,
} from "@bettertui/core";
import { TextNode } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

let mainContainer: Box | null = null;
let demoText: Text | null = null;
let instructionsText: Text | null = null;
let statusText: Text | null = null;
let updateInterval: ReturnType<typeof setInterval> | null = null;

function clearUpdateInterval(): void {
  if (updateInterval) {
    clearInterval(updateInterval);
    updateInterval = null;
  }
}

export function run(renderer: CliRenderer): void {
  renderer.setBackgroundColor("#0d1117");

  mainContainer = new Box(renderer, {
    id: "mainContainer",
    width: 88,
    height: 32,
    backgroundColor: "#161b22",
    zIndex: 1,
    borderColor: "#50565d",
    title: "TextNode Demo",
    titleAlignment: "center",
    border: true,
  });
  renderer.root.add(mainContainer);

  // Create the main demo text area
  demoText = new Text(renderer, {
    id: "demoText",
    width: 60,
    height: 20,
    zIndex: 2,
    fg: "#f0f6fc",
  });
  mainContainer.add(demoText);

  // Create instructions
  instructionsText = new Text(renderer, {
    id: "instructions",
    content: t`${bold(cyan("TextNode Demo"))}
${yellow("•")} Press ${green("1-4")} to see different examples
${yellow("•")} Press ${green("SPACE")} to toggle dynamic updates
${yellow("•")} Press ${green("R")} to reset demo
${yellow("•")} Press ${green("ESC")} to exit

${underline("Current:")} Example 1 - Basic TextNode Creation`,
    fg: "#c9d1d9",
  });
  mainContainer.add(instructionsText);

  // Create status area
  statusText = new Text(renderer, {
    id: "status",
    content: "Ready - Press 1-4 for examples",
    width: 84,
    height: 3,
    fg: "#58a6ff",
  });
  mainContainer.add(statusText);

  // Initialize with first example
  showExample1();

  // Set up keyboard controls
  renderer.keyInput.on("keypress", (event) => {
    const key = event.sequence;
    if (key === "1") {
      showExample1();
    } else if (key === "2") {
      showExample2();
    } else if (key === "3") {
      showExample3();
    } else if (key === "4") {
      showExample4();
    } else if (key === " ") {
      toggleDynamicUpdates();
    } else if (key === "r" || key === "R") {
      resetDemo();
    }
  });
}

function showExample1(): void {
  if (!demoText) return;

  // Clear any running intervals
  clearUpdateInterval();

  // Clear existing TextNodes
  demoText.clear();

  // Example 1: Basic TextNode Creation
  const titleNode = TextNode.fromString("Basic TextNode Demo", {
    fg: "#58a6ff",
    attributes: 1, // bold
  });

  const subtitleNode = TextNode.fromString(
    "\n\nCreating individual TextNodes with different styles:",
    {
      fg: "#8b949e",
    },
  );

  const redNode = TextNode.fromString("\n\nRed Text", {
    fg: "#ff7b72",
  });

  const blueNode = TextNode.fromString(" | Blue Text", {
    fg: "#79c0ff",
  });

  const greenNode = TextNode.fromString(" | Green Text", {
    fg: "#56d364",
  });

  const yellowNode = TextNode.fromString(" | Yellow Background", {
    fg: "#000000",
    bg: "#d29922",
  });

  // Create a container node that holds all the styled nodes
  const containerNode = TextNode.fromNodes([
    titleNode,
    subtitleNode,
    redNode,
    blueNode,
    greenNode,
    yellowNode,
  ]);

  // Add to Text
  demoText.addNode(containerNode);

  updateInstructions(
    "Example 1 - Basic TextNode Creation",
    "Creating individual TextNodes with different colors and styles",
  );
}

function showExample2(): void {
  if (!demoText) return;

  // Clear any running intervals
  clearUpdateInterval();

  // Clear existing TextNodes
  demoText.clear();

  // Example 2: Nested TextNode Composition
  const titleNode = TextNode.fromString("Nested Composition Demo", {
    fg: "#58a6ff",
    attributes: 1, // bold
  });

  const introNode = TextNode.fromString("\n\nBuilding complex text by nesting TextNodes:", {
    fg: "#8b949e",
  });

  // Create nested structure
  const codeBlock = TextNode.fromString(
    "\n\nfunction calculateTotal(items) {\n  return items.reduce((sum, item) => {\n    return sum + item.price;\n  }, 0);\n}",
    {
      fg: "#f0f6fc",
      bg: "#0d1117",
    },
  );

  const commentNode = TextNode.fromString("\n\n// This is a nested comment", {
    fg: "#8b949e",
  });

  const highlightNode = TextNode.fromString(" with ", {
    fg: "#79c0ff",
    attributes: 1, // bold
  });

  const highlightNode2 = TextNode.fromString("highlighting", {
    fg: "#ff7b72",
    attributes: 4, // underline
  });

  // Create a sentence that combines multiple styled parts
  const sentenceNode = TextNode.fromNodes([
    TextNode.fromString("\n\nThis demonstrates ", { fg: "#c9d1d9" }),
    highlightNode,
    TextNode.fromString("and ", { fg: "#c9d1d9" }),
    highlightNode2,
    TextNode.fromString(" within the same text flow.", {
      fg: "#c9d1d9",
    }),
  ]);

  // Create the main container
  const containerNode = TextNode.fromNodes([
    titleNode,
    introNode,
    codeBlock,
    commentNode,
    sentenceNode,
  ]);

  demoText.addNode(containerNode);

  updateInstructions(
    "Example 2 - Nested TextNode Composition",
    "Building complex text structures by composing TextNodes together",
  );
}

function showExample3(): void {
  if (!demoText) return;

  // Clear any existing intervals before setting up new ones
  clearUpdateInterval();

  // Clear existing TextNodes
  demoText.clear();

  // Example 3: Dynamic TextNode Updates
  const titleNode = TextNode.fromString("Dynamic Updates Demo", {
    fg: "#58a6ff",
    attributes: 1, // bold
  });

  const introNode = TextNode.fromString("\n\nTextNodes can be updated dynamically:", {
    fg: "#8b949e",
  });

  const counterNode = TextNode.fromString("\n\nCounter: 0", {
    fg: "#56d364",
    attributes: 1, // bold
  });

  const statusNode = TextNode.fromString("\n\nStatus: Idle", {
    fg: "#79c0ff",
  });

  const progressNode = TextNode.fromString("\n\nProgress: [          ]", {
    fg: "#d29922",
  });

  // Store references to nodes that will be updated
  const containerNode = TextNode.fromNodes([
    titleNode,
    introNode,
    counterNode,
    statusNode,
    progressNode,
  ]);

  demoText.addNode(containerNode);

  // Set up dynamic updates for this example
  let example3Counter = 0;
  const maxCount = 20;

  updateInterval = setInterval(() => {
    if (!demoText || !containerNode) return;

    example3Counter++;
    if (example3Counter > maxCount) {
      example3Counter = 0;
    }

    // Update counter node
    counterNode.children = [`\n\nCounter: ${example3Counter}`];

    // Update status based on counter
    const status =
      example3Counter < 5 ? "Starting" : example3Counter < 15 ? "Running" : "Finishing";
    statusNode.children = [`\n\nStatus: ${status}`];

    // Update progress bar
    const progress = Math.floor((example3Counter / maxCount) * 10);
    const progressBar = "█".repeat(progress).padEnd(10, "░");
    progressNode.children = [`\n\nProgress: [${progressBar}]`];

    // No manual refresh needed — the lifecycle pass auto-syncs dirty nodes
  }, 100);

  updateInstructions(
    "Example 3 - Dynamic TextNode Updates",
    "TextNodes can be modified and the changes reflected in real-time",
  );
}

function showExample4(): void {
  if (!demoText) return;

  // Clear any running intervals
  clearUpdateInterval();

  // Clear existing TextNodes
  demoText.clear();

  // Example 4: Complex Document Structure
  const titleNode = TextNode.fromString("Complex Document Demo", {
    fg: "#58a6ff",
    attributes: 1, // bold
  });

  const introNode = TextNode.fromString("\n\nBuilding a complete document with TextNodes:", {
    fg: "#8b949e",
  });

  // Document sections
  const headerNode = TextNode.fromString("\n\n📋 Project Status Report", {
    fg: "#ffffff",
    attributes: 1, // bold
  });

  const section1Node = TextNode.fromNodes([
    TextNode.fromString("\n\n🚀 ", { fg: "#56d364" }),
    TextNode.fromString("Progress", { fg: "#58a6ff", attributes: 1 }),
    TextNode.fromString(": 85% complete", { fg: "#c9d1d9" }),
  ]);

  const section2Node = TextNode.fromNodes([
    TextNode.fromString("\n\n⚠️  ", { fg: "#d29922" }),
    TextNode.fromString("Issues", { fg: "#ff7b72", attributes: 1 }),
    TextNode.fromString(": 2 minor issues found", { fg: "#c9d1d9" }),
  ]);

  const section3Node = TextNode.fromNodes([
    TextNode.fromString("\n\n✅ ", { fg: "#56d364" }),
    TextNode.fromString("Next Steps", {
      fg: "#58a6ff",
      attributes: 1,
    }),
    TextNode.fromString(": Code review and testing", {
      fg: "#c9d1d9",
    }),
  ]);

  const footerNode = TextNode.fromString("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━", {
    fg: "#30363d",
  });

  const signatureNode = TextNode.fromString("\nGenerated by BetterTUI TextNode Demo", {
    fg: "#8b949e",
    attributes: 2, // italic
  });

  // Combine all sections into the final document
  const documentNode = TextNode.fromNodes([
    titleNode,
    introNode,
    headerNode,
    section1Node,
    section2Node,
    section3Node,
    footerNode,
    signatureNode,
  ]);

  demoText.addNode(documentNode);

  updateInstructions(
    "Example 4 - Complex Document Structure",
    "Creating complete documents by composing multiple styled TextNode sections",
  );
}

function toggleDynamicUpdates(): void {
  if (updateInterval) {
    clearUpdateInterval();
    updateStatus("Dynamic updates stopped");
  } else {
    // Restart Example 3 if we're not already on it
    showExample3();
    updateStatus("Dynamic updates started");
  }
}

function resetDemo(): void {
  clearUpdateInterval();
  showExample1();
  updateStatus("Demo reset");
}

function updateInstructions(title: string, description: string): void {
  if (!instructionsText) return;

  instructionsText.content = t`${bold(cyan("TextNode Demo"))}
${yellow("•")} Press ${green("1-4")} to see different examples
${yellow("•")} Press ${green("SPACE")} to toggle dynamic updates
${yellow("•")} Press ${green("R")} to reset demo
${yellow("•")} Press ${green("ESC")} to exit

${underline("Current:")} ${title}
${description}`;
}

function updateStatus(message: string): void {
  if (!statusText) return;
  statusText.content = message;
}

export function destroy(_renderer: CliRenderer): void {
  clearUpdateInterval();

  mainContainer?.destroyRecursively();
  mainContainer = null;
  demoText = null;
  instructionsText = null;
  statusText = null;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    targetFps: 30,
    enableMouseMovement: true,
    exitOnCtrlC: true,
  });
  run(renderer);
  setupCommonDemoKeys(renderer);
  // renderer.start()
}
