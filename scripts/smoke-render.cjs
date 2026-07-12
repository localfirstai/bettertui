const path = require("node:path");
const { createElement } = require("react");
const { createEngine } = require(path.join(__dirname, "../packages/core/dist/index.mjs"));

console.error("[1] testing native engine load directly...");
try {
  const engine = createEngine(40, 3);
  console.error("[1] engine loaded OK, root:", engine.root());
  const json = JSON.stringify([
    { type: "CreateNode", id: 1, kind: "Flex" },
    { type: "CreateNode", id: 2, kind: "Text" },
    { type: "SetText", id: 2, text: "Direct engine test" },
    { type: "AppendChild", parent: 1, child: 2 },
    { type: "SetFlexDirection", id: 1, direction: "row" },
  ]);
  const res = engine.processCommands(json);
  console.error("[1] processCommands:", res);
  engine.beginFrame();
  const frame = engine.render();
  engine.commitFrame();
  console.error("[1] frame keys:", Object.keys(frame), "outputData len:", frame.outputData?.length);
  const { render } = require(path.join(__dirname, "../packages/react/dist/index.mjs"));
  console.error("[2] testing react render()...");
  const element = createElement(
    "Flex",
    { flexDirection: "column" },
    createElement("Text", { bold: true }, "React->engine"),
  );
  const handle = render(element);
  console.error("[2] render returned, runtime:", !!handle.runtime);
  handle.dispose();
} catch (e) {
  console.error("FAILED:", e.message);
  console.error(e.stack);
}
