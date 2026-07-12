import {
  Badge,
  Box,
  Flex,
  Heading,
  Separator,
  StatusLine,
  Text,
  render,
  useKeyboard,
  useRuntime,
} from "@bettertui/react";

interface Capability {
  name: string;
  value: string;
  supported: boolean;
  category: string;
}

function detectTerminalInfo(): Capability[] {
  const termProgram = process.env.TERM_PROGRAM || "Unknown";
  const term = process.env.TERM || "Unknown";
  const shell = process.env.SHELL || "Unknown";
  const termVersion = process.env.TERM_VERSION || "";

  return [
    {
      name: "Terminal Program",
      value: termProgram,
      supported: termProgram !== "Unknown",
      category: "Terminal Info",
    },
    { name: "TERM", value: term, supported: term !== "Unknown", category: "Terminal Info" },
    { name: "Shell", value: shell, supported: shell !== "Unknown", category: "Terminal Info" },
    {
      name: "TERM_VERSION",
      value: termVersion || "Not set",
      supported: !!termVersion,
      category: "Terminal Info",
    },
  ];
}

function detectDisplayCapabilities(): Capability[] {
  const colorterm = process.env.COLORTERM || "";
  const truecolor = colorterm.toLowerCase() === "truecolor";

  return [
    {
      name: "Color Depth",
      value: truecolor ? "True Color" : "Unknown",
      supported: truecolor,
      category: "Display",
    },
    {
      name: "Unicode",
      value: "Unknown",
      supported: false,
      category: "Display",
    },
    {
      name: "Emoji",
      value: "Unknown",
      supported: false,
      category: "Display",
    },
    {
      name: "Nerd Font",
      value: "Unknown",
      supported: false,
      category: "Display",
    },
  ];
}

function detectInputCapabilities(): Capability[] {
  return [
    {
      name: "Keyboard Protocol",
      value: "Unknown",
      supported: false,
      category: "Input",
    },
    {
      name: "Mouse Support",
      value: "Unknown",
      supported: false,
      category: "Input",
    },
    {
      name: "Bracketed Paste",
      value: "Unknown",
      supported: false,
      category: "Input",
    },
    {
      name: "Focus Events",
      value: "Unknown",
      supported: false,
      category: "Input",
    },
  ];
}

function detectRenderingCapabilities(): Capability[] {
  return [
    {
      name: "Synchronized Output",
      value: "Unknown",
      supported: false,
      category: "Rendering",
    },
    {
      name: "Cursor Style",
      value: "Unknown",
      supported: false,
      category: "Rendering",
    },
    {
      name: "Alternate Screen",
      value: "Unknown",
      supported: false,
      category: "Rendering",
    },
    {
      name: "OSC 8 (Links)",
      value: "Unknown",
      supported: false,
      category: "Rendering",
    },
  ];
}

function detectGraphicsCapabilities(): Capability[] {
  return [
    {
      name: "Kitty Graphics",
      value: "Unknown",
      supported: false,
      category: "Graphics",
    },
    {
      name: "Sixel",
      value: "Unknown",
      supported: false,
      category: "Graphics",
    },
    {
      name: "iTerm Images",
      value: "Unknown",
      supported: false,
      category: "Graphics",
    },
  ];
}

function detectClipboardCapabilities(): Capability[] {
  return [
    {
      name: "OSC 52",
      value: "Unknown",
      supported: false,
      category: "Clipboard",
    },
    {
      name: "System Clipboard",
      value: "Unknown",
      supported: false,
      category: "Clipboard",
    },
  ];
}

function detectSizeCapabilities(): Capability[] {
  const cols = process.stdout.columns || 80;
  const rows = process.stdout.rows || 24;

  return [
    { name: "Terminal Columns", value: String(cols), supported: true, category: "Size" },
    { name: "Terminal Rows", value: String(rows), supported: true, category: "Size" },
    {
      name: "Pixel Size",
      value: "Unknown",
      supported: false,
      category: "Size",
    },
    {
      name: "Cell Size",
      value: "Unknown",
      supported: false,
      category: "Size",
    },
  ];
}

function detectEnvironmentCapabilities(): Capability[] {
  return [
    { name: "Node.js Version", value: process.version, supported: true, category: "Environment" },
    { name: "Platform", value: process.platform, supported: true, category: "Environment" },
    { name: "Architecture", value: process.arch, supported: true, category: "Environment" },
    { name: "BetterTUI Version", value: "1.0.0-dev", supported: true, category: "Environment" },
  ];
}

function getBadgeVariant(supported: boolean, value: string): "success" | "danger" | "info" {
  if (value === "Unknown" || value === "Not set") return "info";
  return supported ? "success" : "danger";
}

const CATEGORIES = [
  "Terminal Info",
  "Display",
  "Input",
  "Rendering",
  "Graphics",
  "Clipboard",
  "Size",
  "Environment",
];

function Inspector() {
  const runtime = useRuntime();
  const allCapabilities = [
    ...detectTerminalInfo(),
    ...detectDisplayCapabilities(),
    ...detectInputCapabilities(),
    ...detectRenderingCapabilities(),
    ...detectGraphicsCapabilities(),
    ...detectClipboardCapabilities(),
    ...detectSizeCapabilities(),
    ...detectEnvironmentCapabilities(),
  ];

  useKeyboard((key) => {
    if (key.key === "q") {
      runtime?.dispose();
      process.exit(0);
    }
    return true;
  });

  return (
    <Flex flexDirection="column" width="100%" height="100%">
      <Box>
        <Heading level={1}>Capability Inspector</Heading>
      </Box>
      <Separator />
      <Box flexDirection="column" flexGrow={1}>
        {CATEGORIES.map((category) => {
          const caps = allCapabilities.filter((c) => c.category === category);
          return (
            <Box key={category} flexDirection="column" marginTop={1}>
              <Heading level={2}>{category}</Heading>
              {caps.map((cap) => (
                <Flex key={cap.name} marginTop={0}>
                  <Box width={22}>
                    <Text>{cap.name}</Text>
                  </Box>
                  <Box width={24}>
                    <Text>{cap.value}</Text>
                  </Box>
                  <Box>
                    <Badge variant={getBadgeVariant(cap.supported, cap.value)}>
                      {cap.value === "Unknown" || cap.value === "Not set"
                        ? "Unknown"
                        : cap.supported
                          ? "Supported"
                          : "Not Supported"}
                    </Badge>
                  </Box>
                </Flex>
              ))}
            </Box>
          );
        })}
      </Box>
      <Separator />
      <StatusLine left="Tab: cycle category | q: quit" right="BetterTUI Capability Inspector" />
    </Flex>
  );
}

function renderApp() {
  render(<Inspector />);
}

console.log("BetterTUI Capability Inspector");
console.log("Press q to quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();

process.on("SIGINT", () => {
  process.exit(0);
});
