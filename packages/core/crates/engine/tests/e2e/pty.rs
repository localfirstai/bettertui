use std::time::Duration;

use bettertui_engine::pty::{PtyConfig, PtyProcess, PtySize};

#[test]
fn e2e_pty_spawn_echo() {
    let config = PtyConfig::new("echo")
        .with_args(vec!["Hello PTY".to_string()])
        .with_size(PtySize::new(80, 24));

    let mut process = PtyProcess::spawn(config).expect("failed to spawn echo via PTY");

    let mut output = Vec::new();
    let mut buf = [0u8; 4096];
    let timeout = Duration::from_secs(5);

    let start = std::time::Instant::now();
    loop {
        if start.elapsed() > timeout {
            break;
        }
        match process.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => output.extend_from_slice(&buf[..n]),
            Err(_) => break,
        }
    }

    process.kill().ok();

    let output_str = String::from_utf8_lossy(&output);
    assert!(!output_str.is_empty(), "PTY output should not be empty");
    assert!(
        output_str.contains("Hello PTY"),
        "output should contain the echo text"
    );
}

#[test]
fn e2e_pty_size_resize() {
    let config = PtyConfig::new("echo")
        .with_args(vec!["test".to_string()])
        .with_size(PtySize::new(40, 10));

    let mut process = PtyProcess::spawn(config).expect("failed to spawn process");

    let initial = process.size();
    assert_eq!(initial.cols, 40);
    assert_eq!(initial.rows, 10);

    process
        .resize(PtySize::new(80, 24))
        .expect("resize should succeed");
    let resized = process.size();
    assert_eq!(resized.cols, 80);
    assert_eq!(resized.rows, 24);

    process.kill().ok();
}

#[test]
fn e2e_pty_process_lifecycle() {
    let config = PtyConfig::new("true").with_size(PtySize::new(80, 24));
    let mut process = PtyProcess::spawn(config).expect("failed to spawn true");

    assert!(process.is_running(), "process should be running initially");

    let status = process.wait().expect("wait should succeed");
    assert!(status.is_some());

    assert!(
        !process.is_running(),
        "process should not be running after exit"
    );
}
