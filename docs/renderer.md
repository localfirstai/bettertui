# Renderer

The renderer turns the arena node tree into ANSI output. It is the top of the Rust render pipeline.

## Responsibilities

```mermaid
flowchart LR
    A[NodeArena] --> B[Renderer.render]
    B --> C[layout sync]
    C --> D[build render tree]
    D --> E[paint -> FrameBuffer]
    E --> F[dirty diff]
    F --> G[AnsiBackend encode]
    G --> H[stdout]
```

See [Architecture: Rendering Pipeline](architecture/rendering-pipeline.md) for the full stage breakdown and the `Renderer`/`RenderFrame`/`RenderBackend` API.

## Pipeline pieces

- `render/` — `Renderer`, `RenderFrame`, `RenderBackend` trait, `AnsiBackend` (in `render/render.rs`); `effects.rs` post-processing passes
- `render/` — `build_render_tree`, `RenderTree` (z-sorted), `PaintContext`
- `framebuffer/` + `dirty_diff/` — rasterization + diffing
- `tree.rs` / `graphics.rs` — compositing primitives (see [Compositor](architecture/compositor.md))

## Status

Implemented and tested (~22 renderer tests). Known issues documented in the architecture pipeline doc: full-buffer clear per paint and O(n) dirty scan.
