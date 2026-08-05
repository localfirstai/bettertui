// DevTools initialization module for React apps
import "./devtoolsPolyfill";

export function initReactDevTools(): void {
  try {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const devtools = require("react-devtools-core");
    devtools.initialize();
    devtools.connectToDevTools();
  } catch {
    // react-devtools-core not installed or failed to connect
  }
}
