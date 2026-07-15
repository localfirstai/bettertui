use std::io::Write;

use bettertui_engine::engine::Engine;
use bettertui_engine::render::Renderer;
use bettertui_engine::taffy::{FlexDirection, LayoutProps, RectValues};
use bettertui_engine::tree::{Color, NamedColor, NodeKind, Style};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let scenario = args.get(1).map(|s| s.as_str()).unwrap_or("basic");

    let output = match scenario {
        "basic" => render_basic(),
        "flex-row" => render_flex_row(),
        "flex-column" => render_flex_column(),
        "styled" => render_styled(),
        "nested" => render_nested(),
        "empty" => render_empty(),
        _ => {
            eprintln!("unknown scenario: {scenario}");
            std::process::exit(1);
        }
    };

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&output).unwrap();
    handle.flush().unwrap();
}

fn render_basic() -> Vec<u8> {
    let mut engine = Engine::new();
    let root = engine.arena().root();
    let child = engine.create_node(NodeKind::Text);
    engine.set_text(child, "Hello Layout E2E");
    engine.append_child(root, child).unwrap();
    engine.begin_frame();
    engine.commit_frame();

    let mut renderer = Renderer::new(80, 24);
    renderer.render_full(engine.arena_mut()).output_data
}

fn render_flex_row() -> Vec<u8> {
    let mut engine = Engine::new();
    let root = engine.arena().root();
    engine.set_layout(
        root,
        LayoutProps {
            direction: FlexDirection::Row,
            ..LayoutProps::default()
        },
    );
    let a = engine.create_node(NodeKind::Text);
    let b = engine.create_node(NodeKind::Text);
    engine.set_text(a, "Left");
    engine.set_text(b, "Right");
    engine.set_layout(
        a,
        LayoutProps {
            flex_grow: 1.0,
            ..LayoutProps::default()
        },
    );
    engine.set_layout(
        b,
        LayoutProps {
            flex_grow: 1.0,
            ..LayoutProps::default()
        },
    );
    engine.append_child(root, a).unwrap();
    engine.append_child(root, b).unwrap();
    engine.begin_frame();
    engine.commit_frame();

    let mut renderer = Renderer::new(80, 24);
    renderer.render_full(engine.arena_mut()).output_data
}

fn render_flex_column() -> Vec<u8> {
    let mut engine = Engine::new();
    let root = engine.arena().root();
    engine.set_layout(
        root,
        LayoutProps {
            direction: FlexDirection::Column,
            ..LayoutProps::default()
        },
    );
    let a = engine.create_node(NodeKind::Text);
    let b = engine.create_node(NodeKind::Text);
    engine.set_text(a, "Top");
    engine.set_text(b, "Bottom");
    engine.set_layout(
        a,
        LayoutProps {
            flex_grow: 1.0,
            ..LayoutProps::default()
        },
    );
    engine.set_layout(
        b,
        LayoutProps {
            flex_grow: 1.0,
            ..LayoutProps::default()
        },
    );
    engine.append_child(root, a).unwrap();
    engine.append_child(root, b).unwrap();
    engine.begin_frame();
    engine.commit_frame();

    let mut renderer = Renderer::new(80, 24);
    renderer.render_full(engine.arena_mut()).output_data
}

fn render_styled() -> Vec<u8> {
    let mut engine = Engine::new();
    let root = engine.arena().root();
    let colors = [
        ("Red", NamedColor::BrightRed),
        ("Green", NamedColor::BrightGreen),
        ("Blue", NamedColor::BrightBlue),
    ];
    for &(label, named) in &colors {
        let child = engine.create_node(NodeKind::Text);
        engine.set_text(child, label);
        engine.set_style(child, Style::new().fg(Color::Named(named)));
        engine.set_layout(
            child,
            LayoutProps {
                flex_grow: 1.0,
                ..LayoutProps::default()
            },
        );
        engine.append_child(root, child).unwrap();
    }
    engine.begin_frame();
    engine.commit_frame();

    let mut renderer = Renderer::new(80, 24);
    renderer.render_full(engine.arena_mut()).output_data
}

fn render_nested() -> Vec<u8> {
    let mut engine = Engine::new();
    let root = engine.arena().root();
    let container = engine.create_node(NodeKind::Box);
    engine.set_layout(
        container,
        LayoutProps {
            padding: Some(RectValues {
                left: Some(5.0),
                right: Some(5.0),
                top: Some(2.0),
                bottom: Some(2.0),
            }),
            ..LayoutProps::default()
        },
    );
    let child = engine.create_node(NodeKind::Text);
    engine.set_text(child, "Indented");
    engine.append_child(root, container).unwrap();
    engine.append_child(container, child).unwrap();
    engine.begin_frame();
    engine.commit_frame();

    let mut renderer = Renderer::new(80, 24);
    renderer.render_full(engine.arena_mut()).output_data
}

fn render_empty() -> Vec<u8> {
    let mut engine = Engine::new();
    engine.begin_frame();
    engine.commit_frame();

    let mut renderer = Renderer::new(80, 24);
    renderer.render_full(engine.arena_mut()).output_data
}
