# BetterTUI Architecture Boundary Audit

**Date:** 2026-07-11 16:52
**Auditor:** Buffy (AI Chief Architect)
**Commit:** dfc27fa59d2dcd01a567b40000c5d2bbec4f2e56
**Status:** COMPLETE

---

## 1. Executive Summary

BetterTUI is a Rust-native + TypeScript adapter TUI framework with a clean, well-tested core engine (910 library tests passing, clippy clean at `-D warnings`). The foundation — tree/arena model, protocol command batching, Taffy layout, framebuffer rendering, event bus, focus management, PTY — is solid, framework-agnostic, and aligned with the stated vision of reusable TUI primitives.

However, the repository has drifted into application territory. The most material finding is that the engine ships **AI chat application code** (`widgets/chat/` with `Role::{User,Assistant,System}`, `Message`, `ChatView`, `ThinkingIndicator`) and an **embedded Neovim integration** (`neovim/` with process/state management) directly inside the core Rust crate. These are Category C and do not belong in a framework. The `editor/` module is a 13-line empty stub (`pub struct Editor;`) — Category D/experimental. A dedicated `docs/architecture/IDE_PLATFORM.md` explicitly states the objective to "make BetterTUI capable of powering a modern coding IDE," confirming the intent to build an application layer on top of (and inside) the framework.

A second-class issue is **stale tests**: the integration test (`tests/integration_test.rs`) does not even compile — it references a removed `RenderScheduler` and a renamed `RenderFrame.ansi_data` field. This is testing debt that hides regression coverage.

