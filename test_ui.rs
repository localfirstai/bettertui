use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};

fn main() {
    let pty_system = NativePtySystem::default();
    
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 100,
        pixel_width: 0,
        pixel_height: 0,
    }).unwrap();

    let mut cmd = CommandBuilder::new("cargo");
    cmd.args(["run", "--manifest-path", "packages/core/Cargo.toml", "-p", "bettertui-examples"]);
    
    let mut child = pair.slave.spawn_command(cmd).unwrap();
    
    let mut reader = pair.master.try_clone_reader().unwrap();
    
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(2));
        let mut writer = pair.master.take_writer().unwrap();
        writer.write_all(b"\x03").unwrap(); // Ctrl+C
    });
    
    let mut output = String::new();
    let mut buf = [0u8; 1024];
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 { break; }
        output.push_str(&String::from_utf8_lossy(&buf[..n]));
    }
    
    let mut parser = vt100::Parser::new(24, 100, 0);
    parser.process(output.as_bytes());
    
    let screen = parser.screen();
    for row in 0..24 {
        let line: String = screen.rows_formatted(row, 100).map(|c| c.0.to_string()).collect();
        println!("{}", line);
    }
}
