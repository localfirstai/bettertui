import {
  Box,
  type CliRenderer,
  Markdown,
  SyntaxStyle,
  Text,
  createCliRenderer,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

/** Local stub type for a markdown code block renderer callback. */
type MarkdownCodeBlockRenderer = (token: {
  text: string;
  lang?: string;
}) => Box;

/** Local stub factory — registers per-language block renderers. */
function _createMarkdownCodeBlockRenderer(
  _renderers: Record<string, MarkdownCodeBlockRenderer>,
): MarkdownCodeBlockRenderer {
  return (token: { text: string; lang?: string }) => {
    const lang = token.lang ?? "";
    const renderer = _renderers[lang];
    if (renderer) return renderer(token);
    return new Box(null as unknown as CliRenderer, {});
  };
}

interface TaskFlowStep {
  label: string;
  status: "done" | "active" | "queued" | "blocked";
}

interface TaskFlowDocument {
  title: string;
  owner: string;
  steps: TaskFlowStep[];
}

const markdownContent = `# Markdown Code Block Renderers

This markdown document contains a fenced \`taskflow\` block. The source stays plain text, but the markdown renderer swaps that single language into a custom BetterTUI widget.

\`\`\`taskflow
title Ship markdown plug-ins
owner terminal team
step Parse fenced language done
step Render custom widget active
step Capture screenshot queued
step Open pull request queued
\`\`\`

Everything around the custom fence is still rendered by the normal markdown pipeline, including **bold text**, links, and ordinary code fences.

\`\`\`ts
const renderNode = createMarkdownCodeBlockRenderer({ taskflow: renderTaskFlow })
\`\`\`
`;

let root: Box | null = null;
let syntaxStyle: SyntaxStyle | null = null;

function createSyntaxStyle(): SyntaxStyle {
  return new SyntaxStyle();
}

function parseTaskFlow(source: string): TaskFlowDocument {
  const document: TaskFlowDocument = {
    title: "Untitled taskflow",
    owner: "unknown",
    steps: [],
  };

  for (const line of source.split("\n")) {
    const trimmed = line.trim();
    if (!trimmed) continue;

    if (trimmed.startsWith("title ")) {
      document.title = trimmed.slice("title ".length);
      continue;
    }

    if (trimmed.startsWith("owner ")) {
      document.owner = trimmed.slice("owner ".length);
      continue;
    }

    if (trimmed.startsWith("step ")) {
      const rest = trimmed.slice("step ".length);
      const statusStart = rest.lastIndexOf(" ");
      if (statusStart === -1) continue;

      const label = rest.slice(0, statusStart);
      const status = rest.slice(statusStart + 1) as TaskFlowStep["status"];
      if (status === "done" || status === "active" || status === "queued" || status === "blocked") {
        document.steps.push({ label, status });
      }
    }
  }

  return document;
}

function stepStyle(status: TaskFlowStep["status"]): {
  marker: string;
  color: string;
} {
  if (status === "done") return { marker: "OK", color: "#86EFAC" };
  if (status === "active") return { marker: ">>", color: "#67E8F9" };
  if (status === "blocked") return { marker: "!!", color: "#FDA4AF" };
  return { marker: "--", color: "#CBD5E1" };
}

function _createTaskFlowRenderer(renderer: CliRenderer): MarkdownCodeBlockRenderer {
  return (token: { text: string; lang?: string }) => {
    const flow = parseTaskFlow(token.text);
    const card = new Box(renderer, {
      id: "taskflow-card",
      width: "100%",
      flexDirection: "column",
      border: true,
      borderStyle: "round",
      borderColor: "#38BDF8",
      backgroundColor: "#07111F",
      paddingX: 2,
      paddingY: 1,
      marginBottom: 1,
      title: `taskflow: ${flow.title}`,
      titleAlignment: "left",
    });

    card.add(
      new Text(renderer, {
        content: `owner ${flow.owner}  |  ${flow.steps.length} steps`,
        fg: "#93A4B8",
        width: "100%",
      }),
    );

    for (const step of flow.steps) {
      const style = stepStyle(step.status);
      card.add(
        new Text(renderer, {
          content: `${style.marker} ${step.label.padEnd(28)} ${step.status}`,
          fg: style.color,
          width: "100%",
        }),
      );
    }

    return card;
  };
}

export function run(renderer: CliRenderer): void {
  renderer.start();
  renderer.setBackgroundColor("#020617");
  syntaxStyle = createSyntaxStyle();

  root = new Box(renderer, {
    id: "markdown-code-block-renderer-root",
    width: "100%",
    height: "100%",
    flexDirection: "column",
    paddingX: 2,
    paddingY: 1,
    backgroundColor: "#020617",
  });
  renderer.root.add(root);

  root.add(
    new Markdown(renderer, {
      id: "markdown-code-block-renderer-doc",
      content: markdownContent,
      fg: "#DDE7FF",
      bg: "#020617",
      width: "100%",
    }),
  );
}

export function destroy(): void {
  root?.destroyRecursively();
  syntaxStyle?.destroy();
  root = null;
  syntaxStyle = null;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
    targetFps: 60,
  });
  run(renderer);
  setupCommonDemoKeys(renderer);
}
