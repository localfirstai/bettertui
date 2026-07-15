use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use bettertui_engine::ansi::{CsiCommand, CursorMovement, EraseMode, ForegroundColor, ParserEvent, SgrAttribute};
use bettertui_engine::framebuffer::Cell;
use bettertui_engine::terminal::ScrollbackBuffer;
use bettertui_engine::terminal::{Cursor, CursorState, ScreenState, Terminal, TerminalMode, VtMachine};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn make_vt(width: u16, height: u16) -> VtMachine {
    VtMachine::new(width, height)
}

fn make_scrollback(capacity: usize) -> ScrollbackBuffer {
    let mut sb = ScrollbackBuffer::new();
    for i in 0..capacity {
        let text: String = format!("scrollback line {i} with some content for realism");
        let cells: Vec<Cell> = text.chars().map(Cell::new).collect();
        sb.push_line(cells, 80, true);
    }
    sb
}

fn make_csi_position(col: u32, row: u32) -> ParserEvent {
    ParserEvent::Csi(CsiCommand::CursorMovement(CursorMovement::Position(col, row)))
}

fn make_csi_sgr(fg: ForegroundColor) -> ParserEvent {
    ParserEvent::Csi(CsiCommand::Sgr(vec![SgrAttribute::Foreground(fg)]))
}

fn make_csi_erase() -> ParserEvent {
    ParserEvent::Csi(CsiCommand::Erase(EraseMode::Entire))
}

// ─── Terminal Construction ────────────────────────────────────────────────────

fn bench_terminal_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("terminal/construction");

    group.bench_function("new_default", |b| {
        b.iter(|| {
            let t = Terminal::new();
            black_box(t.size());
        });
    });

    group.bench_function("size_query", |b| {
        let t = Terminal::new();
        b.iter(|| {
            let (w, h) = t.size();
            black_box((w, h));
        });
    });

    group.finish();
}

// ─── VtMachine Benchmarks ────────────────────────────────────────────────────

fn bench_vt_machine(c: &mut Criterion) {
    let mut group = c.benchmark_group("terminal/vt_machine");

    group.bench_function("new_80x24", |b| {
        b.iter(|| {
            let vm = VtMachine::new(80, 24);
            black_box(vm.current_screen().width());
        });
    });

    group.bench_function("new_120x40", |b| {
        b.iter(|| {
            let vm = VtMachine::new(120, 40);
            black_box(vm.current_screen().width());
        });
    });

    group.bench_function("resize", |b| {
        let mut vm = make_vt(80, 24);
        b.iter(|| {
            vm.resize(black_box(120), black_box(40));
            black_box(vm.current_screen().width());
        });
    });

    group.bench_function("process_chars_100", |b| {
        let mut vm = make_vt(80, 24);
        let text = b"Hello, BetterTUI World! This is a test of the VT machine.\n";
        let events: Vec<ParserEvent> = text.iter().map(|&b| ParserEvent::Char(b)).collect();
        b.iter(|| {
            for event in &events {
                vm.process(black_box(event));
            }
            black_box(vm.current_cursor().position());
        });
    });

    group.bench_function("process_csi_sequences", |b| {
        let mut vm = make_vt(80, 24);
        let csi_events = vec![
            make_csi_position(10, 20),
            make_csi_erase(),
            make_csi_sgr(ForegroundColor::Extended(196)),
            ParserEvent::Csi(CsiCommand::CursorMovement(CursorMovement::Up(5))),
            ParserEvent::Csi(CsiCommand::CursorMovement(CursorMovement::Down(3))),
        ];
        b.iter(|| {
            for event in &csi_events {
                vm.process(black_box(event));
            }
            black_box(vm.current_cursor().position());
        });
    });

    group.bench_function("process_mixed_stream", |b| {
        let mut vm = make_vt(80, 24);
        let mut events = Vec::new();
        for i in 0..50 {
            if i % 5 == 0 {
                events.push(make_csi_sgr(ForegroundColor::Extended(31 + i as u8 % 6)));
            }
            for &b in b"text content " {
                events.push(ParserEvent::Char(b));
            }
            if i % 10 == 9 {
                events.push(ParserEvent::LineFeed);
            }
        }
        b.iter(|| {
            for event in &events {
                vm.process(black_box(event));
            }
            black_box(vm.current_screen().buffer().get(0, 0).ch);
        });
    });

    group.bench_function("line_feeds_with_scroll", |b| {
        let mut vm = make_vt(80, 24);
        b.iter(|| {
            for _ in 0..100 {
                vm.process(&ParserEvent::LineFeed);
            }
            black_box(vm.current_cursor().position());
        });
    });

    group.finish();
}

