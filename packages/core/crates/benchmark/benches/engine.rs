//! End-to-end engine benchmarks covering the full public API surface:
//! text editing, framebuffer, VT parsing/machine, rendering pipeline,
//! graphics, fonts, syntax highlighting, and the high-level Engine.

use criterion::{Criterion, black_box, criterion_group, criterion_main};

use bettertui_engine::ansi::AnsiParser;
use bettertui_engine::engine::{Engine, Inspector};
use bettertui_engine::font::{FontMetrics, FontMetricsCache, FontProvider, measure_text};
use bettertui_engine::framebuffer::{Cell, CellAttributes, FrameBuffer};
use bettertui_engine::graphics::{DrawStyle, GraphicsContext, Rect};
use bettertui_engine::protocol::Command;
use bettertui_engine::pty::{PtyConfig, PtyReader, PtySize, PtyWriter};
use bettertui_engine::render::effects::{
    BrightnessPass, GrayscalePass, InvertPass, RainbowPass, VignettePass,
};
use bettertui_engine::render::{
    AnsiBackend, RenderBackend, RenderPass, RenderPassContext, RenderPipeline,
};
use bettertui_engine::syntax::SyntaxHighlighter;
use bettertui_engine::text::{
    EditBuffer, SelectionRange, StyledText, TextAlign, TextEngine, ViewportConfig, layout_text,
};
use bettertui_engine::tree::{Color, NamedColor, NodeArena, NodeId, NodeKind, RenderNode, Style};

const SAMPLE_TEXT: &str = "fn main() {\n    println!(\"Hello, BetterTUI!\");\n}\n";

fn build_sample_tree(arena: &mut NodeArena, depth: usize, width: usize) -> Vec<NodeId> {
    let mut ids = Vec::new();
    let root = arena.root();
    for _ in 0..width {
        let child = arena.insert(RenderNode::text("label"));
        let _ = arena.append_child(root, child);
        ids.push(child);
    }
    if depth > 0 {
        for &id in &ids.clone() {
            for _ in 0..width {
                let child = arena.insert(RenderNode::new(NodeKind::Box));
                let _ = arena.append_child(id, child);
                ids.push(child);
            }
            if depth == 1 {
                break;
            }
        }
    }
    ids
}

fn bench_text_engine(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_text");

    group.bench_function("insert_chars", |b| {
        b.iter(|| {
            let mut engine = TextEngine::new();
            for ch in black_box(SAMPLE_TEXT).chars() {
                engine.insert_char(ch);
            }
            black_box(engine.text())
        })
    });

    group.bench_function("insert_str_large", |b| {
        let big = "line of text\n".repeat(500);
        b.iter(|| {
            let mut engine = TextEngine::new();
            engine.insert_str(black_box(&big));
            black_box(engine.line_count())
        })
    });

    group.bench_function("undo_redo", |b| {
        b.iter(|| {
            let mut engine = TextEngine::new();
            for ch in "hello world".chars() {
                engine.insert_char(ch);
            }
            let _ = engine.undo();
            let _ = engine.redo();
            black_box(engine.text())
        })
    });

    group.bench_function("search", |b| {
        let haystack = "needle in a haystack\nneedle again\n".repeat(200);
        let mut engine = TextEngine::with_text(&haystack);
        b.iter(|| {
            let results = engine.search(
                black_box("needle"),
                bettertui_engine::text::SearchOptions::default(),
            );
            black_box(results.len())
        })
    });

    group.bench_function("replace_all", |b| {
        let haystack = "alpha beta alpha gamma alpha\n".repeat(300);
        b.iter(|| {
            let mut engine = TextEngine::with_text(black_box(&haystack));
            let count = engine.replace(
                "alpha",
                "X",
                bettertui_engine::text::SearchOptions::default(),
            );
            black_box(count)
        })
    });

    group.bench_function("edit_buffer_with_config", |b| {
        b.iter(|| {
            let mut buf = EditBuffer::new();
            buf.insert_str(black_box(SAMPLE_TEXT));
            let _ = buf.undo();
            black_box(buf.content())
        })
    });

    group.bench_function("selection_and_range", |b| {
        b.iter(|| {
            let range = SelectionRange::new(2, 10);
            black_box(range.len())
        })
    });

    group.bench_function("styled_text_merge", |b| {
        b.iter(|| {
            let mut st = StyledText::new();
            for i in 0..50 {
                st.push_styled(
                    format!("word{i} "),
                    Style::default().fg(Color::Named(NamedColor::Red)),
                );
            }
            st.merge_adjacent_with_same_style();
            black_box(st.spans.len())
        })
    });

    group.bench_function("styled_text_truncate", |b| {
        b.iter(|| {
            let st = StyledText::from(black_box(SAMPLE_TEXT));
            black_box(st.truncate_to_width(20))
        })
    });

    group.bench_function("layout_text_wrap", |b| {
        b.iter(|| {
            let config = ViewportConfig {
                wrap: true,
                align: TextAlign::Left,
                max_width: 80,
                max_height: 24,
                ..ViewportConfig::default()
            };
            let layout = layout_text(black_box(SAMPLE_TEXT), &config);
            black_box(layout.lines.len())
        })
    });

    group.finish();
}

