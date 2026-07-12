import * as React from "react";
import { Flex, Text } from "../packages/react/src/components";
import { renderToString } from "../packages/react/src/test-renderer";

const out = renderToString(
  React.createElement(
    Flex,
    { flexDirection: "column", border: "solid" },
    React.createElement(Text, { color: "green" }, "Hello testing!"),
  ),
);

console.log(out);
