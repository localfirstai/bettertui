> This is a repository restructure only. The public API, runtime behaviour, rendering pipeline, React integration, Rust engine, TypeScript runtime, tests, and examples must remain functionally identical.

packages/core/
├── src/ # TypeScript runtime
├── crates/ # Rust workspace
│ ├── engine/
│ └── bindings/
├── assets/ # Fonts, icons, etc.
├── scripts/ # Build/release tooling
├── tests/ # Cross-language integration tests
├── benches/ # Native benchmarks
├── Cargo.toml
├── build.rs
├── package.json
└── README.md

---

# BetterTUI Architecture Migration

## Final Repository Restructure (Architecture Freeze)

Use the following skills:

- opencode
- ralph-loop
- sequential-thinking
- caveman

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MISSION

This is the FINAL repository architecture migration before BetterTUI v1.0.

The architecture of BetterTUI is now considered stable.

This task exists ONLY to reorganise the repository into its final layout.

This is NOT a feature task.

This is NOT a refactor of the runtime.

This is NOT a redesign.

This is a repository migration.

The entire behaviour of BetterTUI MUST remain identical after completion.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

ARCHITECTURE GOAL

We no longer want

native/
packages/native/

These two locations duplicate ownership.

The BetterTUI runtime should have ONE owner.

That owner is

packages/core/

packages/core becomes the entire BetterTUI engine.

It owns BOTH

Rust

AND

TypeScript runtime.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

FINAL PACKAGE STRUCTURE

packages/

core/
react/
shared/
icons/
devtools/
benchmark/

No separate native/ directory should remain.

No packages/native package should remain.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

TARGET STRUCTURE

packages/core/

package.json
Cargo.toml
Cargo.lock (if tracked)
build.rs
README.md

src/
(TypeScript runtime)

crates/

engine/

bindings/

assets/

fonts/

icons/

scripts/

tests/

benches/

.cargo/

All Rust-specific files belong inside packages/core.

Everything required to build the Rust engine should live here.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

CRITICAL RULE

This migration MUST preserve

100%

runtime compatibility.

No public API may change.

No package consumer should notice the migration.

Only repository organisation changes.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 1

Repository Audit

Study the ENTIRE repository before moving anything.

Understand

native/

packages/native/

packages/core/

Cargo.toml

Cargo.lock

.cargo

build.rs

scripts

Turbo

pnpm workspace

Rust workspace

NAPI build

Every dependency.

Generate a dependency graph.

Understand every build path.

Do NOT move anything yet.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 2

Migration Plan

Determine

Every file

Every directory

Every Cargo member

Every TS source

Every build script

Every asset

Every generated file

that belongs inside packages/core.

Create an internal migration order.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 3

Move Rust Workspace

Move

native/

↓

packages/core/crates/

Maintain

engine

bindings

without changing their internal architecture.

Rust crates should remain independent crates.

Only their location changes.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 4

Move Rust Workspace Files

Move

Cargo.toml

Cargo.lock

build.rs

.cargo

Rust scripts

workspace configuration

to packages/core.

packages/core becomes the Rust workspace root.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 5

Move TypeScript Runtime

Move

packages/native/src

↓

packages/core/src

Merge cleanly.

Do NOT duplicate code.

Do NOT overwrite existing files.

If duplicate functionality exists

merge carefully.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 6

Update Cargo Workspace

Update

workspace members

relative paths

crate references

build scripts

workspace dependencies

path dependencies

Ensure Cargo resolves correctly.

No absolute paths.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 7

Update TypeScript

Update

imports

exports

package references

workspace references

Turbo configuration

pnpm workspace

TypeScript paths

Build scripts

No broken imports.

No deep imports.

No obsolete references.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 8

Update napi Build

Ensure napi-rs still builds correctly.

Update

relative paths

Cargo metadata

generated bindings

loader paths

Node binary loading

Platform detection

Nothing should break.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 9

Update Build Tooling

Update

Turbo

pnpm

Cargo

scripts

release scripts

build scripts

developer scripts

documentation references

Everything should reference packages/core.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 10

Update Documentation

Update

README

Architecture docs

Package docs

Developer docs

Repository layout

Build instructions

Contributing guide

Every path reference.

Remove references to

native/

packages/native/

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 11

Delete Legacy Layout

ONLY AFTER

everything builds

everything tests

everything passes

remove

native/

packages/native/

No dead files.

No duplicated code.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 12

Verification

Verify

Rust workspace

TypeScript workspace

Turbo

pnpm

Cargo

NAPI

Examples

Tests

Benchmarks

Examples should still launch.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 13

Regression Audit

Search the ENTIRE repository.

Verify there are NO references remaining to

native/

packages/native/

old Cargo paths

old build paths

old workspace members

old documentation

old imports

old scripts

old CI references

old release paths

Everything should point to packages/core.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

QUALITY GATES

Rust

cargo fmt --all

cargo check --workspace

cargo build --workspace

cargo clippy --workspace --all-targets --all-features -- -D warnings

cargo test --workspace

cargo doc --workspace --no-deps

TypeScript

pnpm install

pnpm build

pnpm typecheck

pnpm lint

pnpm format:check

Examples

Every example builds.

Every example starts.

Every example still renders.

Native

NAPI builds correctly.

Rust engine loads correctly.

React still renders correctly.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

AFTER MIGRATION

Perform a complete repository audit.

Verify

No duplicate packages

No duplicate runtime

No duplicate Cargo workspaces

No duplicate assets

No duplicate scripts

No broken references

No stale documentation

No orphaned imports

No orphaned exports

No obsolete files

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DO NOT

❌ Change runtime behaviour

❌ Rewrite the engine

❌ Change public APIs

❌ Introduce breaking changes

❌ Redesign architecture

❌ Rename exported symbols

❌ Modify React behaviour

❌ Modify Rust engine logic

❌ Change rendering behaviour

This is a repository migration ONLY.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

FINAL REPORT

Do NOT create multiple reports.

Create ONE report only.

tasks/reports/core-architecture-migration-report.md

The report must contain

1. Executive Summary

2. Migration Overview

3. Old Repository Layout

4. New Repository Layout

5. Rust Workspace Migration

6. TypeScript Runtime Migration

7. Cargo Workspace Changes

8. Package Reference Updates

9. Build Tool Updates

10. Documentation Updates

11. Files Moved

12. Files Deleted

13. Import/Export Changes

14. Workspace Verification

15. Test Results

16. Build Results

17. Remaining Technical Debt

18. Architecture Validation

19. Final Package Dependency Graph

20. Final Repository Tree

Finally answer ONLY these questions

1. Is packages/core now the single owner of the BetterTUI runtime?

2. Has every Rust component been migrated successfully?

3. Has every TypeScript runtime component been migrated successfully?

4. Are there any remaining references to native/ or packages/native/?

5. Is the repository architecture now frozen for v1.0?

Only answer using verified implementation.