fn bench_framebuffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_framebuffer");

    group.bench_function("write_str_full", |b| {
        b.iter(|| {
            let mut fb = FrameBuffer::new(80, 24);
            for y in 0..24u16 {
                fb.write_str(
                    0,
                    y,
                    black_box("The quick brown fox jumps over the lazy dog"),
                    Color::Default,
                    Color::Default,
                );
            }
            black_box(fb.get(0, 0).ch)
        })
    });

    group.bench_function("diff_full_screen", |b| {
        let mut fb = FrameBuffer::new(80, 24);
        fb.write_str(
            0,
            0,
            "baseline content for diffing",
            Color::Default,
            Color::Default,
        );
        let mut other = FrameBuffer::new(80, 24);
        other.write_str(
            0,
            0,
            "baseline content for diffing!",
            Color::Default,
            Color::Default,
        );
        b.iter(|| black_box(fb.diff()))
    });

    group.bench_function("process_cells_with_color", |b| {
        b.iter(|| {
            let mut fb = FrameBuffer::new(80, 24);
            fb.process_cells(|_, _, cell| {
                cell.fg = Color::Named(NamedColor::Cyan);
                cell.attributes = CellAttributes::BOLD;
            });
            black_box(fb.cells().len())
        })
    });

    group.bench_function("fill_rect", |b| {
        b.iter(|| {
            let mut fb = FrameBuffer::new(80, 24);
            fb.fill_rect(
                0,
                0,
                80,
                24,
                Cell::new('#').with_fg(Color::Named(NamedColor::Red)),
            );
            black_box(fb.get(40, 12).ch)
        })
    });

    group.finish();
}

fn bench_vt(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_vt");

    let csi_stream =
        b"\x1b[2J\x1b[1;1H\x1b[31mHello\x1b[0m\x1b[2;1HWorld\x1b[?25h\x1b[3;3HXY\x1b[1;1H";

    group.bench_function("parse_csi_stream", |b| {
        b.iter(|| {
            let mut parser = AnsiParser::new();
            parser.feed(black_box(csi_stream));
            let mut count = 0;
            while parser.poll_event().is_some() {
                count += 1;
            }
            black_box(count)
        })
    });

    group.bench_function("framebuffer_write_str", |b| {
        let mut fb = FrameBuffer::new(80, 24);
        b.iter(|| {
            fb.write_str(
                5,
                5,
                black_box("Hello, world!"),
                Color::Named(NamedColor::Green),
                Color::Default,
            );
            black_box(fb.get(5, 5).ch)
        })
    });

    group.bench_function("framebuffer_diff", |b| {
        let mut fb = FrameBuffer::new(80, 24);
        fb.write_str(
            0,
            0,
            "the quick brown fox",
            Color::Named(NamedColor::White),
            Color::Default,
        );
        fb.swap();
        fb.write_str(
            0,
            0,
            "the quick brown dog",
            Color::Named(NamedColor::White),
            Color::Default,
        );
        b.iter(|| black_box(fb.diff().len()))
    });

    group.bench_function("framebuffer_resize", |b| {
        b.iter(|| {
            let mut fb = FrameBuffer::new(80, 24);
            fb.resize(120, 30);
            black_box(fb.get(0, 0).ch)
        })
    });

    group.bench_function("framebuffer_fill_rect", |b| {
        b.iter(|| {
            let mut fb = FrameBuffer::new(80, 24);
            fb.fill_rect(
                0,
                0,
                80,
                24,
                Cell::new('#').with_fg(Color::Named(NamedColor::Red)),
            );
            black_box(fb.get(40, 12).ch)
        })
    });

    group.bench_function("framebuffer_clear", |b| {
        b.iter(|| {
            let mut fb = FrameBuffer::new(80, 24);
            fb.fill_rect(
                0,
                0,
                80,
                24,
                Cell::new('#').with_fg(Color::Named(NamedColor::Red)),
            );
            fb.swap();
            fb.clear();
            black_box(fb.get(0, 0).ch)
        })
    });

    group.finish();
}

