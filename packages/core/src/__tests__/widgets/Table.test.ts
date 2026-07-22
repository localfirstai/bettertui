import { describe, expect, it, vi } from "vitest";
import { Table } from "../../widgets/Table";

const columns = [
  { header: "Name", width: 10, align: "left" as const },
  { header: "Age", width: 5, align: "right" as const },
  { header: "City", width: 12 },
];

const rows = [
  ["Alice", "30", "New York"],
  ["Bob", "25", "Los Angeles"],
  ["Carol", "35", "Chicago"],
];

describe("Table", () => {
  it("constructs with default options", () => {
    const t = new Table();
    expect(t.rowCount).toBe(0);
    expect(t.selectedRow).toBe(-1);
  });

  it("constructs with rows", () => {
    const t = new Table({ rows });
    expect(t.rowCount).toBe(3);
  });

  it("renderCommands creates a Box container", () => {
    const t = new Table({ columns, rows });
    const cmds = t.renderCommands("t1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") expect(cmds[0].kind).toBe("Box");
  });

  it("renderCommands includes column header text", () => {
    const t = new Table({ columns, rows });
    const cmds = t.renderCommands("t1");
    const texts = cmds
      .filter((c) => c.type === "SetText")
      .map((c) => (c as never as { text: string }).text);
    expect(texts.some((txt) => txt.trim() === "Name")).toBe(true);
    expect(texts.some((txt) => txt.trim() === "Age")).toBe(true);
    expect(texts.some((txt) => txt.trim() === "City")).toBe(true);
  });

  it("renderCommands renders data rows", () => {
    const t = new Table({ columns, rows });
    const cmds = t.renderCommands("t1");
    const texts = cmds
      .filter((c) => c.type === "SetText")
      .map((c) => (c as never as { text: string }).text);
    expect(texts.some((txt) => txt.trim() === "Alice")).toBe(true);
    expect(texts.some((txt) => txt.trim() === "Bob")).toBe(true);
  });

  it("renders border by default", () => {
    const t = new Table({ columns, rows });
    const cmds = t.renderCommands("t1");
    const texts = cmds
      .filter((c) => c.type === "SetText")
      .map((c) => (c as never as { text: string }).text);
    // Top border should contain corner characters
    expect(texts.some((txt) => txt.includes("┌"))).toBe(true);
    expect(texts.some((txt) => txt.includes("┐"))).toBe(true);
  });

  it("renders no border when showBorder is false", () => {
    const t = new Table({ columns, rows, showBorder: false });
    const cmds = t.renderCommands("t1");
    const texts = cmds
      .filter((c) => c.type === "SetText")
      .map((c) => (c as never as { text: string }).text);
    expect(texts.every((txt) => !txt.includes("┌") && !txt.includes("╔"))).toBe(true);
  });

  it("renders double border style", () => {
    const t = new Table({ columns, rows, borderStyle: "double" });
    const cmds = t.renderCommands("t1");
    const texts = cmds
      .filter((c) => c.type === "SetText")
      .map((c) => (c as never as { text: string }).text);
    expect(texts.some((txt) => txt.includes("╔"))).toBe(true);
  });

  it("renders rounded border style", () => {
    const t = new Table({ columns, rows, borderStyle: "rounded" });
    const cmds = t.renderCommands("t1");
    const texts = cmds
      .filter((c) => c.type === "SetText")
      .map((c) => (c as never as { text: string }).text);
    expect(texts.some((txt) => txt.includes("╭"))).toBe(true);
  });

  it("renders bold border style", () => {
    const t = new Table({ columns, rows, borderStyle: "bold" });
    const cmds = t.renderCommands("t1");
    const texts = cmds
      .filter((c) => c.type === "SetText")
      .map((c) => (c as never as { text: string }).text);
    expect(texts.some((txt) => txt.includes("┏"))).toBe(true);
  });

  it("hides header when showHeader is false", () => {
    const t = new Table({ columns, rows, showHeader: false });
    const cmds = t.renderCommands("t1");
    const boldCmds = cmds.filter((c) => c.type === "SetBold");
    // No bold header cells
    expect(boldCmds.length).toBe(0);
  });

  it("highlights selected row with inverse", () => {
    const t = new Table({ columns, rows, selectedRow: 0 });
    const cmds = t.renderCommands("t1");
    const inverseCmds = cmds.filter((c) => c.type === "SetInverse");
    expect(inverseCmds.length).toBeGreaterThan(0);
  });

  it("handleKey selects rows with down/up", () => {
    let selected: string[] | null = null;
    const t = new Table({
      columns,
      rows,
      selectedRow: 0,
      onSelect: (row) => {
        selected = row;
      },
    });
    t.handleKey({
      key: "down",
      code: "ArrowDown",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(t.selectedRow).toBe(1);
    expect(selected).toEqual(["Bob", "25", "Los Angeles"]);
  });

  it("handleKey does not go below 0", () => {
    const t = new Table({ columns, rows, selectedRow: 0 });
    t.handleKey({
      key: "up",
      code: "ArrowUp",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(t.selectedRow).toBe(0);
  });

  it("handleKey does not exceed last row", () => {
    const t = new Table({ columns, rows, selectedRow: 2 });
    t.handleKey({
      key: "down",
      code: "ArrowDown",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(t.selectedRow).toBe(2);
  });

  it("handleKey home goes to first row", () => {
    const t = new Table({ columns, rows, selectedRow: 2 });
    t.handleKey({
      key: "home",
      code: "Home",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(t.selectedRow).toBe(0);
  });

  it("handleKey end goes to last row", () => {
    const t = new Table({ columns, rows, selectedRow: 0 });
    t.handleKey({
      key: "end",
      code: "End",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(t.selectedRow).toBe(2);
  });

  it("handleKey return fires onSelect", () => {
    const onSelect = vi.fn();
    const t = new Table({ columns, rows, selectedRow: 1, onSelect });
    t.handleKey({
      key: "return",
      code: "Enter",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(onSelect).toHaveBeenCalledWith(["Bob", "25", "Los Angeles"], 1);
  });

  it("handleKey returns false for empty table", () => {
    const t = new Table();
    const result = t.handleKey({
      key: "down",
      code: "ArrowDown",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(result).toBe(false);
  });

  it("update changes rows and selectedRow", () => {
    const t = new Table({ columns, rows });
    const newRows = [["Dave", "28", "Seattle"]];
    t.update({ rows: newRows, selectedRow: 0 });
    expect(t.rowCount).toBe(1);
    expect(t.selectedRow).toBe(0);
  });

  it("renders striped rows with dim on odd rows", () => {
    const t = new Table({ columns, rows, striped: true, selectedRow: -1 });
    const cmds = t.renderCommands("t1");
    const dimCmds = cmds.filter((c) => c.type === "SetDim");
    // There should be dim commands for border + odd rows
    expect(dimCmds.length).toBeGreaterThan(0);
  });

  it("renders without column definitions (auto-infer)", () => {
    const t = new Table({ rows });
    const cmds = t.renderCommands("t1");
    expect(cmds.length).toBeGreaterThan(0);
    const texts = cmds
      .filter((c) => c.type === "SetText")
      .map((c) => (c as never as { text: string }).text);
    expect(texts.some((txt) => txt.trim() === "Alice")).toBe(true);
  });
});
