use std::io::{self, Write};
use std::time::Duration;

use bettertui_terminal::Terminal;

use crate::util;

pub fn run() {
    util::heading("Terminal Demo: Raw Mode, Events, Drawing");

    let mut terminal = Terminal::new();
    let (w, h) = terminal.size();
    println!("Terminal size: {w}x{h}");
    println!("Entering raw mode + alternate screen. Press 'q' to quit.\n");

    // Enter raw mode and alternate screen
    if let Err(e) = terminal.enter_raw_mode() {
        eprintln!("Failed to enter raw mode: {e}");
        return;
    }
    if let Err(e) = terminal.enter_alternate_screen() {
        eprintln!("Failed to enter alternate screen: {e}");
        let _ = terminal.leave_raw_mode();
        return;
    }

    let result = run_loop(&mut terminal);

    // Cleanup on exit
    let _ = terminal.leave_alternate_screen();
    let _ = terminal.leave_raw_mode();
    let _ = terminal.show_cursor();

    if let Err(e) = result {
        eprintln!("Terminal demo error: {e}");
    }
}

fn run_loop(terminal: &mut Terminal) -> io::Result<()> {
    terminal.clear()?;
    terminal.hide_cursor()?;

    let (w, h) = terminal.size();
    let mut frame = 0u64;

    loop {
        // ── Draw frame ──
        let title = format!("BetterTUI Terminal Demo - Frame {frame}");
        let help = "Press 'q' to quit | Arrow keys move | Resize to test";

        // Draw border
        draw_border(terminal, w, h)?;

        // Draw title
        let title_x = (w.saturating_sub(title.len() as u16)) / 2;
        terminal.move_cursor(title_x, 1)?;
        write!(io::stdout(), "\x1b[1;97m{title}\x1b[0m")?;

        // Draw help
        let help_x = (w.saturating_sub(help.len() as u16)) / 2;
        terminal.move_cursor(help_x, h - 2)?;
        write!(io::stdout(), "\x1b[2;90m{help}\x1b[0m")?;

        // Draw center content
        let center_y = h / 2;
        let lines = [
            format!("Terminal: {w}x{h}"),
            format!("Frame: #{frame}"),
            String::from("  ╭───── BetterTUI ─────╮"),
            String::from("  │    Native Engine    │"),
            String::from("  │    Command Proto    │"),
            String::from("  │    Widgets Layer    │"),
            String::from("  ╰─────────────────────╯"),
        ];
        for (i, line) in lines.iter().enumerate() {
            let x = (w.saturating_sub(line.len() as u16)) / 2;
            terminal.move_cursor(x, center_y + i as u16 - 2)?;
            let style = if i == 2 { "\x1b[36m" } else { "\x1b[37m" };
            write!(io::stdout(), "{style}{line}\x1b[0m")?;
        }

        io::stdout().flush()?;

        // ── Handle events ──
        match terminal.poll_event(Duration::from_millis(50))? {
            Some(bettertui_terminal::TerminalEvent::Key(key)) => match key.code {
                bettertui_terminal::Key::Char('q') | bettertui_terminal::Key::Esc => {
                    break;
                }
                _ => {}
            },
            Some(bettertui_terminal::TerminalEvent::Resize(_nw, _nh)) => {
                let _ = terminal.refresh_size();
                let _ = terminal.clear();
            }
            _ => {}
        }

        frame += 1;
    }

    Ok(())
}

fn draw_border(terminal: &Terminal, w: u16, h: u16) -> io::Result<()> {
    if w < 2 || h < 2 {
        return Ok(());
    }
    // Top
    terminal.move_cursor(0, 0)?;
    write!(io::stdout(), "\x1b[90m┌")?;
    for _ in 1..w - 1 {
        write!(io::stdout(), "─")?;
    }
    write!(io::stdout(), "┐\x1b[0m")?;

    // Bottom
    terminal.move_cursor(0, h - 1)?;
    write!(io::stdout(), "\x1b[90m└")?;
    for _ in 1..w - 1 {
        write!(io::stdout(), "─")?;
    }
    write!(io::stdout(), "┘\x1b[0m")?;

    // Sides
    for y in 1..h - 1 {
        terminal.move_cursor(0, y)?;
        write!(io::stdout(), "\x1b[90m│\x1b[0m")?;
        terminal.move_cursor(w - 1, y)?;
        write!(io::stdout(), "\x1b[90m│\x1b[0m")?;
    }

    Ok(())
}
