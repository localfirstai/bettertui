mod engine_demo;
mod layout_demo;
mod post_process_demo;
mod styling_demo;
mod syntax_demo;
mod terminal_demo;
mod text_demo;
mod util;
mod widgets_demo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let demo = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match demo {
        "engine" | "hello-engine" => engine_demo::run(),
        "layout" => layout_demo::run(),
        "styling" | "style" => styling_demo::run(),
        "text" => text_demo::run(),
        "terminal" | "interactive" => terminal_demo::run(),
        "widgets" => widgets_demo::run(),
        "syntax" | "highlight" => syntax_demo::run(),
        "post-process" | "effects" => post_process_demo::run(),
        "all" => run_all(),
        "list" => list_demos(),
        _ => print_usage(&args),
    }
}

fn run_all() {
    println!("═══ Running ALL demos ═══\n");
    engine_demo::run();
    println!();
    layout_demo::run();
    println!();
    styling_demo::run();
    println!();
    text_demo::run();
    println!();
    widgets_demo::run();
    println!();
    syntax_demo::run();
    println!();
    post_process_demo::run();
    println!("\n═══ All demos complete ═══");
}

fn list_demos() {
    println!("Available demos:");
    for (name, desc) in DEMOS {
        println!("  {name:<20} {desc}");
    }
}

const DEMOS: &[(&str, &str)] = &[
    ("engine", "Basic engine: tree building, commands, rendering"),
    ("layout", "Flexbox layout with nested containers"),
    ("styling", "Colors, borders, text styles, SGR output"),
    ("text", "TextEngine: buffer editing, cursor, search"),
    ("terminal", "Interactive terminal: raw mode, events, drawing"),
    ("widgets", "WidgetHost and widget lifecycle"),
    ("syntax", "Tree-sitter syntax highlighting"),
    ("post-process", "Render effects: CRT, scanlines, bloom"),
    ("all", "Run all non-interactive demos sequentially"),
    ("list", "Show this list"),
];

fn print_usage(args: &[String]) {
    eprintln!("BetterTUI Native Examples");
    eprintln!(
        "Usage: {} <demo>",
        args.first().map(|s| s.as_str()).unwrap_or("bettertui-examples")
    );
    eprintln!();
    list_demos();
    std::process::exit(1);
}
