import { Box, type CliRenderer, Text, bold, createCliRenderer, fg, t } from "@bettertui/core";
import { ScrollBox } from "@bettertui/core";
import { mixColor } from "../lib/colorUtils";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let scrollBox: ScrollBox | null = null;
let renderer: CliRenderer | null = null;
let mainContainer: Box | null = null;
let instructionsBox: Box | null = null;
let itemCount = 0;
let animationInterval: ReturnType<typeof setInterval> | null = null;

// Track items with their creation time for animation
interface AnimatedItem {
  box: Box;
  text: Text;
  createdAt: number;
  originalContent: string;
  normalBgColor: string;
  isAtTop: boolean;
}

const animatedItems = new Map<string, AnimatedItem>();

// Clear all items from the scroll box
function clearAllItems() {
  if (!scrollBox) return;

  // Stop any running animations
  if (animationInterval) {
    clearInterval(animationInterval);
    animationInterval = null;
  }

  // Clear animated items tracking
  animatedItems.clear();

  // Remove all children from scroll box
  const children = scrollBox.getChildren();
  for (const child of children) {
    scrollBox.remove(child);
    child.destroyRecursively();
  }

  // Reset item count
  itemCount = 0;
}

// Animation update function
function updateAnimations() {
  const now = Date.now();

  for (const [id, item] of animatedItems) {
    const age = now - item.createdAt;
    const duration = 800; // 800ms animation

    if (age >= duration) {
      // Animation complete, set to final colors
      item.box.backgroundColor = item.normalBgColor;

      // Set final content with normal colors
      const itemNumber = id.split("-")[1];
      const timeString = new Date(item.createdAt).toLocaleTimeString();
      const finalContent = t`${bold(fg("#7aa2f7")(`Item #${itemNumber}`))}
${fg("#9aa5ce")("This is a dynamically added item with enhanced content.")}
${fg("#c0caf5")("Contains additional information and styling.")}
${fg("#565f89")("Added at:")} ${timeString}
${fg("#565f89")("Position:")} ${item.isAtTop ? "TOP" : "BOTTOM"}
${fg("#565f89")("Status:")} ${fg("#9ece6a")("ACTIVE")}`;

      item.text.content = finalContent;
      animatedItems.delete(id);
      continue;
    }

    const progress = age / duration;
    const easeProgress = 1 - (1 - progress) ** 3; // Ease out cubic

    // Interpolate background color (bright to normal)
    const brightBg = "#4c4f69";
    item.box.backgroundColor = mixColor(brightBg, item.normalBgColor, easeProgress);

    // Update text with flashy purple fading to normal blue
    const itemNumber = id.split("-")[1];
    const currentTitleColor = mixColor("#bb9af7", "#7aa2f7", easeProgress);
    const timeString = new Date(item.createdAt).toLocaleTimeString();

    // Reconstruct content with interpolated color
    const updatedContent = t`${bold(fg(currentTitleColor)(`Item #${itemNumber}`))}
${fg("#9aa5ce")("This is a dynamically added item with enhanced content.")}
${fg("#c0caf5")("Contains additional information and styling.")}
${fg("#565f89")("Added at:")} ${timeString}
${fg("#565f89")("Position:")} ${item.isAtTop ? "TOP" : "BOTTOM"}
${fg("#565f89")("Status:")} ${fg("#9ece6a")("ACTIVE")}`;

    item.text.content = updatedContent;
  }

  // Stop animation loop if no items are animating
  if (animatedItems.size === 0 && animationInterval) {
    clearInterval(animationInterval);
    animationInterval = null;
  }
}

function addItem(atTop = false) {
  if (!renderer || !scrollBox) return;

  itemCount++;

  const boxId = `item-${itemCount}`;
  const normalBgColor = itemCount % 2 === 0 ? "#24283b" : "#1f2335";

  // Start with flashy colors
  const box = new Box(renderer, {
    id: boxId,
    width: "auto",
    padding: 1,
    marginBottom: 1,
    backgroundColor: "#4c4f69", // Bright initial background
  });

  const timeString = new Date().toLocaleTimeString();

  // Store original content as string (without template literal processing)
  const originalContent = `Item #${itemCount}\nThis is a dynamically added item with enhanced content.\nContains additional information and styling.\nAdded at: ${timeString}\nPosition: ${atTop ? "TOP" : "BOTTOM"}\nStatus: ACTIVE`;

  // Start with flashy purple title using template literal
  const flashyContent = t`${bold(fg("#bb9af7")(`Item #${itemCount}`))}
${fg("#9aa5ce")("This is a dynamically added item with enhanced content.")}
${fg("#c0caf5")("Contains additional information and styling.")}
${fg("#565f89")("Added at:")} ${timeString}
${fg("#565f89")("Position:")} ${atTop ? "TOP" : "BOTTOM"}
${fg("#565f89")("Status:")} ${fg("#9ece6a")("ACTIVE")}`;

  const text = new Text(renderer, {
    content: flashyContent,
  });

  box.add(text);

  if (atTop) {
    scrollBox.add(box, 0); // Add at the beginning
  } else {
    scrollBox.add(box); // Add at the end
  }

  // Track for animation
  animatedItems.set(boxId, {
    box,
    text,
    createdAt: Date.now(),
    originalContent,
    normalBgColor,
    isAtTop: atTop,
  });

  // Start animation loop if not running
  if (!animationInterval) {
    animationInterval = setInterval(updateAnimations, 16); // ~60fps
  }
}

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#0a0a14");

  mainContainer = new Box(renderer, {
    id: "main-container",
    flexGrow: 1,
    maxHeight: "100%",
    maxWidth: "100%",
    flexDirection: "column",
    backgroundColor: "#0f0f23",
  });

  scrollBox = new ScrollBox(renderer, {
    id: "sticky-scroll-box",
    stickyScroll: true,
    stickyStart: "bottom",
    rootOptions: {
      backgroundColor: "#1e1e2e",
      border: true,
    },
    wrapperOptions: {
      backgroundColor: "#181825",
    },
    viewportOptions: {
      backgroundColor: "#11111b",
    },
    contentOptions: {
      backgroundColor: "#0f0f0f",
    },
    scrollbarOptions: {
      // width: 2,
      // showArrows: true,
      trackOptions: {
        foregroundColor: "#7aa2f7",
        backgroundColor: "#313244",
      },
    },
  });

  instructionsBox = new Box(renderer, {
    id: "instructions",
    width: "100%",
    flexDirection: "column",
    backgroundColor: "#1e1e2e",
    paddingLeft: 1,
    flexShrink: 0,
  });

  const instructionsText1 = new Text(renderer, {
    content: t`${bold(fg("#7aa2f7")("Sticky Scroll Demo"))} ${fg("#565f89")("-")} ${bold(fg("#9ece6a")("S"))} ${fg("#c0caf5")("Toggle sticky scroll")} ${fg("#565f89")("|")} ${bold(fg("#bb9af7")("T"))} ${fg("#c0caf5")("Add item at top")} ${fg("#565f89")("|")} ${bold(fg("#f7768e")("B"))} ${fg("#c0caf5")("Add item at bottom")} ${fg("#565f89")("|")} ${bold(fg("#e0af68")("E"))} ${fg("#c0caf5")("Clear all items")}`,
  });

  const instructionsText2 = new Text(renderer, {
    content: t`${bold(fg("#7aa2f7")("Behavior:"))} ${fg("#c0caf5")("Scroll to top/bottom, then add items to see sticky behavior")}`,
  });

  const instructionsText3 = new Text(renderer, {
    content: t`${bold(fg("#7aa2f7")("Status:"))} ${fg("#c0caf5")("Sticky Scroll:")} ${scrollBox.stickyScroll ? fg("#9ece6a")("ENABLED") : fg("#f7768e")("DISABLED")}`,
  });

  instructionsBox.add(instructionsText1);
  instructionsBox.add(instructionsText2);
  instructionsBox.add(instructionsText3);

  mainContainer.add(scrollBox);
  mainContainer.add(instructionsBox);

  renderer.root.add(mainContainer);

  scrollBox.focus();

  // Add some initial items
  for (let i = 0; i < 10; i++) {
    addItem(false);
  }

  rendererInstance.keyInput.on("keypress", (key) => {
    if (key.name === "s" && scrollBox) {
      // Toggle sticky scroll
      const currentSticky = scrollBox.stickyScroll;
      scrollBox.stickyScroll = !currentSticky;
      console.log(`Sticky scroll ${!currentSticky ? "enabled" : "disabled"}`);

      // Update status display
      if (instructionsBox && instructionsBox.getChildren().length >= 3) {
        const statusText = instructionsBox.getChildren()[2] as Text;
        statusText.content = t`${bold(fg("#7aa2f7")("Status:"))} ${fg("#c0caf5")("Sticky Scroll:")} ${scrollBox.stickyScroll ? fg("#9ece6a")("ENABLED") : fg("#f7768e")("DISABLED")}`;
      }
    } else if (key.name === "t" && scrollBox) {
      addItem(true); // Add at top
      console.log("Added item at top");
    } else if (key.name === "b" && scrollBox) {
      addItem(false); // Add at bottom
      console.log("Added item at bottom");
    } else if (key.name === "e" && scrollBox) {
      clearAllItems(); // Clear all items
    }
  });
}

export function destroy(rendererInstance: CliRenderer): void {
  if (mainContainer) {
    rendererInstance.root.remove(mainContainer);
    mainContainer.destroyRecursively();
    mainContainer = null;
  }

  // Clean up animation
  if (animationInterval) {
    clearInterval(animationInterval);
    animationInterval = null;
  }
  animatedItems.clear();

  renderer = null;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
