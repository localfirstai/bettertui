import {
  Box,
  type CliRenderer,
  Input,
  InputEvents,
  RenderableEvents,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { Text } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

let nameInput: Input | null = null;
let emailInput: Input | null = null;
let passwordInput: Input | null = null;
let commentInput: Input | null = null;
let renderer: CliRenderer | null = null;
let keyboardHandler:
  | ((key: {
      name?: string;
      sequence?: string;
      ctrl?: boolean;
      meta?: boolean;
      shift?: boolean;
    }) => void)
  | null = null;
let keyLegendDisplay: Text | null = null;
let statusDisplay: Text | null = null;
let lastActionText = "Welcome to Input demo! Use Tab to navigate between fields.";
let lastActionColor = "#FFCC00";
let activeInputIndex = 0;

const inputElements: Input[] = [];

function getActiveInput(): Input | null {
  return inputElements[activeInputIndex] || null;
}

function updateDisplays() {
  if (inputElements.length === 0) return;

  const activeInput = getActiveInput();
  const activeInputName = getInputName(activeInput);

  const keyLegendText = t`${bold(fg("#FFFFFF")("Key Controls:"))}
Tab/Shift+Tab: Navigate between inputs
Left/Right: Move cursor within input
Home/End: Move to start/end of input
Backspace/Delete: Remove characters
Enter: Submit current input
Ctrl+F: Toggle focus on active input
Ctrl+C: Clear active input
Ctrl+R: Reset all inputs to defaults
Type: Enter text in focused field`;

  if (keyLegendDisplay) {
    keyLegendDisplay.content = keyLegendText;
  }

  const nameValue = nameInput?.value || "";
  const emailValue = emailInput?.value || "";
  const passwordValue = passwordInput?.value || "";
  const commentValue = commentInput?.value || "";

  const nameStatus = nameInput?.focused ? "FOCUSED" : "BLURRED";
  const nameColor = nameInput?.focused ? "#00FF00" : "#FF0000";

  const emailStatus = emailInput?.focused ? "FOCUSED" : "BLURRED";
  const emailColor = emailInput?.focused ? "#00FF00" : "#FF0000";

  const passwordStatus = passwordInput?.focused ? "FOCUSED" : "BLURRED";
  const passwordColor = passwordInput?.focused ? "#00FF00" : "#FF0000";

  const commentStatus = commentInput?.focused ? "FOCUSED" : "BLURRED";
  const commentColor = commentInput?.focused ? "#00FF00" : "#FF0000";

  const statusText = t`${bold(fg("#FFFFFF")("Input Values:"))}
Name: "${nameValue}" (${fg(nameColor)(nameStatus)})
Email: "${emailValue}" (${fg(emailColor)(emailStatus)})
Password: "${passwordValue.replace(/./g, "*")}" (${fg(passwordColor)(passwordStatus)})
Comment: "${commentValue}" (${fg(commentColor)(commentStatus)})

${bold(fg("#FFAA00")(`Active Input: ${activeInputName}`))}

${bold(fg("#CCCCCC")("Validation:"))}
Name: ${validateName(nameValue) ? fg("#00FF00")("✓ Valid") : fg("#FF0000")("✗ Invalid (min 2 chars)")}
Email: ${validateEmail(emailValue) ? fg("#00FF00")("✓ Valid") : fg("#FF0000")("✗ Invalid format")}
Password: ${validatePassword(passwordValue) ? fg("#00FF00")("✓ Valid") : fg("#FF0000")("✗ Invalid (min 6 chars)")}

${fg(lastActionColor)(lastActionText)}`;

  if (statusDisplay) {
    statusDisplay.content = statusText;
  }
}

function getInputName(input: Input | null): string {
  if (input === nameInput) return "Name";
  if (input === emailInput) return "Email";
  if (input === passwordInput) return "Password";
  if (input === commentInput) return "Comment";
  return "Unknown";
}

function validateName(value: string): boolean {
  return value.length >= 2;
}

function validateEmail(value: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(value);
}

function validatePassword(value: string): boolean {
  return value.length >= 6;
}

function navigateToInput(index: number): void {
  const currentActive = getActiveInput();
  currentActive?.blur();

  activeInputIndex = Math.max(0, Math.min(index, inputElements.length - 1));
  const newActive = getActiveInput();
  newActive?.focus();

  lastActionText = `Switched to ${getInputName(newActive)} input`;
  lastActionColor = "#FFCC00";
  updateDisplays();
}

function resetInputs(): void {
  if (nameInput) nameInput.value = "";
  if (emailInput) emailInput.value = "";
  if (passwordInput) passwordInput.value = "";
  if (commentInput) commentInput.value = "";

  lastActionText = "All inputs reset to empty values";
  lastActionColor = "#FF00FF";
  updateDisplays();

  setTimeout(() => {
    lastActionColor = "#FFCC00";
    updateDisplays();
  }, 1000);
}

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#001122");

  const parentContainer = new Box(renderer, {
    id: "parent-container",
    zIndex: 10,
  });
  renderer.root.add(parentContainer);

  // Create input elements
  nameInput = new Input(renderer, {
    id: "name-input",
    position: "absolute",
    left: 5,
    top: 2,
    width: 40,
    height: 3,
    zIndex: 100,
    backgroundColor: "#001122",
    textColor: "#FFFFFF",
    placeholder: "Enter your name...",
    placeholderColor: "#666666",
    cursorColor: "#FFFF00",
    value: "",
    maxLength: 50,
  });

  emailInput = new Input(renderer, {
    id: "email-input",
    position: "absolute",
    left: 5,
    top: 6,
    width: 40,
    height: 3,
    zIndex: 100,
    backgroundColor: "#001122",
    textColor: "#FFFFFF",
    placeholder: "Enter your email...",
    placeholderColor: "#666666",
    cursorColor: "#FFFF00",
    value: "",
    maxLength: 100,
  });

  passwordInput = new Input(renderer, {
    id: "password-input",
    position: "absolute",
    left: 5,
    top: 10,
    width: 40,
    height: 3,
    zIndex: 100,
    backgroundColor: "#001122",
    textColor: "#FFFFFF",
    placeholder: "Enter password...",
    placeholderColor: "#666666",
    cursorColor: "#FFFF00",
    value: "",
    maxLength: 50,
  });

  commentInput = new Input(renderer, {
    id: "comment-input",
    position: "absolute",
    left: 5,
    top: 14,
    width: 60,
    height: 3,
    zIndex: 100,
    backgroundColor: "#001122",
    textColor: "#FFFFFF",
    placeholder: "Enter a comment...",
    placeholderColor: "#666666",
    cursorColor: "#FFFF00",
    value: "",
    maxLength: 200,
  });

  inputElements.push(nameInput, emailInput, passwordInput, commentInput);

  renderer.root.add(nameInput);
  renderer.root.add(emailInput);
  renderer.root.add(passwordInput);
  renderer.root.add(commentInput);

  keyLegendDisplay = new Text(renderer, {
    id: "key-legend",
    content: t``,
    width: 50,
    height: 12,
    position: "absolute",
    left: 50,
    top: 2,
    zIndex: 50,
    fg: "#AAAAAA",
  });
  parentContainer.add(keyLegendDisplay);

  statusDisplay = new Text(renderer, {
    id: "status-display",
    content: t``,
    width: 80,
    height: 18,
    position: "absolute",
    left: 5,
    top: 19,
    zIndex: 50,
  });
  parentContainer.add(statusDisplay);

  // Set up event handlers for all inputs
  inputElements.forEach((input, _index) => {
    input.on(InputEvents.INPUT, (value: string) => {
      lastActionText = `${getInputName(input)} input: "${value}"`;
      lastActionColor = "#00FFFF";
      updateDisplays();
    });

    input.on(InputEvents.CHANGE, (value: string) => {
      lastActionText = `*** ${getInputName(input)} CHANGED: "${value}" ***`;
      lastActionColor = "#FF00FF";
      updateDisplays();
      setTimeout(() => {
        lastActionColor = "#FFCC00";
        updateDisplays();
      }, 1000);
    });

    input.on(InputEvents.ENTER, (value: string) => {
      const inputName = getInputName(input);
      const isValid =
        inputName === "Name"
          ? validateName(value)
          : inputName === "Email"
            ? validateEmail(value)
            : inputName === "Password"
              ? validatePassword(value)
              : true;

      lastActionText = `*** ${inputName} SUBMITTED: "${value}" ${isValid ? "(Valid)" : "(Invalid)"} ***`;
      lastActionColor = isValid ? "#00FF00" : "#FF0000";
      updateDisplays();
      setTimeout(() => {
        lastActionColor = "#FFCC00";
        updateDisplays();
      }, 1500);
    });

    input.on(RenderableEvents.FOCUSED, () => {
      updateDisplays();
    });

    input.on(RenderableEvents.BLURRED, () => {
      updateDisplays();
    });
  });

  updateDisplays();

  keyboardHandler = (key) => {
    const _anyInputFocused = inputElements.some((input) => input.focused);

    if (key.name === "tab") {
      if (key.shift) {
        // Navigate backward
        navigateToInput(activeInputIndex - 1);
      } else {
        // Navigate forward
        navigateToInput(activeInputIndex + 1);
      }
    } else if (key.ctrl && key.name === "f") {
      // Only respond to Ctrl+F for focus toggle
      const activeInput = getActiveInput();
      if (activeInput?.focused) {
        activeInput.blur();
        lastActionText = `Focus removed from ${getInputName(activeInput)} input`;
      } else {
        activeInput?.focus();
        lastActionText = `${getInputName(activeInput)} input focused`;
      }
      lastActionColor = "#FFCC00";
      updateDisplays();
    } else if (key.ctrl && key.name === "c") {
      // Only respond to Ctrl+C for clear
      const activeInput = getActiveInput();
      if (activeInput) {
        activeInput.value = "";
        lastActionText = `${getInputName(activeInput)} input cleared`;
        lastActionColor = "#FFAA00";
        updateDisplays();
      }
    } else if (key.ctrl && key.name === "r") {
      // Only respond to Ctrl+R for reset
      resetInputs();
    }
  };

  rendererInstance.keyInput.on("keypress", keyboardHandler);
  nameInput.focus();
}

export function destroy(rendererInstance: CliRenderer): void {
  if (keyboardHandler) {
    rendererInstance.keyInput.off("keypress", keyboardHandler);
    keyboardHandler = null;
  }

  for (const input of inputElements) {
    if (input) {
      rendererInstance.root.remove(input);
      input.destroy();
    }
  }
  inputElements.length = 0;

  const parentContainer = rendererInstance.root.getRenderable("parent-container");
  if (parentContainer) rendererInstance.root.remove(parentContainer);

  nameInput = null;
  emailInput = null;
  passwordInput = null;
  commentInput = null;
  keyLegendDisplay = null;
  statusDisplay = null;
  renderer = null;
  activeInputIndex = 0;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
  renderer.start();
}
