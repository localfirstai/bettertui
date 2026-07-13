# Text Engine Audit — Phase 1 Deliverable

## 1. Current Pipeline Architecture

```
React Components (Text, Heading, Label, Code, Markdown, CodeBlock)
  → TSX props mapped to layout/style commands
  → TypeScript CommandBuffer.push()
  → JSON serialization
  → Rust processCommands()
  → Tree Arena (slotmap NodeId)
  → Layout (Yoga/terminal)
  → Render Tree Build (resolve styles, compute ClipBounds, cull)
  → Painter (walk RenderTree, paint background + text + borders → FrameBuffer)
  → DirtyDiff (compare old/new FrameBuffer → DirtyRegion[])
  → AnsiBackend (coalesce same-style runs, emit SGR → write to terminal)
```

## 2. What Exists

### Text Widgets
- `TextWidget`: stores `content: Box<str>`, `style: Style`, creates text node
- `LabelWidget`, `HeadingWidget`: wrappers around TextWidget with default style
- `InputWidget`: single-line input, cursor at char-index
- `TextareaWidget`: multi-line input, uses TextBuffer (ropey)
- `CodeWidget`: tree-sitter syntax highlighting, block/inline modes
- `Markdown` renderer: custom line-by-line parser → AST → widget tree

### Rendering Infrastructure
- `FrameBuffer`: SoA (CellArrays), `Cell.ch: char`, `CellAttributes` bitflags
- `Painter`: iterates chars left-to-right, paints into cells at absolute positions
- `ClipBounds`: computed from `PaintBounds + Overflow::Hidden`, coarse per-node
- `DirtyDiff`: cell-by-cell row scan, region merging (max 32 regions)
- `AnsiBackend`: run-length coalescing SGR encoder
- `AnsiEncoder`: per-cell SGR encoder (legacy)
- `RenderObject`: `text: Option<Box<str>>`, `style: ResolvedStyle`, `paint_flags`

### Style System
- `Style`: `Option<bool>` for attributes (bold, italic, underline...)
- `ResolvedStyle`: resolved `bool` values after parent chain walk
- `Color`: Named, Indexed (8-bit), Rgb (24-bit), Default
- Inheritance: `Option<Color>` per style field, `None` = inherit from parent

### TextBuffer
- Wraps `ropey::Rope`
- `len_chars()` counts Unicode scalar values, not display width
- Line access via ropey `line()` method
- No visual column → char index translation

### Unicode Capabilities
- `UnicodeCapabilities` detects Kitty/Ghostty via env vars
- No actual width calculation — stub only
- `EmojiWidth` enum exists but unused
- No `unicode-width` or `unicode-segmentation` crate in dependencies

## 3. Critical Gaps

| Capability | Status | Severity |
|---|---|---|
| Unicode width measurement | NOT IMPLEMENTED | HIGH |
| Grapheme cluster support | NOT IMPLEMENTED | HIGH |
| Emoji / ZWJ / skin tones | NOT IMPLEMENTED | HIGH |
| CJK fullwidth characters | NOT IMPLEMENTED | HIGH |
| Text wrapping (word-wrap) | NOT IMPLEMENTED | HIGH |
| Text wrapping (char-wrap) | NOT IMPLEMENTED | HIGH |
| Text truncation + ellipsis | NOT IMPLEMENTED | HIGH |
| Visual-column cursor positioning | NOT IMPLEMENTED (char-index only) | MEDIUM |
| Wide characters in framebuffer | NOT SUPPORTED (char per cell) | HIGH |
| Paragraph widget (auto-wrap) | DOES NOT EXIST | HIGH |
| Bidirectional text | NOT CONSIDERED | LOW |
| Inline markdown formatting | MINIMAL (no inline bold/italic/code) | MEDIUM |
| CommonMark compliance | NOT COMPLIANT (custom parser) | MEDIUM |
| Selection rendering | NOT IMPLEMENTED | MEDIUM |
| Tab expansion | NOT IMPLEMENTED | LOW |

---

## 4. OpenTUI Comparison

### Architecture Comparison

| Aspect | BetterTUI | OpenTUI |
|---|---|---|
| Native layer | Rust | Zig |
| Text buffer | ropey::Rope (Rust crate) | Custom UnifiedRope (Zig) |
| Unicode width | None | `uucode` library with 3 methods (wcwidth, unicode, no_zwj) |
| Grapheme handling | None | GraphemePool: slab-allocated, interned, refcounted |
| Cell storage | `char` per cell (single u32 scalar) | u32 with encoding: direct scalar / grapheme ID / continuation |
| Wrap modes | None | `none`, `char`, `word` |
| Truncation | None | Boolean + ellipsis (`…`) |
| Markdown parser | Custom, non-CommonMark | `marked` library (CommonMark compliant) |
| Style representation | `Option<bool>` fields on Style struct | `TextAttributes` bitflags (u32) |
| Text measurement | None | Zig `utf8.zig` with full width tables |
| FrameBuffer type | SoA (CharArrays) | Not directly comparable |
| ANSI generation | Rust AnsiBackend (run-length) | Zig `ansi.zig` |
| Selection | Not implemented | Full: TextSelection, LocalSelection, selection rendering |
| Scroll containers | Viewport culling via binary search | TextBufferView with virtual lines, viewport |
| Cursor | char-index in InputWidget | grapheme-based cursor positioning |
| Tab handling | None | Configurable tab width + tab indicator rendering |
| Link rendering | Not implemented | Link pool, link tracker, `detect-links.ts` |

### Text Rendering Flow: OpenTUI

