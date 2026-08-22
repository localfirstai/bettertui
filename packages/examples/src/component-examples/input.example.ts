import {
  Box,
  type CliRenderer,
  Input,
  InputEvents,
  RenderableEvents,
  Text,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let rootContainer: Box | null = null;
let renderer: CliRenderer | null = null;
let _nameInput: Input | null = null;
let _emailInput: Input | null = null;
let _passwordInput: Input | null = null;
let statusText: Text | null = null;
let activeIndex = 0;
const inputs: Input[] = [];

function getLabel(idx: number): string {
  return ["Name", "Email", "Password"][idx] ?? "Unknown";
}

function updateStatus(): void {
  if (!statusText) return;

  const lines = inputs.map((inp, i) => {
    const _focusColor = inp.focused ? "#9ece6a" : "#565f89";
    const focusLabel = inp.focused ? "FOCUSED" : "blurred";
    const raw = inp.value;
    const display = i === 2 ? raw.replace(/./g, "*") : raw;
    const valueStr = display || "(empty)";
    return `${getLabel(i)}: ${valueStr} [${focusLabel}]`;
  });

  statusText.content = t`${bold(fg("#a9b1d6")(lines[0] ?? ""))}
${bold(fg("#a9b1d6")(lines[1] ?? ""))}
${bold(fg("#a9b1d6")(lines[2] ?? ""))}

${fg("#565f89")("Tab / Shift+Tab to navigate  ·  Ctrl+C to quit")}`;
}

function focusInput(idx: number): void {
  inputs[activeIndex]?.blur();
  activeIndex = Math.max(0, Math.min(idx, inputs.length - 1));
  inputs[activeIndex]?.focus();
  updateStatus();
}

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  rootContainer = new Box(renderer, {
    id: "input-example-root",
    flexDirection: "column",
    width: "100%",
    height: "100%",
    padding: 2,
    gap: 1,
  });
  renderer.root.add(rootContainer);

  // Header
  rootContainer.add(
    new Text(renderer, {
      content: t`${bold(fg("#7aa2f7")("Input Component Example"))}`,
      height: 1,
      flexShrink: 0,
    }),
  );
  rootContainer.add(
    new Text(renderer, {
      content: "─".repeat(60),
      fg: "#414868",
      height: 1,
      flexShrink: 0,
    }),
  );

  // Form fields
  const fieldDefs: Array<{
    label: string;
    placeholder: string;
    password?: boolean;
  }> = [
    { label: "Name", placeholder: "Enter your full name…" },
    { label: "Email", placeholder: "user@example.com" },
    { label: "Password", placeholder: "At least 8 characters…", password: true },
  ];

  for (const { label, placeholder, password } of fieldDefs) {
    const row = new Box(renderer, {
      flexDirection: "row",
      gap: 1,
      alignItems: "center",
      flexShrink: 0,
    });

    row.add(
      new Text(renderer, {
        content: `${label}:`,
        width: 10,
        fg: "#a9b1d6",
      }),
    );

    const field = new Input(renderer, {
      width: "100%",
      height: 1,
      placeholder,
      password: password ?? false,
      placeholderColor: "#414868",
      textColor: "#c0caf5",
      focusedTextColor: "#ffffff",
      cursorColor: "#7aa2f7",
      backgroundColor: "transparent",
      focusedBackgroundColor: "transparent",
    });

    // Wrap in a bordered Box so the border surrounds the input properly.
    // Input itself stays border-free; the Box provides the visual frame.
    const fieldWrapper = new Box(renderer, {
      flexGrow: 1,
      maxWidth: 40,
      border: true,
      borderStyle: "single",
      borderColor: "#414868",
      focusedBorderColor: "#7aa2f7",
    });
    fieldWrapper.add(field);

    field.on(InputEvents.INPUT, () => updateStatus());
    field.on(RenderableEvents.FOCUSED, () => {
      fieldWrapper.borderColor = "#7aa2f7";
      updateStatus();
    });
    field.on(RenderableEvents.BLURRED, () => {
      fieldWrapper.borderColor = "#414868";
      updateStatus();
    });
    field.on(InputEvents.ENTER, () => focusInput(activeIndex + 1));

    row.add(fieldWrapper);
    rootContainer.add(row);
    inputs.push(field);
  }

  [_nameInput, _emailInput, _passwordInput] = inputs as [Input, Input, Input];

  // Status display
  rootContainer.add(
    new Text(renderer, { content: "─".repeat(60), fg: "#414868", height: 1, flexShrink: 0 }),
  );

  statusText = new Text(renderer, {
    width: 70,
    wrapMode: "word",
    fg: "#a9b1d6",
    flexShrink: 0,
  });
  rootContainer.add(statusText);

  // Tab navigation
  rendererInstance.keyInput.on("keypress", (key) => {
    if (key.name === "tab") {
      key.preventDefault?.();
      if (key.shift) {
        focusInput(activeIndex - 1);
      } else {
        focusInput(activeIndex + 1);
      }
    }
  });

  updateStatus();
  focusInput(0);
}

export function destroy(rendererInstance: CliRenderer): void {
  for (const inp of inputs) inp.destroy();
  inputs.length = 0;

  if (rootContainer) {
    rendererInstance.root.remove(rootContainer);
    rootContainer.destroy();
    rootContainer = null;
  }

  _nameInput = null;
  _emailInput = null;
  _passwordInput = null;
  statusText = null;
  renderer = null;
  activeIndex = 0;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
