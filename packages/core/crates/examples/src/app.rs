//! Application shell using BetterTUI's Engine and Renderer.
//!
//! This demonstrates the recommended pattern:
//! 1. Build UI tree with `Engine`
//! 2. Style nodes with `Style` and `Color`
//! 3. Render with `Renderer` + `AnsiBackend`
//! 4. Handle events with `Terminal`

use std::io::{self, Write};

use bettertui_engine::engine::Engine;
use bettertui_engine::layout::{
    AlignItems, FlexDirection, JustifyContent, LayoutProps, Position, RectValues, Sizing,
};
use bettertui_engine::render::{AnsiBackend, Renderer};
use bettertui_engine::tree::{BorderStyle, Color, NamedColor, NodeKind, Style};
use bettertui_terminal::{Key, KeyInput, Terminal, TerminalEvent};
use crossterm::event::{KeyModifiers, MouseEvent, MouseEventKind};
use tracing::{debug, info, trace, warn};

use crate::examples::{self, Category, Example};
use crate::theme::Theme;

const CATEGORY_LABELS: &[(Category, &str)] = &[
    (Category::Engine, "ENGINE"),
    (Category::Layout, "LAYOUT"),
    (Category::Styling, "STYLING"),
    (Category::Text, "TEXT"),
    (Category::Widgets, "WIDGETS"),
    (Category::Effects, "EFFECTS"),
    (Category::Terminal, "TERMINAL"),
];

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Focus {
    #[default]
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
    list_start_y: u16,
    list_end_y: u16,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
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
            list_start_y: 0,
            list_end_y: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal) -> io::Result<()> {
        info!("App::run() - starting application main loop");

        if !terminal.is_tty() {
            warn!("App::run() - not running in a TTY, drawing one frame and exiting");
            let (w, h) = terminal.size();
            self.renderer.resize(w, h);
            self.renderer.set_backend(Box::new(AnsiBackend::new()));
            self.draw(terminal)?;
            return Ok(());
        }

        terminal.enter_raw_mode()?;
        terminal.enter_alternate_screen()?;
        terminal.hide_cursor()?;

        crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;

        terminal.refresh_size()?;
        let (w, h) = terminal.size();
        self.renderer.resize(w, h);
        self.renderer.set_backend(Box::new(AnsiBackend::new()));

        let result = self.main_loop(terminal);

        info!("App::run() - application exiting");
        let _ = crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture);
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
                Some(TerminalEvent::Mouse(m)) => {
                    self.handle_mouse(m);
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

    fn handle_mouse(&mut self, event: MouseEvent) {
        if let MouseEventKind::Down(crossterm::event::MouseButton::Left) = event.kind {
            let y = event.row;

            if (7..=9).contains(&y) {
                self.focus = Focus::Filter;
            } else if y >= self.list_start_y && y < self.list_end_y {
                self.focus = Focus::List;
                // Rough estimate based on new layout - this isn't exact due to descriptions
                let click_y = y - self.list_start_y;
                let mut current_y = 0;
                let mut prev_cat: Option<Category> = None;

                for &idx in &self.filtered {
                    let ex = &self.examples[idx];
                    if prev_cat != Some(ex.category) {
                        current_y += 1;
                        prev_cat = Some(ex.category);
                    }

                    if click_y == current_y
                        || (click_y == current_y + 1 && !ex.description.is_empty())
                    {
                        let abs_idx = self.filtered.iter().position(|&i| i == idx).unwrap_or(0);
                        self.selected_index = abs_idx;
                        break;
                    }

                    current_y += 1;
                    if !ex.description.is_empty() {
                        current_y += 1;
                    }
                }
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

        let header = self.engine.create_node(NodeKind::Flex);
        self.engine.set_layout(
            header,
            LayoutProps {
                direction: FlexDirection::Column,
                width: Some(Sizing::Percent(1.0)),
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                margin: Some(RectValues::new(0.0, 1.0)),
                ..LayoutProps::default()
            },
        );
        self.engine.append_child(root, header).unwrap();

        let title_node = self.engine.create_node(NodeKind::Text);
        let title_text = r#"
 ____      _     _ _        _______     _    _ _____ ____    _  _____ _____ 
| __ ) _  (_) __| | | ___  |_   _\ \   / /  | |  ___/ ___|  / \|_   _| ____|
|  _ \ \/ / |/ _` | |/ _ \   | |  \ \ / /   | | |_  \___ \ / _ \ | | |  _|  
| |_) >  <| | (_| | |  __/   | |   \ V /| |_| |  _|  ___) / ___ \| | | |___ 
|____/_/\_\_|\__,_|_|\___|   |_|    \_/  \___/|_|   |____/_/   \_\_| |_____|
"#;
        self.engine.set_text(title_node, title_text.trim());
        self.engine
            .set_style(title_node, Style::new().fg(t.title_color).bold(true));
        self.engine.append_child(header, title_node).unwrap();

        let filter_container = self.engine.create_node(NodeKind::Box);
        let filter_focused = self.focus == Focus::Filter;
        let filter_bg = if filter_focused {
            t.focused_bg_color
        } else {
            t.bg_color
        };
        self.engine.set_layout(
            filter_container,
            LayoutProps {
                direction: FlexDirection::Row,
                width: Some(Sizing::Percent(1.0)),
                margin: Some(RectValues::new(1.0, 0.0)),
                padding: Some(RectValues::sides(0.0, 0.0, 0.0, 1.0)),
                border: Some(RectValues::uniform(1.0)),
                ..LayoutProps::default()
            },
        );
        self.engine.set_style(
            filter_container,
            Style::new().bg(filter_bg).border(
                BorderStyle::Solid,
                if filter_focused {
                    t.focused_border_color
                } else {
                    t.border_color
                },
            ),
        );
        self.engine.append_child(root, filter_container).unwrap();

        let filter_label = self.engine.create_node(NodeKind::Text);
        let display_text = if self.filter_text.is_empty() {
            "Filter examples...".to_string()
        } else {
            self.filter_text.clone()
        };
        self.engine.set_text(filter_label, display_text);
        self.engine.set_style(
            filter_label,
            Style::new()
                .fg(if self.filter_text.is_empty() {
                    t.placeholder_color
                } else {
                    t.text_color
                })
                .bg(filter_bg),
        );
        self.engine
            .append_child(filter_container, filter_label)
            .unwrap();

        let list_container = self.engine.create_node(NodeKind::Flex);
        self.engine.set_layout(
            list_container,
            LayoutProps {
                direction: FlexDirection::Column,
                width: Some(Sizing::Percent(1.0)),
                flex_grow: 1.0,
                margin: Some(RectValues::sides(0.0, 1.0, 1.0, 1.0)),
                padding: Some(RectValues::sides(0.0, 1.0, 1.0, 1.0)),
                border: Some(RectValues::uniform(1.0)),
                ..LayoutProps::default()
            },
        );
        self.engine.set_style(
            list_container,
            Style::new().border(BorderStyle::Solid, t.border_color),
        );
        self.engine.append_child(root, list_container).unwrap();

        let list_title = self.engine.create_node(NodeKind::Text);
        self.engine.set_layout(
            list_title,
            LayoutProps {
                position: Position::Absolute,
                inset: Some(RectValues {
                    top: Some(-1.0),
                    right: None,
                    bottom: None,
                    left: Some(2.0),
                }),
                ..LayoutProps::default()
            },
        );
        self.engine.set_text(list_title, "Examples");
        self.engine
            .set_style(list_title, Style::new().fg(t.border_color));
        self.engine
            .append_child(list_container, list_title)
            .unwrap();

        self.list_start_y = 12; // Approx starting Y after header, filter, margins, border, padding

        let mut current_y = self.list_start_y;
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
                self.engine.set_text(cat_node, cat_label.to_string());
                self.engine
                    .set_style(cat_node, Style::new().fg(t.category_color).bold(true));
                self.engine.append_child(list_container, cat_node).unwrap();
                current_y += 1;
                prev_cat = Some(ex.category);
            }

            let abs_idx = self.filtered.iter().position(|&i| i == idx).unwrap_or(0);
            let is_sel = abs_idx == self.selected_index;
            let list_focused = self.focus == Focus::List;

            let item_node = self.engine.create_node(NodeKind::Text);
            self.engine.set_layout(
                item_node,
                LayoutProps {
                    width: Some(Sizing::Percent(1.0)),
                    ..LayoutProps::default()
                },
            );
            let prefix = if is_sel && list_focused {
                "▶   "
            } else {
                "    "
            };
            self.engine
                .set_text(item_node, format!("{}{}", prefix, ex.name));
            let item_bg = if is_sel && list_focused {
                t.selected_bg_color
            } else if is_sel {
                t.hover_bg_color
            } else {
                Color::Default
            };
            let item_fg = if is_sel {
                t.selected_text_color
            } else {
                t.text_color
            };
            self.engine
                .set_style(item_node, Style::new().fg(item_fg).bg(item_bg).bold(is_sel));
            self.engine.append_child(list_container, item_node).unwrap();
            current_y += 1;

            if !ex.description.is_empty() {
                let desc_node = self.engine.create_node(NodeKind::Text);
                self.engine.set_layout(
                    desc_node,
                    LayoutProps {
                        width: Some(Sizing::Percent(1.0)),
                        ..LayoutProps::default()
                    },
                );
                self.engine
                    .set_text(desc_node, format!("      {}", ex.description));
                self.engine
                    .set_style(desc_node, Style::new().fg(t.description_color).bg(item_bg));
                self.engine.append_child(list_container, desc_node).unwrap();
                current_y += 1;
            }
        }
        self.list_end_y = current_y;

        let footer = self.engine.create_node(NodeKind::Flex);
        self.engine.set_layout(
            footer,
            LayoutProps {
                direction: FlexDirection::Column,
                align: AlignItems::Center,
                flex_shrink: 0.0,
                ..LayoutProps::default()
            },
        );
        self.engine.append_child(root, footer).unwrap();

        let help_node = self.engine.create_node(NodeKind::Text);
        self.engine.set_text(
            help_node,
            " Tab/Esc switch focus | Type in filter | ↑↓/j/k list | Enter run | / filter | ctrl+c quit ",
        );
        self.engine
            .set_style(help_node, Style::new().fg(t.instructions_color));
        self.engine.append_child(footer, help_node).unwrap();

        self.engine.begin_frame();
        self.engine.commit_frame();

        if self.focus == Focus::Filter {
            let cursor_x = if self.filter_text.is_empty() {
                3
            } else {
                3 + self.filter_text.len() as u16
            };
            let cursor_y = 9;
            self.renderer.set_cursor_position(cursor_x, cursor_y, true);
        } else {
            self.renderer.set_cursor_position(0, 0, false);
        }

        let frame = self.renderer.render(self.engine.arena_mut());

        let mut out = io::stdout();
        out.write_all(&frame.output_data)?;
        out.flush()?;

        Ok(())
    }
}