fn bench_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_render");

    group.bench_function("painter_paint_tree", |b| {
        let mut arena = NodeArena::new();
        let ids = build_sample_tree(&mut arena, 2, 10);
        let _ = ids;
        let mut layout = bettertui_engine::layout::LayoutTreeSync::new();
        layout.sync_full(&arena);
        let _ = layout.compute(arena.root(), 80, 24);
        let tree = bettertui_engine::layout::build_render_tree(&arena, layout.results());
        b.iter(|| {
            let mut painter = bettertui_engine::render::Painter::new(80, 24);
            let ctx = bettertui_engine::layout::PaintContext::new(80, 24);
            painter.paint(&tree, &ctx);
            black_box(painter.diff().len())
        })
    });

    group.bench_function("ansi_backend_encode", |b| {
        let mut fb = FrameBuffer::new(80, 24);
        for y in 0..24u16 {
            fb.write_str(
                0,
                y,
                "rendered line with color",
                Color::Named(NamedColor::Green),
                Color::Default,
            );
        }
        let empty = FrameBuffer::new(80, 24);
        let mut dirty = bettertui_engine::dirty_diff::DirtyDiff::new();
        let regions = dirty.compute(&fb, &empty, 1).to_vec();
        let mut backend = AnsiBackend::new();
        b.iter(|| {
            backend.encode(black_box(&fb), &regions);
            black_box(backend.finish().len())
        })
    });

    group.bench_function("pipeline_execute_passes", |b| {
        let mut fb = FrameBuffer::new(80, 24);
        for y in 0..24u16 {
            fb.write_str(
                0,
                y,
                "colored content here",
                Color::Named(NamedColor::Yellow),
                Color::Default,
            );
        }
        let ctx = RenderPassContext::new(80, 24);
        let mut pipeline = RenderPipeline::new();
        pipeline.add_pass(Box::new(InvertPass::new(
            bettertui_engine::render::effects::INVERT_MATRIX,
        )));
        pipeline.add_pass(Box::new(BrightnessPass::new(0.2)));
        pipeline.add_pass(Box::new(GrayscalePass::new(
            bettertui_engine::render::effects::GRAYSCALE_MATRIX,
        )));
        pipeline.add_pass(Box::new(VignettePass::new()));
        b.iter(|| black_box(pipeline.execute(black_box(&mut fb.clone()), &ctx)))
    });

    group.bench_function("rainbow_pass", |b| {
        let mut fb = FrameBuffer::new(80, 24);
        for y in 0..24u16 {
            fb.write_str(
                0,
                y,
                "rainbow colored text content",
                Color::Named(NamedColor::Magenta),
                Color::Default,
            );
        }
        let ctx = RenderPassContext::new(80, 24);
        let mut pass = RainbowPass::new();
        b.iter(|| black_box(pass.execute(black_box(&mut fb.clone()), &ctx)))
    });

    group.finish();
}

fn bench_graphics(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_graphics");

    group.bench_function("draw_box", |b| {
        b.iter(|| {
            let mut fb = FrameBuffer::new(100, 40);
            let mut ctx = GraphicsContext::new(&mut fb);
            ctx.draw_box(
                Rect::new(1, 1, 60, 20),
                &DrawStyle::new().fg(Color::Named(NamedColor::Cyan)),
            );
            black_box(ctx.buffer().get(1, 1).ch)
        })
    });

    group.bench_function("fill_rect_with_style", |b| {
        b.iter(|| {
            let mut fb = FrameBuffer::new(100, 40);
            let mut ctx = GraphicsContext::new(&mut fb);
            ctx.fill_rect(
                Rect::new(0, 0, 100, 40),
                '#',
                &DrawStyle::new().bg(Color::Named(NamedColor::Blue)),
            );
            black_box(ctx.buffer().get(50, 20).ch)
        })
    });

    group.bench_function("draw_str_gradient", |b| {
        b.iter(|| {
            let mut fb = FrameBuffer::new(100, 40);
            let mut ctx = GraphicsContext::new(&mut fb);
            for y in 0..40u16 {
                ctx.draw_str(
                    0,
                    y,
                    "gradient line",
                    &DrawStyle::new().fg(Color::rgb((y * 6) as u8, 0, 0)),
                );
            }
            black_box(ctx.buffer().get(0, 39).ch)
        })
    });

    group.finish();
}