```
React <Text> component
  → TextRenderable (creates RootTextNodeRenderable)
  → RootTextNodeRenderable gathers chunks with inherited styles
  → StyledText (array of TextChunks: text + fg + bg + attributes)
  → TextBuffer.setStyledText(chunks)
    → Zig UnifiedTextBuffer: stores rope, computes grapheme info, wrap offsets
  → TextBufferView: computes virtual lines (wrapped), caches line starts/widths
  → TextBufferRenderable.render()
    → buffer.drawTextBuffer(view)  // Zig native render
    → Zig walks virtual lines, maps viewport, renders grapheme-by-grapheme
    → ANSI output optimized per virtual line
```

### Key OpenTUI Innovations

1. **GraphemePool**: Slab allocator with 5 size classes (8, 16, 32, 64, 128 bytes). Interns grapheme byte sequences. Reference counted. Enables O(1) grapheme width lookup.

2. **u32 char encoding**: Each framebuffer cell is a u32. Top 2 bits encode type:
   - `00` = direct Unicode scalar value (30 bits)
   - `10` = grapheme start cell with pool ID (26 bits payload)
   - `11` = continuation cell marker for wide/grapheme rendering
   - Extent fields (bits 29:28, 27:26) encode left/right rendering extent for wide chars

3. **Virtual lines**: TextBufferView computes virtual (wrapped) lines from source lines. Each virtual line stores chunk references, column offsets, truncation state with ellipsis position. Caches line starts, widths, sources, wrap indices.

4. **Three width methods**: wcwidth (fast, basic), unicode (full Unicode width tables), no_zwj (ignore ZWJ sequences for terminals that render emoji as separate chars).

5. **Wrap break detection**: In `utf8.zig`:
   - ASCII wrap breaks: space, tab, hyphen, slash, punctuation
   - Unicode wrap breaks: NBSP, various spaces, soft hyphen, CJK punctuation
   - Word classes: ASCII vs CJK boundary tracking
   - Line break result with break kinds (LF, CR, CRLF)

6. **Lazy caching**: Grapheme info and wrap offsets are computed lazily per TextChunk and cached. ASCII-only chunks skip grapheme computation entirely.

---

## 5. Weakness Analysis

### HIGH Priority

1. **No Unicode width** — Every text measurement is wrong for non-ASCII. CJK chars (width 2), emoji (width 2), combining marks (width 0), zero-width joiners all break. The entire rendering pipeline assumes `1 char = 1 column`.

2. **No grapheme clusters** — Emoji sequences (👨‍👩‍👧, 🇺🇳, 👋🏽) are stored as multiple chars, but the pipeline renders each char as a separate cell. This breaks visual rendering and cursor movement.

3. **No text wrapping** — Any text wider than its container overflows. No word-wrap, char-wrap, or soft line breaks. The ParagraphWidget from design docs doesn't exist.

4. **No truncation** — Overflowing text is either clipped (coarse ClipBounds) or overflows. No ellipsis indicator.

5. **Cell is single char** — The framebuffer cannot represent wide characters. A CJK character or emoji would write to one cell and corrupt column alignment. No continuation/placeholder cell concept.

### MEDIUM Priority

6. **Custom markdown parser** — Not CommonMark compliant. No inline formatting (bold, italic, code spans). Limited table support. Fails on edge cases.

7. **Char-index cursor** — InputWidget and TextareaWidget use char index for cursor position. Moving through wide chars or combining sequences produces wrong visual position.

8. **No selection** — Users cannot select text. No highlight/selection rendering in the painter or framebuffer.

9. **No tab expansion** — Tabs render as literal `\t` without column alignment.

10. **Styling bits are additive** — `Option<bool>` style inheritance works but resolution happens in `build.rs` during render tree construction, not during paint. This prevents dynamic style changes in leaf nodes.

### LOW Priority

11. **No bidirectional text** — RTL scripts render left-to-right incorrectly. Requires ICU or similar.

12. **No link rendering** — No clickable links, no underline decoration for URLs.

13. **AnsiBackend per-cell compare** — Run-length coalescing is good, but it compares every cell. For large unchanged regions, this is wasted work (DirtyDiff should already filter this).

---

## 6. Recommendations

### Phase 2 — Unicode Foundation
1. Add `unicode-width` crate dependency
2. Add `unicode-segmentation` crate dependency
3. Create `measurement.rs` module: `fn str_width(s: &str) -> usize`, `fn grapheme_width(g: &str) -> usize`
4. Create `GraphemeCursor` for grapheme-aware traversal
5. Add width tables for East Asian Width, emoji, and special characters
6. Tab expansion: `fn expand_tabs(s: &str, tab_width: u8) -> String`

### Phase 3 — Width Calculation
1. Extend `Cell` to support wide chars (u32 with type tag, or separate WideCell type)
2. Extend `FrameBuffer` to handle multi-column cells (placeholder cells)
3. Add width caching per string/slice
4. Handle Nerd Font, Powerline, box drawing characters (width 1)
5. Handle emoji width variations by terminal capability

### Phase 4 — Line Breaking
1. Create `LineBreaker` struct with `wrap_mode: enum { None, Char, Word }`
2. Implement word-wrapping at grapheme boundaries
3. Implement char-wrapping (hard break at width limit)
4. Implement ellipsis truncation at content_rect boundary
5. Whitespace preservation and indentation handling
6. Soft wrap (favor break points: spaces, hyphens, CJK boundaries)

### Phase 5 — Text Layout
1. Create `TextLayouter` that produces laid-out lines from raw text + width constraint
2. Support left/center/right alignment
3. Support padding/margin interaction
4. Scroll container integration (virtual lines from laid-out output)

Then continue through Phases 6-13 per the task specification.
