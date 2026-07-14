//! Application shell using BetterTUI's Engine and Renderer.
//!
//! This demonstrates the recommended pattern:
//! 1. Build UI tree with `Engine`
//! 2. Style nodes with `Style` and `Color`
//! 3. Render with `Renderer` + `AnsiBackend`
//! 4. Handle events with `Terminal`

use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::layout::{FlexDirection, LayoutProps, Sizing};
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};
use bettertui_terminal::{Key, KeyInput, Terminal, TerminalEvent};
use crossterm::event::KeyModifiers;
use tracing::{debug, info, trace};

use crate::examples::{self, Category, Example};
use crate::theme::Theme;

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
    theme: Theme,
    examples: Vec<Example>,
    filtered: Vec<usize>,
    selected_index: usize,
    filter_text: String,
    focus: Focus,
    engine: Engine,
    renderer: Renderer,
}

impl App {
    pub fn new() -> Self {
        info!("App::new() - creating application");
        let all = examples::all();
        let indices: Vec<usize> = (0..all.len()).collect();
        Self {
            theme: Theme::dark(),
            examples: all,
            filtered: indices,
            selected_index: 0,
            filter_text: String::new(),
            focus: Focus::Filter,
            engine: Engine::new(),
            renderer: Renderer::new(80, 24),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal) -> io::Result<()> {
        info!("App::run() - starting application main loop");
        terminal.enter_raw_mode()?;
        terminal.enter_alternate_screen()?;
        terminal.hide_cursor()?;

        terminal.refresh_size()?;
        let (w, h) = terminal.size();
        self.renderer.resize(w, h);
        self.renderer.set_backend(Box::new(AnsiBackend::new()));

        let result = self.main_loop(terminal);

        info!("App::run() - application exiting");
        terminal.show_cursor()?;
        let _ = terminal.leave_alternate_screen();
        let _ = terminal.leave_raw_mode();
        result
    }

    fn main_loop(&mut self, terminal: &mut Terminal) -> io::Result<()> {
        debug!("App::main_loop() - entering main loop");
        loop {
            self.draw(terminal)?;

            match terminal.poll_event(std::time::Duration::from_millis(80))? {
                Some(TerminalEvent::Key(k)) => {
                    if k.code == Key::Char('c') && k.modifiers.contains(KeyModifiers::CONTROL) {
                        info!("App::main_loop() - Ctrl+C received, exiting");
                        return Ok(());
                    }
                    self.handle_key(k, terminal)?;
                }
                Some(TerminalEvent::Resize(w, h)) => {
                    let (old_w, old_h) = terminal.size();
                    debug!(
                        old_width = old_w,
                        old_height = old_h,
                        new_width = w,
                        new_height = h,
                        "App::main_loop() - resize event received"
                    );
                    terminal.update_size(w, h);
                    self.renderer.resize(w, h);
                    debug!(width = w, height = h, "App::main_loop() - renderer resized");
                }
                _ => {}
            }
        }
    }

    fn handle_key(&mut self, key: KeyInput, terminal: &mut Terminal) -> io::Result<()> {
        debug!(?key, "App::handle_key() - processing key event");
        match key.code {
            Key::Tab | Key::Char('\t') => {
                self.focus = if self.focus == Focus::Filter {
                    Focus::List
                } else {
                    Focus::Filter
                };
            }
            Key::Esc => {
                self.focus = if self.focus == Focus::Filter {
                    Focus::List
                } else {
                    Focus::Filter
                };
            }
            Key::Up => {
                if self.focus == Focus::List {
                    self.move_selection(-1);
                }
            }
            Key::Down => {
                if self.focus == Focus::List {
                    self.move_selection(1);
                }
            }
            Key::Char('k')
                if self.focus == Focus::List && !key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
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
        if self.filtered.is_empty() {
            return;
        }
        let len = self.filtered.len();
        let new = (self.selected_index as isize + delta).rem_euclid(len as isize) as usize;
        self.selected_index = new;
    }

    fn apply_filter(&mut self) {
        let text = self.filter_text.to_lowercase();
        if text.is_empty() {
            self.filtered = (0..self.examples.len()).collect();
        } else {
            self.filtered = self
                .examples
                .iter()
                .enumerate()
                .filter(|(_, ex)| {
                    let cat_label = CATEGORY_LABELS
                        .iter()
                        .find(|(c, _)| *c == ex.category)
                        .map(|(_, l)| *l)
                        .unwrap_or("");
                    let search = format!(
                        "{} {} {} {}",
                        cat_label,
                        ex.name,
                        ex.description,
                        ex.category.label()
                    );
                    search.to_lowercase().contains(&text)
                })
                .map(|(i, _)| i)
                .collect();
        }
        self.selected_index = 0;
    }

    fn run_example(&mut self, idx: usize, terminal: &mut Terminal) -> io::Result<()> {
        let example = &self.examples[idx];
        info!(idx, name = %example.name, "App::run_example() - running example");

        let mut out = io::stdout();
        let mut engine = Engine::new();
        let root = engine.arena().root();

        let title_node = engine.create_node(NodeKind::Text);
        engine.set_text(title_node, format!("Running: {}", example.name));
        engine.set_style(
            title_node,
            Style::new()
                .fg(Color::Named(NamedColor::BrightYellow))
                .bold(true),
        );
        engine.append_child(root, title_node).unwrap();

        let mut renderer = Renderer::new(80, 2);
        renderer.set_backend(Box::new(AnsiBackend::new()));
        let frame = renderer.render_full(engine.arena_mut());
        out.write_all(&frame.output_data)?;
        out.write_all(b"\n\n")?;
        out.flush()?;

        (example.run)(terminal)?;

        info!(name = %example.name, "App::run_example() - example completed");
        terminal.hide_cursor()?;

        let (w, h) = terminal.size();
        self.renderer.resize(w, h);
        self.engine.arena_mut().clear();

        Ok(())
    }

    fn draw(&mut self, _terminal: &mut Terminal) -> io::Result<()> {
        let (w, h) = _terminal.size();
        trace!(width = w, height = h, "App::draw() - drawing frame");
        let t = self.theme;

        self.engine.arena_mut().clear();
        let root = self.engine.arena().root();

        self.engine.set_layout(
            root,
            LayoutProps {
                direction: FlexDirection::Column,
                width: Some(Sizing::Points(w as f32)),
                height: Some(Sizing::Points(h as f32)),
                ..LayoutProps::default()
            },
        );

        let title_node = self.engine.create_node(NodeKind::Text);
        self.engine.set_text(title_node, TITLE);
        self.engine
            .set_style(title_node, Style::new().fg(t.title_color).bold(true));
        self.engine.append_child(root, title_node).unwrap();

        let spacer1 = self.engine.create_node(NodeKind::Text);
        self.engine.set_text(spacer1, "");
        self.engine.append_child(root, spacer1).unwrap();

        let filter_label = self.engine.create_node(NodeKind::Text);
        self.engine
            .set_text(filter_label, format!("Filter: {}", self.filter_text));
        self.engine.set_style(
            filter_label,
            Style::new()
                .fg(if self.focus == Focus::Filter {
                    t.focused_border_color
                } else {
                    t.border_color
                })
                .bold(true),
        );
        self.engine.append_child(root, filter_label).unwrap();

        let spacer2 = self.engine.create_node(NodeKind::Text);
        self.engine.set_text(spacer2, "");
        self.engine.append_child(root, spacer2).unwrap();

        let mut prev_cat: Option<Category> = None;
        for &idx in &self.filtered {
            let ex = &self.examples[idx];
            let is_new_cat = prev_cat != Some(ex.category);

            if is_new_cat {
                let cat_label = CATEGORY_LABELS
                    .iter()
                    .find(|(c, _)| *c == ex.category)
                    .map(|(_, l)| *l)
                    .unwrap_or("");

                let cat_node = self.engine.create_node(NodeKind::Text);
                self.engine
                    .set_text(cat_node, format!("  [ {} ]", cat_label));
                self.engine
                    .set_style(cat_node, Style::new().fg(t.category_color).bold(true));
                self.engine.append_child(root, cat_node).unwrap();
                prev_cat = Some(ex.category);
            }

            let abs_idx = self.filtered.iter().position(|&i| i == idx).unwrap_or(0);
            let is_sel = abs_idx == self.selected_index;

            let item_node = self.engine.create_node(NodeKind::Text);
            let prefix = if is_sel && self.focus == Focus::List {
                "  ▶ "
            } else {
                "    "
            };
            self.engine
                .set_text(item_node, format!("{}{}", prefix, ex.name));
            self.engine.set_style(
                item_node,
                if is_sel && self.focus == Focus::List {
                    Style::new()
                        .fg(t.selected_text_color)
                        .bg(t.selected_bg_color)
                        .bold(true)
                } else if is_sel {
                    Style::new().fg(t.selected_text_color).bold(true)
                } else {
                    Style::new().fg(t.text_color)
                },
            );
            self.engine.append_child(root, item_node).unwrap();
        }

        let spacer3 = self.engine.create_node(NodeKind::Text);
        self.engine.set_text(spacer3, "");
        self.engine.append_child(root, spacer3).unwrap();

        if let Some(&idx) = self.filtered.get(
            self.selected_index
                .min(self.filtered.len().saturating_sub(1)),
        ) {
            let desc = &self.examples[idx].description;
            let desc_node = self.engine.create_node(NodeKind::Text);
            self.engine.set_text(desc_node, format!("  {}", desc));
            self.engine
                .set_style(desc_node, Style::new().fg(t.description_color));
            self.engine.append_child(root, desc_node).unwrap();
        }

        let spacer4 = self.engine.create_node(NodeKind::Text);
        self.engine.set_text(spacer4, "");
        self.engine.append_child(root, spacer4).unwrap();

        let help_node = self.engine.create_node(NodeKind::Text);
        self.engine.set_text(
            help_node,
            "Tab: switch  |  ↑↓/jk: navigate  |  Enter: run  |  /: filter  |  Ctrl+C: quit",
        );
        self.engine
            .set_style(help_node, Style::new().fg(t.instructions_color));
        self.engine.append_child(root, help_node).unwrap();

        self.engine.begin_frame();
        self.engine.commit_frame();

        let frame = self.renderer.render(self.engine.arena_mut());

        let mut out = io::stdout();
        out.write_all(&frame.output_data)?;
        out.flush()?;

        Ok(())
    }
}
