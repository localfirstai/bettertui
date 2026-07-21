/**
 * parser.bench.ts
 *
 * Throughput benchmarks for the TS-side input parsers. The actual parser
 * implementations live in `packages/core/src/lib/`:
 *   - `parse.keypress.ts`    (455 LoC) — xterm CSI/SS3 key encoding
 *   - `parse.keypress-kitty.ts` (461 LoC) — Kitty keyboard protocol CSI u
 *   - `parse.mouse.ts`       (249 LoC) — SGR / X10 / urxvt mouse
 *   - `stdin-parser.ts`      (1,683 LoC) — full stdin state machine
 *
 * OpenTUI counterpart: `keypress-debug-demo.ts` is the closest, but OpenTUI
 * benchmarks these informally through `lib/parse.keypress.test.ts` etc.
 *
 * Preconditions: none (pure-TS, no native round-trip).
 */

import {
  Keymap,
  MouseParser,
  StdinParser,
  parseKeypress,
  parseKittyKeyboard,
} from "@bettertui/core";
import { bench, describe } from "vitest";

describe("parseKeypress — xterm encoding", () => {
  bench("letter key", () => {
    parseKeypress("a");
  });

  bench("Ctrl+C", () => {
    parseKeypress("\u0003");
  });

  bench("arrow Up (CSI A)", () => {
    parseKeypress("\u001b[A");
  });

  bench("Ctrl+Shift+Right (CSI 1;6 C)", () => {
    parseKeypress("\u001b[1;6C");
  });

  bench("Home (CSI 1~)", () => {
    parseKeypress("\u001b[1~");
  });
});

describe("parseKittyKeyboard — CSI u encoding", () => {
  bench("a (CSI 97 ; 0 u)", () => {
    parseKittyKeyboard("\u001b[97;0u");
  });

  bench("Ctrl+A (CSI 1 ; 5 u)", () => {
    parseKittyKeyboard("\u001b[1;5u");
  });

  bench("Shift+Insert (CSI 2 ; 2 u)", () => {
    parseKittyKeyboard("\u001b[2;2u");
  });
});

describe("MouseParser — SGR mouse", () => {
  bench("left click at (10,5)", () => {
    const parser = new MouseParser();
    parser.parseMouseEvent(Buffer.from("\u001b[<0;10;5M", "latin1"));
  });

  bench("drag at (50,20)", () => {
    const parser = new MouseParser();
    parser.parseMouseEvent(Buffer.from("\u001b[<32;50;20M", "latin1"));
  });

  bench("release at (50,20)", () => {
    const parser = new MouseParser();
    parser.parseMouseEvent(Buffer.from("\u001b[<0;50;20m", "latin1"));
  });

  bench(
    "parseAllMouseEvents over 100 SGR clicks",
    () => {
      const parser = new MouseParser();
      const buf = Buffer.concat(Array(100).fill(Buffer.from("\u001b[<0;10;5M", "latin1")));
      const events = parser.parseAllMouseEvents(buf);
      if (events.length === 0) throw new Error("no events parsed");
    },
    { iterations: 20, time: 1000 },
  );
});

describe("StdinParser — bulk byte throughput", () => {
  bench(
    "100 letter keys (1 byte each)",
    () => {
      const parser = new StdinParser();
      for (let i = 0; i < 100; i++) {
        parser.push(Buffer.from("a"));
        parser.drain(() => {});
      }
    },
    { iterations: 20, time: 1000 },
  );

  bench(
    "100 arrow keys (3 bytes each)",
    () => {
      const parser = new StdinParser();
      const buf = Buffer.from("\u001b[A");
      for (let i = 0; i < 100; i++) {
        parser.push(buf);
        parser.drain(() => {});
      }
    },
    { iterations: 20, time: 1000 },
  );

  bench(
    "1 KB paste (single chunk)",
    () => {
      const parser = new StdinParser();
      // Wrap in bracketed paste markers so the parser recognises it as paste
      const payload = Buffer.concat([
        Buffer.from("\u001b[200~"),
        Buffer.alloc(1024, 0x61),
        Buffer.from("\u001b[201~"),
      ]);
      parser.push(payload);
      parser.drain(() => {});
    },
    { iterations: 20, time: 1500 },
  );

  bench(
    "100 mouse events (SGR, ~12 bytes each)",
    () => {
      const parser = new StdinParser();
      const buf = Buffer.from("\u001b[<0;10;5M");
      for (let i = 0; i < 100; i++) {
        parser.push(buf);
        parser.drain(() => {});
      }
    },
    { iterations: 20, time: 1500 },
  );
});

describe("Keymap — binding registration", () => {
  bench("Keymap construct", () => {
    new Keymap();
  });

  bench(
    "addSimpleBinding 100 entries",
    () => {
      const km = new Keymap();
      for (let i = 0; i < 100; i++) km.addSimpleBinding(`ctrl+${i}`, `cmd-${i}`);
    },
    { iterations: 10, time: 1000 },
  );

  bench(
    "addBinding 100 entries (with layer + description + priority)",
    () => {
      const km = new Keymap();
      for (let i = 0; i < 100; i++) {
        km.addBinding("default", `cmd-${i}`, `ctrl+${i}`, `cmd-${i}`, `desc-${i}`, i);
      }
    },
    { iterations: 10, time: 1000 },
  );

  bench(
    "handleKey over 100 pre-registered bindings",
    () => {
      const km = new Keymap();
      for (let i = 0; i < 100; i++) km.addSimpleBinding(`ctrl+${i % 100}`, `cmd-${i % 100}`);
      for (let i = 0; i < 100; i++) km.handleKey(`ctrl+${i % 100}`);
    },
    { iterations: 10, time: 1000 },
  );
});