// ─── Cursor Benchmarks ────────────────────────────────────────────────────────

fn bench_cursor(c: &mut Criterion) {
    let mut group = c.benchmark_group("terminal/cursor");

    group.bench_function("cursor_new", |b| {
        b.iter(|| {
            let c = Cursor::new();
            black_box(c.position());
        });
    });

    group.bench_function("cursor_move_operations", |b| {
        let mut c = Cursor::new();
        b.iter(|| {
            c.move_to(10, 20);
            c.move_up(3);
            c.move_down(5, 24);
            c.move_left(2);
            c.move_right(8, 80);
            c.save_position();
            c.restore_position();
            c.carriage_return();
            c.newline();
            black_box(c.position());
        });
    });

    group.bench_function("cursor_state_new", |b| {
        b.iter(|| {
            let cs = CursorState::new();
            black_box((cs.x(), cs.y()));
        });
    });

    group.bench_function("cursor_state_operations", |b| {
        let mut cs = CursorState::new();
        b.iter(|| {
            cs.set_position(15, 30);
            cs.hide();
            cs.show();
            cs.set_style(bettertui_engine::terminal::screen::CursorStyle::Bar);
            black_box(cs.visible());
        });
    });

    group.finish();
}

// ─── ScreenState Benchmarks ──────────────────────────────────────────────────

fn bench_screen_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("terminal/screen_state");

    group.bench_function("new", |b| {
        b.iter(|| {
            let ss = ScreenState::new();
            black_box(ss.buffer_size());
        });
    });

    group.bench_function("with_size_120x40", |b| {
        b.iter(|| {
            let ss = ScreenState::with_size(120, 40);
            black_box(ss.buffer_size());
        });
    });

    group.bench_function("resize", |b| {
        let mut ss = ScreenState::new();
        b.iter(|| {
            ss.resize(black_box(120), black_box(40));
            black_box(ss.buffer_size());
        });
    });

    group.bench_function("scroll_up_down", |b| {
        let mut ss = ScreenState::new();
        b.iter(|| {
            ss.scroll_up(5);
            ss.scroll_down(3);
            ss.scroll_reset();
            black_box(ss.buffer_size());
        });
    });

    group.bench_function("alternate_screen_cycle", |b| {
        let mut ss = ScreenState::new();
        b.iter(|| {
            ss.enter_alternate_screen();
            ss.leave_alternate_screen();
            black_box(ss.is_alternate_screen());
        });
    });

    group.bench_function("selection_cycle", |b| {
        let mut ss = ScreenState::new();
        b.iter(|| {
            ss.set_selection((0, 0), (40, 12));
            ss.clear_selection();
            black_box(ss.selection_active());
        });
    });

    group.finish();
}

// ─── ScrollbackBuffer Benchmarks ──────────────────────────────────────────────

