pub fn heading(title: &str) {
    let line = "═".repeat(title.len() + 4);
    println!("\n{line}");
    println!("  {title}");
    println!("{line}\n");
}
