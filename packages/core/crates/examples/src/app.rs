use std::io::{self, Write};

use bettertui_terminal::{Key, KeyInput, Terminal, TerminalEvent};
use crossterm::event::KeyModifiers;

use crate::theme::{self, Theme};
use crate::examples::{self, Category, Example};

const TITLE: &str = "BETTERTUI EXAMPLES";

const CATEGORY_LABELS: &[(Category, &str)] = &[
    (Category::Engine, "ENGINE"),
    (Category::Layout, "LAYOUT"),
    (Category::Styling, "STYLING"),
    (Category::Text, "TEXT"),
    (Category::Widgets, "WIDGETS"),
    (Category::Effects, "EFFECTS"),
    (Category::Terminal, "TERMINAL"),
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Focus {
    Filter,
    List,
}

pub struct App {
    theme: &'static Theme,
    examples: Vec<Example>,
    filtered: Vec<usize>,
    selected_index: usize,
    filter_text: String,
    focus: Focus,
}

impl App {
    pub fn new() -> Self {
        let all = examples::all();
        let indices: Vec<usize> = (0..all.len()).collect();
        Self {
            theme: &theme::DARK,
            examples: all,
            filtered: indices,
            selected_index: 0,
            filter_text: String::new(),
            focus: Focus::Filter,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal) -> io::Result<()> {
        terminal.enter_raw_mode()?;
        terminal.enter_alternate_screen()?;
        terminal.hide_cursor()?;

        let result = self.main_loop(terminal);

        terminal.show_cursor()?;
        let _ = terminal.leave_alternate_screen();
        let _ = terminal.leave_raw_mode();
        result
    }

    fn main_loop(&mut self, terminal: &mut Terminal) -> io::Result<()> {
        let mut out = io::stdout();
        self.draw(&mut out, terminal)?;

        loop {
            match terminal.poll_event(std::time::Duration::from_millis(80))? {
                Some(TerminalEvent::Key(k)) => {
                    if k.code == Key::Char('c') && k.modifiers.contains(KeyModifiers::CONTROL) {
                        return Ok(());
                    }
                    self.handle_key(k, terminal)?;
                    self.draw(&mut out, terminal)?;
                }
                Some(TerminalEvent::Resize(_, _)) => {
                    self.draw(&mut out, terminal)?;
                }
                _ => {}
            }
        }
    }

    fn handle_key(&mut self, key: KeyInput, terminal: &mut Terminal) -> io::Result<()> {
        match key.code {
            Key::Tab | Key::Char('\t') => {
                self.focus = if self.focus == Focus::Filter { Focus::List } else { Focus::Filter };
            }
            Key::Esc => {
                self.focus = if self.focus == Focus::Filter { Focus::List } else { Focus::Filter };
            }
            Key::Up => {
                if self.focus == Focus::List { self.move_selection(-1); }
            }
            Key::Down => {
                if self.focus == Focus::List { self.move_selection(1); }
            }
            Key::Char('k') if self.focus == Focus::List && !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.move_selection(-1);
            }
            Key::Char('j') if self.focus == Focus::List => {
                self.move_selection(1);
            }
            Key::Enter | Key::Char('\n') | Key::Char('\r') => {
                if !self.filtered.is_empty() {
                    let idx = self.filtered[self.selected_index.min(self.filtered.len() - 1)];
                    self.run_example(idx, terminal)?;
                }
            }
            Key::Char('/') if self.focus == Focus::List => {
                self.focus = Focus::Filter;
            }
            Key::Char(c) if self.focus == Focus::Filter => {
                self.filter_text.push(c);
                self.apply_filter();
            }
            Key::Backspace if self.focus == Focus::Filter => {
                self.filter_text.pop();
                self.apply_filter();
            }
            _ => {}
        }
        Ok(())
    }

    fn move_selection(&mut self, delta: isize) {
        if self.filtered.is_empty() { return; }
        let len = self.filtered.len();
        let new = (self.selected_index as isize + delta).rem_euclid(len as isize) as usize;
        self.selected_index = new;
    }

    fn apply_filter(&mut self) {
        let text = self.filter_text.to_lowercase();
        if text.is_empty() {
            self.filtered = (0..self.examples.len()).collect();
        } else {
            self.filtered = self.examples.iter().enumerate()
                .filter(|(_, ex)| {
                    let cat_label = CATEGORY_LABELS.iter()
                        .find(|(c, _)| *c == ex.category)
                        .map(|(_, l)| *l)
                        .unwrap_or("");
                    let search = format!("{} {} {} {}", cat_label, ex.name, ex.description, ex.category.label());
                    search.to_lowercase().contains(&text)
                })
                .map(|(i, _)| i)
                .collect();
        }
        self.selected_index = 0;
    }

    fn run_example(&mut self, idx: usize, terminal: &mut Terminal) -> io::Result<()> {
        let example = &self.examples[idx];
        let mut out = io::stdout();
        terminal.clear()?;
        terminal.move_cursor(0, 0)?;
        write!(out, "\x1b[2;37mRunning: {}\x1b[0m\n\n", example.name)?;
        out.flush()?;

        (example.run)(terminal)?;

        terminal.clear()?;
        terminal.hide_cursor()?;
        Ok(())
    }

    fn draw(&self, out: &mut io::Stdout, terminal: &Terminal) -> io::Result<()> {
        let (term_w, term_h) = terminal.size();
        let w = term_w as usize;
        let h = term_h as usize;
        let t = self.theme;

        terminal.clear()?;

        // Title
        let title_x = w.saturating_sub(TITLE.len()) / 2;
        terminal.move_cursor(title_x as u16, 0)?;
        write!(out, "{}{}\x1b[0m", t.title_color, TITLE)?;

        // Filter box
        let filter_y: u16 = 2;
        let box_w = w.saturating_sub(4).max(10);
        let is_filter_focused = self.focus == Focus::Filter;
        let f_border = if is_filter_focused { t.focused_border_color } else { t.border_color };

        // Filter border top
        terminal.move_cursor(2, filter_y)?;
        write!(out, "{f_border}┌─")?;
        write!(out, "{}Filter\x1b[0m{f_border}", if is_filter_focused { "\x1b[1;38;2;96;165;250m" } else { "\x1b[2m" })?;
        for _ in 0..box_w.saturating_sub(8) { write!(out, "─")?; }
        write!(out, "┐\x1b[0m")?;

        // Filter input
        terminal.move_cursor(4, filter_y + 1)?;
        if self.filter_text.is_empty() {
            write!(out, " \x1b[3m{}Filter examples...\x1b[0m{}", t.input_placeholder_color,
                if is_filter_focused { format!("{}█\x1b[0m", t.input_cursor_color) } else { " ".into() })?;
        } else {
            write!(out, " {}{}\x1b[0m{}", t.input_text_color, self.filter_text,
                if is_filter_focused { format!("{}█\x1b[0m", t.input_cursor_color) } else { " ".into() })?;
        }
        // Fill rest of line
        let used = 1 + self.filter_text.len().max(16);
        for _ in used..box_w { write!(out, " ")?; }

        // Filter border bottom
        terminal.move_cursor(2, filter_y + 2)?;
        write!(out, "{f_border}└")?;
        for _ in 0..box_w { write!(out, "─")?; }
        write!(out, "┘\x1b[0m")?;

        // Examples box
        let list_y = filter_y + 4;
        let list_h = (h as u16).saturating_sub(list_y + 3);
        let is_list_focused = self.focus == Focus::List;
        let l_border = if is_list_focused { t.focused_border_color } else { t.border_color };

        // Examples border top
        terminal.move_cursor(2, list_y)?;
        let title_text = if self.filtered.is_empty() && !self.filter_text.is_empty() {
            " Examples (No Matches) "
        } else {
            " Examples "
        };
        write!(out, "{l_border}┌─{title_text}\x1b[0m{l_border}")?;
        let title_len = title_text.len() + 3;
        for _ in 0..box_w.saturating_sub(title_len) { write!(out, "─")?; }
        write!(out, "┐\x1b[0m")?;

        // Draw items
        if self.filtered.is_empty() {
            terminal.move_cursor(4, list_y + 1)?;
            write!(out, "{}  No matching examples\x1b[0m", t.select_description_color)?;
            for _ in 1..list_h { write!(out, " ")?; }
        } else {
            let mut line: u16 = 0;
            let mut prev_cat: Option<Category> = None;

            for &idx in &self.filtered {
                if line >= list_h { break; }
                let abs_idx = self.filtered.iter().position(|&i| i == idx).unwrap_or(0);
                let is_sel = abs_idx == self.selected_index;
                let ex = &self.examples[idx];
                let is_new_cat = prev_cat != Some(ex.category);

                if is_new_cat && line < list_h {
                    terminal.move_cursor(4, list_y + 1 + line)?;
                    let cat_label = CATEGORY_LABELS.iter()
                        .find(|(c, _)| *c == ex.category)
                        .map(|(_, l)| *l)
                        .unwrap_or("");
                    write!(out, "{}  {}\x1b[0m", t.select_category_color, cat_label)?;
                    let used = cat_label.len() + 4;
                    for _ in used..box_w { write!(out, " ")?; }
                    line += 1;
                    prev_cat = Some(ex.category);
                }

                if line >= list_h { break; }

                terminal.move_cursor(4, list_y + 1 + line)?;
                if is_sel && is_list_focused {
                    write!(out, "{}  \u{25b6} {}\x1b[0m", t.select_selected_bg, ex.name)?;
                    let rest = box_w.saturating_sub(ex.name.len() + 5);
                    for _ in 0..rest { write!(out, " ")?; }
                } else if is_sel {
                    write!(out, "  \u{25b6} {}\x1b[0m", ex.name)?;
                } else {
                    write!(out, "    {}\x1b[0m", ex.name)?;
                }
                line += 1;
            }

            // Fill remaining lines
            while line < list_h {
                terminal.move_cursor(2, list_y + 1 + line)?;
                for _ in 0..box_w { write!(out, " ")?; }
                line += 1;
            }
        }

        // Examples border bottom
        terminal.move_cursor(2, list_y + list_h)?;
        write!(out, "{l_border}└")?;
        for _ in 0..box_w { write!(out, "─")?; }
        write!(out, "┘\x1b[0m")?;

        // Description line
        let desc_y = list_y + list_h + 1;
        if desc_y < term_h {
            terminal.move_cursor(2, desc_y)?;
            if let Some(&idx) = self.filtered.get(self.selected_index.min(self.filtered.len().saturating_sub(1))) {
                let desc = self.examples[idx].description;
                let max_w = w.saturating_sub(4);
                let d = if desc.len() > max_w { &desc[..max_w] } else { desc };
                write!(out, "{}{}\x1b[0m", t.select_description_color, d)?;
            }
        }

        // Footer instructions
        let footer_y = term_h.saturating_sub(1);
        terminal.move_cursor(0, footer_y)?;
        write!(out, "{}Tab switch focus  |  \u{2191}\u{2193}/j/k navigate  |  Enter run  |  / filter  |  Ctrl+C quit\x1b[0m", t.instructions_color)?;

        out.flush()?;
        Ok(())
    }
}