fn bench_font(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_font");

    group.bench_function("provider_new", |b| {
        b.iter(|| {
            let provider = FontProvider::new();
            black_box(provider.total_icons())
        })
    });

    group.bench_function("registry_lookup_name", |b| {
        let provider = FontProvider::new();
        b.iter(|| {
            let result = provider.lookup_name(black_box("nf-fa-folder"));
            black_box(result)
        })
    });

    group.bench_function("registry_lookup_codepoint", |b| {
        let provider = FontProvider::new();
        b.iter(|| {
            let result = provider.lookup_codepoint(black_box(0xF000));
            black_box(result)
        })
    });

    group.bench_function("resolve_icon", |b| {
        let provider = FontProvider::new();
        b.iter(|| {
            let result = provider.resolve_icon(black_box("nf-fa-folder"));
            black_box(result)
        })
    });

    group.bench_function("metrics_cache_insert_get", |b| {
        let mut cache = FontMetricsCache::new(10, 20);
        let m = FontMetrics::new().with_dimensions(10, 20);
        cache.insert(0xE0A0, m);
        b.iter(|| {
            black_box(cache.get(black_box(0xE0A0)));
            black_box(cache.len())
        })
    });

    group.bench_function("metrics_cache_preload_standard", |b| {
        b.iter(|| {
            let mut cache = FontMetricsCache::new(10, 20);
            cache.preload_standard();
            black_box(cache.len())
        })
    });

    group.bench_function("ascii_measure_text", |b| {
        b.iter(|| {
            let result = measure_text(black_box("Hello World"), black_box("tiny"));
            black_box(result)
        })
    });

    group.finish();
}

fn bench_syntax(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_syntax");

    let rust_src = "fn main() {\n    let x: u32 = 42;\n    if x > 0 {\n        println!(\"{}\", x);\n    }\n}\n".repeat(40);

    group.bench_function("highlight_rust", |b| {
        let mut highlighter = SyntaxHighlighter::new();
        b.iter(|| {
            let lines = highlighter.highlight(black_box(&rust_src), "rust");
            black_box(lines.map(|l| l.len()).unwrap_or(0))
        })
    });

    let ts_src =
        "const x: number = 1;\nfunction add(a: number, b: number): number {\n  return a + b;\n}\n"
            .repeat(40);

    group.bench_function("highlight_typescript", |b| {
        let mut highlighter = SyntaxHighlighter::new();
        b.iter(|| {
            let lines = highlighter.highlight(black_box(&ts_src), "typescript");
            black_box(lines.map(|l| l.len()).unwrap_or(0))
        })
    });

    group.bench_function("resolve_language", |b| {
        let highlighter = SyntaxHighlighter::new();
        b.iter(|| black_box(highlighter.resolve_language(black_box("```rust"))))
    });

    group.finish();
}

fn bench_engine_api(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_api");

    group.bench_function("process_command_create_and_text", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            let id = engine.create_node(NodeKind::Text);
            let _ = engine.process_command(Command::SetText {
                id,
                text: "hello".into(),
            });
            black_box(engine.node_count())
        })
    });

    group.bench_function("process_batch_build_tree", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            let root = engine.arena().root();
            let mut cmds = Vec::new();
            let mut ids = Vec::new();
            for _ in 0..50 {
                let id = NodeId::default();
                cmds.push(Command::CreateNode {
                    id,
                    kind: NodeKind::Box,
                });
                ids.push(id);
            }
            let _ = engine.process_commands(cmds);
            let mut link_cmds = Vec::new();
            for id in ids {
                link_cmds.push(Command::AppendChild {
                    parent: root,
                    child: id,
                });
            }
            let result = engine.process_commands(link_cmds);
            black_box(result.is_success())
        })
    });

    group.bench_function("begin_commit_frame", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.begin_frame();
            engine.commit_frame();
            black_box(engine.frame_count())
        })
    });

    group.bench_function("inspector_tree_summary", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            for _ in 0..20 {
                let id = engine.create_node(NodeKind::Box);
                let _ = engine.process_command(Command::AppendChild {
                    parent: engine.arena().root(),
                    child: id,
                });
            }
            let inspector = Inspector::new();
            let summary = inspector.tree_summary(engine.arena());
            black_box(summary.total_nodes)
        })
    });

    group.finish();
}

fn bench_pty(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_pty");

    group.bench_function("config_build", |b| {
        b.iter(|| {
            let cfg = PtyConfig::new("bash")
                .with_args(vec!["-l".into()])
                .with_size(PtySize::new(120, 40));
            black_box(cfg.size)
        })
    });

    group.bench_function("reader_line_buffering", |b| {
        b.iter(|| {
            let mut r = PtyReader::new();
            let _ = r.read_from(&mut &b"line one\nline two\nline three\n"[..]);
            let mut lines = 0;
            while r.read_line().is_some() {
                lines += 1;
            }
            black_box(lines)
        })
    });

    group.bench_function("writer_flush", |b| {
        b.iter(|| {
            let mut w = PtyWriter::new();
            w.write(black_box(b"echo hello world\r\n"));
            let mut sink: Vec<u8> = Vec::new();
            let _ = w.flush(&mut sink);
            black_box(sink.len())
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_text_engine,
    bench_framebuffer,
    bench_vt,
    bench_render,
    bench_graphics,
    bench_font,
    bench_syntax,
    bench_engine_api,
    bench_pty,
);
criterion_main!(benches);
