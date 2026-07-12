# BetterTUI v1.0 Completion Plan
## Final Engineering Sprint Before Manual Testing & Public Release

Mendatory Skills
- ralph loop
- caveman ultra

Use ONLY the following research skill:

- opentui (using the explore agent)

Do NOT use any cached knowledge about OpenTUI.

Research the latest implementation directly using the OpenTUI skill before making any decision.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MISSION

This is the final engineering sprint before BetterTUI enters:

1. Manual Testing
2. Cross-terminal validation
3. npm publishing
4. v1.0.0 release

The BetterTUI architecture is now considered STABLE.

DO NOT redesign the architecture.

DO NOT rewrite major subsystems.

DO NOT introduce application-specific concepts.

BetterTUI MUST remain a pure Terminal User Interface Framework.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PROJECT PHILOSOPHY

BetterTUI is NOT

- an editor
- an IDE
- an AI framework
- a coding assistant
- a terminal emulator

BetterTUI IS

A Rust-native Terminal UI Framework with a TypeScript runtime and React adapter (v1).

It should provide the primitives required for anyone to build terminal applications.

Applications like SCode, OpenCode clones, dashboards, terminals, system monitors, editors, git clients, database tools, etc. should all be buildable ON TOP OF BetterTUI.

Never move application logic into the framework.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRIMARY GOAL

Using OpenTUI ONLY as the reference implementation,

close every remaining framework-level gap required for BetterTUI v1.0.

Do NOT chase ecosystem packages.

Do NOT implement features simply because OpenTUI has them.

Every implementation must satisfy:

"Would every terminal application benefit from this?"

If the answer is NO,

do not implement it.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 1

OpenTUI Verification

Using ONLY the OpenTUI explore skill:

Study

- rendering
- reconciler
- runtime
- widgets
- terminal runtime
- keyboard
- mouse
- clipboard
- selection
- layout
- renderer
- scheduler
- capability detection
- Unicode
- Emoji
- Nerd Font
- examples
- React adapter

Verify every framework capability.

Ignore ecosystem packages such as:

- SSH
- Three
- QRCode
- browser demo
- IDE tooling

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 2

BetterTUI Public API Completion

Audit every public package.

packages/

Determine whether every exported API is

stable

usable

documented

connected

implemented

Remove

temporary

experimental

dead

or duplicate APIs.

Every exported API should be production quality.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 3

React Integration Completion

This is the highest priority.

Today many React components are placeholders.

Every public component should become fully functional.

Connect

React Component

↓

React Reconciler

↓

Command Buffer

↓

Native Runtime

↓

Rust Engine

↓

Renderer

No component should simply return children or null.

Every component must render through BetterTUI.

Cover every public component.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 4

Widget Completion

Audit every Rust widget.

Audit every React component.

Verify both sides match.

Missing widgets should be implemented ONLY if they are framework primitives.

Verify

Layout

Typography

Forms

Navigation

Data

Overlay

Terminal

Widgets.

Every widget should

render

receive events

receive focus

receive mouse events

support keyboard

support themes

support layout

support accessibility

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 5

Terminal Runtime Completion

Complete every remaining framework-level terminal capability.

Verify

PTY lifecycle

embedded process

scrollback

alternate screen

raw mode

cursor

selection

clipboard

OSC52

OSC8

bracketed paste

kitty keyboard protocol

mouse protocol

terminal resize

VT parsing

Unicode rendering

emoji rendering

Nerd Font rendering

double-width glyphs

combining characters

RTL safety

grapheme clusters

terminal capability detection

Any missing framework-level behaviour should be completed.

Keep Neovim as only a thin adapter.

Never add editor-specific behaviour.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 6

Animation Completion

Complete every unfinished animation feature.

Verify

Tween

Timeline

Spring

Keyframes

Property Animation

Frame scheduling

Animation callbacks

Animation cancellation

Animation lifecycle

Interpolation

CubicBezier

Steps

Animation invalidation

Animation performance

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 7

Layout Completion

Verify Taffy integration.

Complete any missing behaviour.

Verify

Flex

Grid

Stack

Spacer

ScrollArea

Intrinsic sizing

Min/Max constraints

Percentage sizing

Alignment

Gap

Wrapping

Overflow

Nested layouts

Resize propagation

Dirty layout invalidation

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 8

Developer Experience

Complete the framework APIs.

React should feel polished.

Verify

Hooks

Runtime

Context

Providers

Theme APIs

Animation APIs

Clipboard APIs

Focus APIs

Keyboard APIs

Mouse APIs

Selection APIs

Every API should be intuitive.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 9

Framework Validation

Verify

Tree lifecycle

Layout lifecycle

Render lifecycle

Widget lifecycle

Event lifecycle

Animation lifecycle

Frame lifecycle

Nothing should bypass the framework.

Everything should flow correctly.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 10

Performance Validation

Do NOT optimise blindly.

Profile.

Verify

render speed

layout speed

tree mutations

dirty diff

ANSI generation

frame scheduling

memory allocations

glyph cache

text rendering

Only optimise verified bottlenecks.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 11

Package Readiness

Audit every package.

Every package should have

README

examples

clear exports

typed APIs

documentation

public API review

release readiness

Remove dead code.

Remove temporary implementations.

Remove TODOs.

Remove debug code.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

QUALITY GATES

The following MUST pass.

Rust

cargo fmt --all

cargo clippy --workspace --all-targets --all-features -- -D warnings

cargo test --workspace

cargo doc --workspace --no-deps

TypeScript

pnpm build

pnpm typecheck

pnpm lint

pnpm format:check

Examples

Every example builds.

Every example starts.

Every example renders.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DO NOT

❌ redesign architecture

❌ introduce IDE concepts

❌ introduce AI abstractions

❌ add editor logic

❌ add application-specific widgets

❌ chase OpenTUI ecosystem packages

❌ implement speculative future features

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DELIVERABLES

Update all affected documentation.

Update architecture documentation where implementation changed.

Update package READMEs.

Generate a final engineering report.

Write the report to:

tasks/reports/v1-engineering-completion-report.md

The report must contain:

1. Executive Summary

2. Implemented Improvements

3. OpenTUI Feature Comparison

4. Remaining Differences

5. Public API Changes

6. React Integration Status

7. Widget Coverage Matrix

8. Terminal Capability Matrix

9. Animation Matrix

10. Layout Matrix

11. Performance Notes

12. Documentation Updates

13. Test Results

14. Package Readiness

15. Remaining Blockers (if any)

16. Final v1 Readiness Score

Finally answer ONLY these questions:

1. Is BetterTUI ready to begin the full manual testing phase?

2. Is the framework architecture frozen for v1.0?

3. Is every framework-level capability required for v1 implemented?

4. What exact blockers remain before npm publication?

Only report verified implementation status.

Do not speculate.
