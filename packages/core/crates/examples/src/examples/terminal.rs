use std::io::{self, Write};
use std::time::Duration;

use bettertui_terminal::Terminal;

pub fn run(terminal: &mut Terminal) -> io::Result<()> {
    let mut out = io::stdout();
    terminal.clear()?;
    terminal.hide_cursor()?;

    let (mut w, mut h) = (terminal.size().0, terminal.size().1);
    let mut frame = 0u64;

    loop {
        // Draw border
        draw_border(terminal, w, h)?;

        // Title
        let title = format!("BetterTUI Terminal Example - Frame {frame}");
        let title_x = (w.saturating_sub(title.len() as u16)) / 2;
        terminal.move_cursor(title_x, 1)?;
        write!(out, "\x1b[1;97m{title}\x1b[0m")?;

        // Instructions
        let help = "Press Esc to return to menu | Arrow keys move | Resize to test";
        let help_x = (w.saturating_sub(help.len() as u16)) / 2;
        terminal.move_cursor(help_x, h - 2)?;
        write!(out, "\x1b[2;90m{help}\x1b[0m")?;

        // Center content
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
            write!(out, "{style}{line}\x1b[0m")?;
        }

        out.flush()?;

        // Events
        match terminal.poll_event(Duration::from_millis(50))? {
            Some(bettertui_terminal::TerminalEvent::Key(k)) => if k.code == bettertui_terminal::Key::Esc { break; },
            Some(bettertui_terminal::TerminalEvent::Resize(_nw, _nh)) => {
                let _ = terminal.refresh_size();
                let (w2, h2) = terminal.size();
                w = w2;
                h = h2;
                terminal.clear()?;
            }
            _ => {}
        }

        frame += 1;
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}

fn draw_border(terminal: &Terminal, w: u16, h: u16) -> io::Result<()> {
    use std::io::Write;
    let mut out = io::stdout();
    if w < 2 || h < 2 { return Ok(()); }
    terminal.move_cursor(0, 0)?;
    write!(out, "\x1b[90m┌")?;
    for _ in 1..w - 1 { write!(out, "─")?; }
    write!(out, "┐\x1b[0m")?;
    terminal.move_cursor(0, h - 1)?;
    write!(out, "\x1b[90m└")?;
    for _ in 1..w - 1 { write!(out, "─")?; }
    write!(out, "┘\x1b[0m")?;
    for y in 1..h - 1 {
        terminal.move_cursor(0, y)?;
        write!(out, "\x1b[90m│\x1b[0m")?;
        terminal.move_cursor(w - 1, y)?;
        write!(out, "\x1b[90m│\x1b[0m")?;
    }
    out.flush()?;
    Ok(())
}