fn bench_scrollback(c: &mut Criterion) {
    let mut group = c.benchmark_group("terminal/scrollback_vt");

    group.bench_function("new", |b| {
        b.iter(|| {
            let sb = ScrollbackBuffer::new();
            black_box(sb.len());
        });
    });

    group.bench_function("push_line_small", |b| {
        let mut sb = ScrollbackBuffer::new();
        let cells: Vec<Cell> = b"Hello, World!".iter().map(|&b| Cell::new(b as char)).collect();
        b.iter(|| {
            sb.push_line(cells.clone(), 80, true);
            black_box(sb.len());
        });
    });

    for count in [100, 1000] {
        group.bench_with_input(BenchmarkId::new("push_bulk", count), &count, |b, &count| {
            b.iter_with_setup(ScrollbackBuffer::new, |mut sb| {
                for i in 0..count {
                    let text = format!("line {i} with some filler text for realism");
                    let cells: Vec<Cell> = text.chars().map(Cell::new).collect();
                    sb.push_line(cells, 80, true);
                }
                black_box(sb.len());
            });
        });
    }

    group.bench_function("line_lookup", |b| {
        let sb = make_scrollback(500);
        b.iter(|| {
            for i in 0..10 {
                let line = sb.line(i);
                black_box(line.map(|l| l.len()));
            }
        });
    });

    group.bench_function("truncate_on_overflow", |b| {
        b.iter_with_setup(ScrollbackBuffer::new, |mut sb| {
            for i in 0..200 {
                let text = format!("overflow line {i}");
                let cells: Vec<Cell> = text.chars().map(Cell::new).collect();
                sb.push_line(cells, 80, true);
            }
            black_box(sb.len());
        });
    });

    group.bench_function("clear", |b| {
        let mut sb = make_scrollback(500);
        b.iter(|| {
            sb.clear();
            black_box(sb.len());
        });
    });

    group.finish();
}

// ─── TerminalMode Benchmarks ─────────────────────────────────────────────────

fn bench_terminal_mode(c: &mut Criterion) {
    let mut group = c.benchmark_group("terminal/mode");

    group.bench_function("default", |b| {
        b.iter(|| {
            let mode = TerminalMode::default();
            black_box(mode.bits());
        });
    });

    group.bench_function("toggle_flags", |b| {
        let mut mode = TerminalMode::default();
        b.iter(|| {
            mode.insert(TerminalMode::INSERT);
            mode.remove(TerminalMode::AUTO_WRAP);
            mode.toggle(TerminalMode::ALT_SCREEN);
            black_box(mode.auto_wrap());
        });
    });

    group.bench_function("read_all_properties", |b| {
        let mode = TerminalMode::all();
        b.iter(|| {
            let _ = black_box(mode.is_insert());
            let _ = black_box(mode.auto_wrap());
            let _ = black_box(mode.alt_screen());
            let _ = black_box(mode.bracketed_paste());
            let _ = black_box(mode.focus_events());
            let _ = black_box(mode.cursor_visible());
        });
    });

    group.finish();
}

// ─── TerminalState Benchmarks ─────────────────────────────────────────────────

fn bench_terminal_state(c: &mut Criterion) {
    use bettertui_engine::terminal::{ProcessStatus, TerminalState};

    let mut group = c.benchmark_group("terminal/state");

    group.bench_function("new", |b| {
        b.iter(|| {
            let ts = TerminalState::new();
            black_box(ts.is_running());
        });
    });

    group.bench_function("status_transitions", |b| {
        let mut ts = TerminalState::new();
        b.iter(|| {
            ts.mark_started(42);
            black_box(ts.is_running());
            ts.mark_exited(0);
            black_box(ts.is_exited());
            ts.reset();
            black_box(ts.is_running());
        });
    });

    group.bench_function("process_status_checks", |b| {
        let statuses = [
            ProcessStatus::Stopped,
            ProcessStatus::Running,
            ProcessStatus::Exited(0),
            ProcessStatus::Signaled(9),
            ProcessStatus::Error,
        ];
        b.iter(|| {
            for s in &statuses {
                let _ = black_box(s.is_running());
                let _ = black_box(s.is_exited());
                let _ = black_box(s.exit_code());
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_terminal_construction,
    bench_vt_machine,
    bench_cursor,
    bench_screen_state,
    bench_scrollback,
    bench_terminal_mode,
    bench_terminal_state,
);
criterion_main!(benches);
