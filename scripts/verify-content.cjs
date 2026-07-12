const path = require("node:path");
const { createElement } = require("react");

let captured = "";
const origWrite = process.stdout.write.bind(process.stdout);
process.stdout.write = (chunk) => {
  captured += Buffer.isBuffer(chunk) ? chunk.toString("utf8") : String(chunk);
  return true;
};

const { render } = require(path.join(__dirname, "../packages/react/dist/index.mjs"));

const element = createElement(
  "Flex",
  { flexDirection: "column", width: 40, height: 10 },
  createElement("Text", { bold: true, width: 20, height: 1 }, "REACT_ENGINE_PROOF"),
);

const handle = render(element);

setTimeout(() => {
  handle.dispose();

  process.stdout.write = origWrite;

  console.log("captured ANSI length:", captured.length);
  console.log("contains 'REACT_ENGINE_PROOF':", captured.includes("REACT_ENGINE_PROOF"));
}, 50);
