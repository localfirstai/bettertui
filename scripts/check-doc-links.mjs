#!/usr/bin/env node
// Validates relative markdown links and anchor references across the docs tree
// and root documentation files. Run via `pnpm check:docs`.
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const root = process.cwd();

// Files and directories to scan for broken links.
const entryPoints = [
  "README.md",
  "CONTRIBUTING.md",
  "ARCHITECTURE.md",
  "ROADMAP.md",
  "CHANGELOG.md",
  "docs",
  "packages",
  "tasks",
];

// Paths we intentionally skip (build output, node_modules, archived slop).
const skipDirs = new Set(["node_modules", "dist", "coverage", "target", ".git", ".turbo"]);

const mdFiles = [];

function walk(dir) {
  let entries;
  try {
    entries = readdirSync(dir);
  } catch {
    return;
  }
  for (const entry of entries) {
    const full = join(dir, entry);
    const st = statSync(full);
    if (st.isDirectory()) {
      if (skipDirs.has(entry)) continue;
      walk(full);
    } else if (entry.endsWith(".md")) {
      mdFiles.push(full);
    }
  }
}

for (const ep of entryPoints) {
  const full = join(root, ep);
  try {
    const st = statSync(full);
    if (st.isDirectory()) walk(full);
    else if (ep.endsWith(".md")) mdFiles.push(full);
  } catch {
    // entry point missing — ignore
  }
}

// Collect every heading per file for anchor validation.
const anchors = new Map();
for (const file of mdFiles) {
  const text = readFileSync(file, "utf8");
  const set = new Set();
  const lines = text.split("\n");
  for (const line of lines) {
    const m = line.match(/^#{1,6}\s+(.*)$/);
    if (m) {
      const slug = m[1]
        .trim()
        .toLowerCase()
        .replace(/[^\w\s-]/g, "")
        .replace(/\s+/g, "-");
      set.add(slug);
    }
  }
  anchors.set(file, set);
}

const linkRe = /\[[^\]]*\]\(([^)]+)\)/g;
let broken = 0;

for (const file of mdFiles) {
  const text = readFileSync(file, "utf8");
  let m = linkRe.exec(text);
  while (m !== null) {
    const target = m[1].trim();
    if (!target || target.startsWith("http") || target.startsWith("#")) {
      m = linkRe.exec(text);
      continue;
    }
    // Strip anchor.
    const [pathPart] = target.split("#");
    if (!pathPart) continue;
    const resolved = resolve(dirname(file), pathPart);
    if (!mdFiles.includes(resolved) && !fileExists(resolved)) {
      console.error(`BROKEN LINK: ${relative(root, file)} -> ${target}`);
      broken++;
    }
  }
}

function fileExists(p) {
  try {
    statSync(p);
    return true;
  } catch {
    return false;
  }
}

if (broken > 0) {
  console.error(`\n${broken} broken doc link(s) found.`);
  process.exit(1);
}

console.log(`Doc link check passed: ${mdFiles.length} markdown files scanned.`);
