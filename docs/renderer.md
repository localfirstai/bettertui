# Renderer

The renderer turns the arena node tree into ANSI output. It is the top of the Rust render pipeline.

## Pipeline

`NodeArena → Renderer.render → layout sync → build render tree → paint → FrameBuffer → dirty diff → AnsiBackend encode → stdout`

See [Architecture: Rendering Pipeline](architecture/rendering-pipeline.md) for the full breakdown.

## Status

Implemented and tested. Known issues: full-buffer clear per paint, O(n) dirty scan (documented in the architecture pipeline doc).