**Key Numbers:**
- Rust files: 164 | LOC: 31,589
- TS files: 30 | LOC: 1,854
- Tests: 910 lib tests passing; integration test BROKEN (won't compile)
- Clippy warnings: 0 (clean with `-D warnings`)
- Category A modules: 33
- Category B modules: 6
- Category C modules: 4 (MUST MIGRATE)
- Dead code modules: 1 (editor stub, D/E borderline)

---

## 2. Repository Health Score

**Score: 7.5/10**

| Dimension | Score | Weight | Notes |
|-----------|-------|--------|-------|
| Architecture alignment | 6/10 | 25% | AI chat + Neovim + IDE intent pollute the framework core |
| Code quality | 9/10 | 20% | Clippy clean, idiomatic Rust, clear module boundaries |
| Test coverage | 7/10 | 15% | 910 lib tests great; integration test broken = hidden gaps |
| Documentation | 8/10 | 10% | 24 architecture docs; but IDE doc signals scope creep |
| Dependency health | 9/10 | 10% | Correct direction; minor unused dep wiring in TS |
| Module boundaries | 6/10 | 20% | Application code lives in core crate; editor stub stray |
| **Weighted Total** | **7.5/10** | | |

---

## 3. Quantitative Metrics

| Metric | Value |
|--------|-------|
| Rust source files | 164 |
| Rust LOC | 31,589 |
| TypeScript source files | 30 |
| TypeScript LOC | 1,854 |
| Total tests | 910 lib passing; integration BROKEN |
| Test pass rate | 100% (lib) / N/A (integration won't compile) |
| Clippy warnings | 0 |
| Rust crates (Cargo.lock) | 111 |
| npm packages | 9 |
| Widget count | 19 |
| Architecture docs | 24 |

---

## 4. Module Classification

### Category A: Core Framework (Keep)

| Module | Path | Purpose | LOC | Tests | Notes |
|--------|------|---------|-----|-------|-------|
| tree | src/tree | NodeArena, RenderNode, Style, LayoutProps, colors | — | yes | Foundation |
| protocol | src/protocol | Command batching, CommandProcessor, CommandBuffer | 1,564 | yes | Core FFI contract |
| layout | src/layout | Taffy layout engine, LayoutTreeSync | — | — | Core |
| renderer | src/renderer | Renderer, RenderBackend, AnsiBackend | — | yes | Core |
| framebuffer | src/framebuffer | Cell grid, double buffering | — | yes | Core |
| painter | src/painter | RenderTree → FrameBuffer paint | — | yes | Core |
| dirty_diff | src/dirty_diff | DirtyDiff, DirtyRegion | — | yes | Core |
| compositor | src/compositor | Layers, z-index compositing | — | — | Core |
| events | src/events | EventBus, capture/target/bubble | — | yes | Core |
| focus | src/focus | FocusManager, FocusScope, traversal | — | — | Core |
| pty | src/pty | PtyRuntime, PtyProcess, reader/writer | — | — | Core |
| terminal | src/terminal | Raw mode, alt screen, crossterm | — | — | Core |
| capabilities | src/capabilities | CapabilityDetector | — | — | Core |
| text | src/text | TextBuffer, cursor, search, undo | — | — | Core |
| input | src/input | InputRuntime, keyboard/mouse/clipboard state | — | — | Core |
| keyboard | src/keyboard | Keyboard handling | — | — | Core |
| mouse | src/mouse | Mouse handling | — | — | Core |
| clipboard | src/clipboard | Clipboard | — | — | Core |
| glyph | src/glyph | Glyph shaping/measurement | — | — | Core |
| ansi | src/ansi | ANSI encoder | — | yes | Core |
| graphics | src/graphics | Drawing primitives | — | — | Core |
| render_object | src/render_object | PaintContext, RenderObject, RenderTree | — | yes | Core |
| scheduler | src/scheduler | Priority queue, frame budget | — | yes | Core |
| screen | src/screen | Screen modes | — | — | Core |
| selection | src/selection | Text selection | — | — | Core |
| snapshot | src/snapshot | State snapshot | — | yes | Core |
| engine/core | src/engine/core | Engine orchestration | — | — | Core |
| ffi | src/ffi | napi-rs FFI boundary | — | — | Core |
| animation | src/animation | Animation primitives | — | — | Core |
| keybinding | src/keybinding | Keymap / keybindings | — | yes | Core |
| nerdfont | src/nerdfont | NerdFont glyph support | — | — | Core |
| plugin | src/plugin | Plugin API | — | yes | Core |
| palette | src/palette | Color palette | — | yes | Core |

### Category B: Framework Extension (Keep with Flag)

| Module | Path | Purpose | LOC | Tests | Notes |
|--------|------|---------|-----|-------|-------|
| widgets/markdown | src/widgets/markdown | Markdown parser/renderer | 1,027 | yes | Reusable B; verify scope creep |
| widgets/code_widget | src/widgets/code_widget | Code display widget | 168 | — | Generic-ish; verify syntax coupling |
| widgets (generic) | src/widgets/* | Button, box, label, input, table, tree, tabs, etc. (19 total) | — | partial | B (framework UI kit) |
| engine/inspector | src/engine/inspector | Developer inspector | 335 | — | Useful dev tool; Category B |
| widgets/tooltip | src/widgets/tooltip | Tooltip | — | — | B |
| widget pipeline/registry | src/widgets/pipeline,registry | Widget lifecycle | — | — | B |

### Category C: Application Layer (MUST MIGRATE)

| Module | Path | Purpose | LOC | Tests | Migrate To | Priority |
|--------|------|---------|-----|-------|------------|----------|
| widgets/chat | src/widgets/chat | AI chat UI: Role(User/Assistant/System), Message, ChatView, ThinkingIndicator, StatusBar | 731 | yes | separate `scode-chat` / app crate | High |
| neovim | src/neovim | Embedded Neovim process/state integration | 514 | — | external plugin crate | High |
| widgets/prompt_composer | src/widgets/prompt_composer | AI prompt composer w/ history (not generic text input) | 484 | 19 | app crate | Med |
| widgets/code_widget (scope) | src/widgets/code_widget | Syntax-aware code widget — borderline app concern | 168 | — | keep generic / split syntax | Med |

### Category D: Experimental (Review)

| Module | Path | Purpose | Status | Recommendation |
|--------|------|---------|--------|----------------|
| editor | src/editor | `pub struct Editor;` empty stub | Abandoned stub | Remove or promote to real primitive |

### Category E: Dead Code (Remove)

| Module | Path | Evidence | Recommendation |
|--------|------|----------|----------------|
| editor | src/editor | Only `pub struct Editor;`; never constructed or referenced outside lib.rs `pub mod editor` | Remove |

### Category F: Duplicate (Consolidate)

| Module A | Module B | Problem | Canonical | Action |
|----------|----------|---------|-----------|--------|
| widgets/text_widget | widgets/textarea_widget | Both render text; textarea adds editing | text_widget + textarea | Verify not overlapping; keep distinct roles |
| ansi encoder | renderer AnsiBackend | Both emit ANSI | protocol/renderer | Confirm single emitter path |

---

## 5. Application-Specific Code Detail

### widgets/chat
- **Path:** `native/engine/src/widgets/chat/`
- **What it does:** Full AI chat UI — `Role` enum (User/Assistant/System), `Message`, `ChatState`, `ChatStatus`, `ChatView`, `ThinkingIndicator`, `StatusBar`.
- **Why it doesn't belong:** This is AI-assistant application code, not a TUI primitive. Exported publicly (`pub use chat::...` in `widgets/mod.rs`).
- **Suggested home:** `@bettertui/app-chat` or a separate SCode crate/repo.
- **Migration priority:** High
- **Dependencies to untangle:** `widgets::Widget`, `tree::style`, `events::types`; no external app deps — self-contained, easy to lift.
- **Estimated effort:** Medium

### neovim
- **Path:** `native/engine/src/neovim/`
- **What it does:** Embedded Neovim instance: `config`, `process` (spawn/manage nvim binary), `state`.
- **Why it doesn't belong:** Application-specific editor-runtime integration; not a framework primitive. Lives in core crate.
- **Suggested home:** `@bettertui/plugin-neovim` crate.
- **Migration priority:** High
- **Dependencies to untangle:** `pty`, `process`, `events`. Bounded to PTY layer — extractable.
- **Estimated effort:** Medium

### widgets/prompt_composer
- **Path:** `native/engine/src/widgets/prompt_composer.rs`
- **What it does:** `PromptComposer` with command history (`history`, `history_index`) — an AI prompt/composer widget, not a generic text input.
- **Why it doesn't belong:** Named "prompt composer" with history semantics tied to conversational UX; exported publicly.
- **Suggested home:** app crate alongside chat.
- **Migration priority:** Medium
- **Dependencies to untangle:** `events`, `tree::style/layout`, `WidgetContext`. Self-contained.
- **Estimated effort:** Small

### widgets/code_widget (borderline)
- **Path:** `native/engine/src/widgets/code_widget.rs`
- **What it does:** `CodeWidget` for displaying code (optionally syntax-highlighted).
- **Why it doesn't belong (partially):** Generic display is framework-appropriate; syntax highlighting coupling is an extension.
- **Suggested home:** keep generic widget in framework; move syntax layer to extension.
- **Migration priority:** Medium
- **Dependencies to untangle:** `Widget`, `tree`.
- **Estimated effort:** Small

---

## 6. Dead Code Detail

| File | Evidence | Last Modified | Recommendation |
|------|----------|---------------|----------------|
| src/editor/mod.rs | Only `pub struct Editor;`, never constructed/referenced outside `lib.rs` | — | Remove module + `pub mod editor` from lib.rs |

---

## 7. Duplicate Systems Detail

| Problem | Module A | Module B | Resolution |
|---------|----------|----------|------------|
| Text rendering overlap | widgets/text_widget | widgets/textarea_widget | Verify roles; text = display, textarea = edit. Keep distinct if intentional |
| ANSI emission path | ansi/AnsiEncoder | renderer/AnsiBackend | Confirm single canonical emitter; avoid dual paths |

---

## 8. Dependency Audit

### Rust Dependencies

All declared deps in `native/engine/Cargo.toml` are used (taffy, crossterm, ropey, unicode-width, unicode-segmentation, tracing, tracing-subscriber, parking_lot, slotmap, smallvec, bitflags, portable-pty). No unused crate found.

| Crate | Used? | Version | Issues |
|-------|-------|---------|--------|
| taffy | Yes | workspace | — |
| crossterm | Yes | workspace | — |
| ropey | Yes | workspace | — |
| unicode-width / -segmentation | Yes | workspace | — |
| tracing / tracing-subscriber | Yes | workspace | — |
| parking_lot | Yes | workspace | — |
| slotmap | Yes | workspace | — |
| smallvec | Yes | workspace | — |
| bitflags | Yes | workspace | — |
| portable-pty | Yes | workspace | — |

### TypeScript Dependencies

| Package | Dependency | Used? | Direction OK? | Issues |
|---------|-----------|-------|---------------|--------|
| @bettertui/core | @bettertui/shared | Yes | Yes | OK |
| @bettertui/native | @bettertui/shared | Yes | Yes | OK |
| @bettertui/react | core, reconciler, shared | Yes | Yes | OK |
| @bettertui/reconciler | @bettertui/core, @bettertui/shared | **core NOT imported** | N/A | ⚠️ Lists `@bettertui/core` but only imports from `@bettertui/shared` — dead dep |
| @bettertui/widgets | @bettertui/core, @bettertui/shared | **core NOT imported** | N/A | ⚠️ Lists `@bettertui/core` but imports nothing from it — dead dep |
| @bettertui/themes | @bettertui/shared | Yes | Yes | OK |

### Circular Dependencies

None detected. `shared` is the leaf; `core` re-exports; `react`/`reconciler`/`widgets` depend on `shared`/`core`. Direction is correct.

---

## 9. Architecture Violations

| # | Violation | Location | Why It's Wrong | Fix |
|---|-----------|----------|----------------|-----|
| 1 | AI chat UI in core engine | src/widgets/chat | Application layer (Category C) inside framework crate | Migrate to app crate |
| 2 | Embedded Neovim runtime in core | src/neovim | Application-specific editor integration in framework | Extract to plugin crate |
| 3 | AI prompt composer in core | src/widgets/prompt_composer.rs | App concern in framework | Migrate |
| 4 | Empty `Editor` stub shipped | src/editor | Dead/abandoned code in lib.rs | Remove |
| 5 | Broken integration test shipped | tests/integration_test.rs | References removed `RenderScheduler` + renamed `ansi_data`; won't compile | Fix or quarantine |
| 6 | Dead TS deps | reconciler, widgets package.json | List `@bettertui/core` but don't import it | Remove from package.json |
| 7 | IDE-as-platform intent documented in core | docs/architecture/IDE_PLATFORM.md | Signals building an IDE inside the framework | Reclassify as external app |

---

## 10. Migration Candidates

### High Priority (Migrate Before v1.0)

| Module | Path | Migrate To | Effort | Blocks Release? |
|--------|------|------------|--------|-----------------|
| widgets/chat | src/widgets/chat | @bettertui/app-chat | Medium | Yes |
| neovim | src/neovim | @bettertui/plugin-neovim | Medium | Yes |

### Medium Priority (Migrate After v1.0)

| Module | Path | Migrate To | Effort |
|--------|------|------------|--------|
| widgets/prompt_composer | src/widgets/prompt_composer.rs | app crate | Small |
| widgets/code_widget (syntax layer) | src/widgets/code_widget.rs | framework ext | Small |

### Low Priority (Consider Later)

| Module | Path | Migrate To | Effort |
|--------|------|------------|--------|
| engine/inspector | src/engine/inspector.rs | keep (B), gate behind feature | Small |

---

## 11. Technical Debt

### Architecture Debt
- AI chat, Neovim, and IDE intent embedded in core framework crate.
- `editor/` stub left in `lib.rs`.

### Code Debt
- Empty `editor` stub.
- Possible ANSI emission path duplication (ansi vs renderer).

### Documentation Debt
- `IDE_PLATFORM.md` frames the framework as an IDE host — needs reframing as an external app.

### Testing Debt
- Integration test (`tests/integration_test.rs`) does not compile (E0432 `RenderScheduler`, E0609 `ansi_data`). It is currently excluded from any passing run and provides zero coverage.

### API Debt
- Publicly exporting app-layer widgets (`pub use chat::...`, `prompt_composer::...`) entrenches wrong boundaries in the public API surface.

---

## 12. Recommended Cleanup Plan

### Phase 1: Safe Cleanup (Week 1)
- Remove `src/editor` module and `pub mod editor` from `lib.rs`.
- Remove unused `@bettertui/core` from `packages/reconciler` and `packages/widgets` package.json.
- Fix or quarantine `tests/integration_test.rs` (update to current `RenderFrame`/`Renderer` API).
- Reclassify `IDE_PLATFORM.md` as an external-application design doc.

### Phase 2: Module Migration (Weeks 2-4)
- Extract `src/widgets/chat` to `@bettertui/app-chat` (or SCode crate).
- Extract `src/neovim` to `@bettertui/plugin-neovim`.
- Remove `pub use chat::...` and `neovim` from core public exports.

### Phase 3: API Cleanup (Weeks 4-6)
- Extract `prompt_composer` and syntax layer of `code_widget` to extension/app.
- Confirm single canonical ANSI emitter; remove duplicate if any.
- Add tests for markdown/code_widget if missing.

### Phase 4: Documentation (Week 6)
- Update architecture docs to reflect framework vs. application boundary.
- Update AGENTS.md (note core is `shared`, not `core`).
- Write migration guide for extracted modules.

### Phase 5: Release Preparation (Week 7)
- Final audit; confirm clippy clean and integration test passes.
- Version bump; changelog.

---

## 13. Final Assessment

### Does the repository represent the BetterTUI vision?
Partially. The engine and TS adapter are genuinely framework-grade and aligned. But the presence of AI chat, embedded Neovim, and an IDE-platform doc shows the repo has absorbed an application (SCode) instead of staying a pure TUI framework.

### Perfectly Aligned Modules
tree, protocol, layout, renderer, framebuffer, painter, dirty_diff, compositor, events, focus, pty, terminal, capabilities, text, input, keyboard, mouse, clipboard, glyph, ansi, graphics, render_object, scheduler, screen, selection, snapshot, engine/core, ffi, animation, keybinding, nerdfont, plugin, palette.

### Drifted into Application Territory
widgets/chat, neovim, widgets/prompt_composer, widgets/code_widget (syntax), editor stub, IDE_PLATFORM.md.

### Should Eventually Live Outside BetterTUI
widgets/chat (→ app crate), neovim (→ plugin crate), prompt_composer (→ app crate), IDE platform (→ separate product).

### Can BetterTUI become production-ready after cleanup?
Yes. After removing ~1,700 LOC of application code and the dead editor stub, and fixing the broken integration test, the remaining framework is clean (clippy 0 warnings, 910 tests) and releasable. The bulk of the work is extraction, not rewrites.

---

**Report Generated:** 2026-07-11 16:52
**Next Audit Recommended:** 2026-10-11
