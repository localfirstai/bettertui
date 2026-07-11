# Input System

> The input system handles keyboard, mouse, paste, and terminal input.
> It parses raw bytes into structured events.

## 1. Overview

The input system sits between the terminal's stdin and the event system:

```
Terminal stdin (raw bytes)
    ↓
Input Parser
    ↓
Structured Events (KeyEvent, MouseEvent, PasteEvent)
    ↓
Event System
    ↓
Node Handlers
```

## 2. Keyboard Input

### 2.1 Key Parsing

Raw terminal bytes are parsed into `KeyEvent`s. The parser handles:

- **ASCII characters:** Direct mapping (a-z, 0-9, etc.).
- **Control characters:** Ctrl+A through Ctrl+Z.
- **Escape sequences:** Arrow keys, function keys, Home/End, etc.
- **Kitty protocol:** Enhanced key reporting with release events.

### 2.2 Escape Sequence Table

| Sequence | Key |
|----------|-----|
| `\x1b[A` | Arrow Up |
| `\x1b[B` | Arrow Down |
| `\x1b[C` | Arrow Right |
| `\x1b[D` | Arrow Left |
| `\x1b[H` | Home |
| `\x1b[F` | End |
| `\x1b[5~` | Page Up |
| `\x1b[6~` | Page Down |
| `\x1b[2~` | Insert |
| `\x1b[3~` | Delete |
| `\x1b[Z` | Shift+Tab |
| `\x1bOP` - `\x1bOS` | F1-F4 |
| `\x1b[15~` - `\x1b[19~` | F5-F9 |
| `\x1b[20~` - `\x1b[23~` | F10-F12 |
| `\x1b[1;2A` | Shift+Arrow Up |
| `\x1b[1;3A` | Alt+Arrow Up |
| `\x1b[1;5A` | Ctrl+Arrow Up |

### 2.3 Modifier Detection

Modifiers are encoded in escape sequences:

```
\x1b[1;{modifiers}X

Modifiers bitmask:
  1 = Shift
  2 = Alt (Option)
  4 = Ctrl
  8 = Super/Meta
```

### 2.4 Kitty Keyboard Protocol

The Kitty protocol provides enhanced key reporting:

```
Enable:  ESC[>31u
Disable: ESC[<u
```

Kitty events include:
- Key press, repeat, and release events.
- Full modifier state.
- Multi-key sequences.
- Unicode key codes.

### 2.5 Key Mapping

```rust
pub struct KeyMapper {
    pub mappings: HashMap<(Key, Modifiers), Key>,
    pub dead_keys: HashMap<char, HashMap<char, char>>,
}

impl KeyMapper {
    pub fn map(&self, key: Key, modifiers: Modifiers) -> (Key, Modifiers) {
        self.mappings
            .get(&(key, modifiers))
            .map(|&mapped| (mapped, modifiers))
            .unwrap_or((key, modifiers))
    }
}
```

### 2.6 Raw Mode

When raw mode is enabled:

```
ESC[?1049h  — switch to alternate screen
ESC[?25l    — hide cursor
ESC[?7l     — disable line wrapping
ESC[?1h     — enable application cursor keys
```

When raw mode is disabled:

```
ESC[?1049l  — switch to main screen
ESC[?25h    — show cursor
ESC[?7h     — enable line wrapping
ESC[?1l     — disable application cursor keys
```

## 3. Mouse Input

### 3.1 Mouse Protocol

**X10 protocol (basic):**
```
Enable: ESC[?9h
Disable: ESC[?9l
Format: ESC[MCbCx Cy
```

**SGR protocol (extended):**
```
Enable: ESC[?1006h
Disable: ESC[?1006l
Format: ESC[<Cb;Cx;Cy M/m
```

M = press, m = release.

### 3.2 Mouse Button Encoding

```
Cb encoding:
  0 = Left press
  1 = Middle press
  2 = Right press
  64 = Scroll Up
  65 = Scroll Down
  + 4 = Shift
  + 8 = Alt/Meta
  + 16 = Ctrl
  + 32 = Motion (drag)
```

### 3.3 Mouse Position

Mouse coordinates are 1-indexed (ANSI convention):

```rust
pub fn parse_mouse_position(x: u16, y: u16) -> Point {
    Point {
        x: x.saturating_sub(1),  // convert to 0-indexed
        y: y.saturating_sub(1),
    }
}
```

### 3.4 Mouse Event Types

```rust
pub enum MouseEventType {
    Press,
    Release,
    Drag,
    Scroll,
    Move,
}
```

### 3.5 Mouse Tracking

```rust
pub struct MouseTracker {
    pub last_position: Option<Point>,
    pub pressed_button: Option<MouseButton>,
    pub drag_start: Option<Point>,
}
```

The tracker maintains state to:
- Detect drag start (press + move).
- Detect drag end (release after drag).
- Detect hover (move without press).

## 4. Paste Input

### 4.1 Bracketed Paste

Bracketed paste wraps pasted text in escape sequences:

```
Enable:  ESC[?2004h
Disable: ESC[?2004l
Start:   ESC[200~
End:     ESC[201~
```

### 4.2 Paste Detection

```
1. Receive ESC[200~ → start collecting paste data
2. Collect characters until ESC[201~
3. Emit PasteEvent with collected text
```

### 4.3 Paste Sanitization

Pasted text is sanitized:

1. Remove control characters (except newline, tab).
2. Normalize line endings (\r\n → \n).
3. Truncate extremely long pastes (> 1MB).

## 5. Terminal Input

### 5.1 Input Sources

| Source | Protocol | Events |
|--------|----------|--------|
| Keyboard | ASCII + escape sequences | Key events |
| Mouse | X10/SGR mouse protocol | Mouse events |
| Paste | Bracketed paste | Paste events |
| Resize | SIGWINCH signal | Resize events |
| Focus | Focus in/out sequences | Focus events |

### 5.2 Input Loop

```rust
pub fn input_loop(terminal: &mut Terminal, event_queue: &mut EventQueue) {
    loop {
        if let Ok(event) = crossterm::event::read() {
            match event {
                crossterm::event::Event::Key(key) => {
                    let parsed = parse_key(key);
                    event_queue.push(Event::Key(parsed));
                }
                crossterm::event::Event::Mouse(mouse) => {
                    let parsed = parse_mouse(mouse);
                    event_queue.push(Event::Mouse(parsed));
                }
                crossterm::event::Event::Resize(width, height) => {
                    event_queue.push(Event::Resize(ResizeEvent {
                        width,
                        height,
                        previous_width: terminal.width(),
                        previous_height: terminal.height(),
                    }));
                }
                _ => {}
            }
        }
    }
}
```

### 5.3 Async Input

For async applications, input is read asynchronously:

```rust
pub async fn async_input_loop(
    terminal: &mut Terminal,
    event_tx: tokio::sync::mpsc::Sender<Event>,
) {
    loop {
        if crossterm::event::poll(Duration::from_millis(10)).await? {
            if let Ok(event) = crossterm::event::read().await {
                let parsed = parse_event(event);
                event_tx.send(parsed).await?;
            }
        }
    }
}
```

## 6. Input Configuration

### 6.1 Terminal Modes

```rust
pub struct InputConfig {
    pub raw_mode: bool,
    pub mouse_protocol: MouseProtocol,
    pub bracketed_paste: bool,
    pub kitty_keyboard: bool,
    pub focus_events: bool,
}

pub enum MouseProtocol {
    None,
    X10,
    SGR,
}
```

### 6.2 Mode Switching

```rust
impl InputConfig {
    pub fn apply(&self, terminal: &mut Terminal) -> Result<(), io::Error> {
        if self.raw_mode {
            terminal.enable_raw_mode()?;
        }

        if self.mouse_protocol != MouseProtocol::None {
            self.enable_mouse_protocol(terminal)?;
        }

        if self.bracketed_paste {
            write!(terminal, "\x1b[?2004h")?;
        }

        if self.kitty_keyboard {
            write!(terminal, "\x1b[>31u")?;
        }

        if self.focus_events {
            write!(terminal, "\x1b[?1004h")?;
        }

        Ok(())
    }
}
```

## 7. Input Buffering

### 7.1 Byte Buffer

Raw bytes from stdin are accumulated in a buffer:

```rust
pub struct InputBuffer {
    buffer: Vec<u8>,
    max_size: usize,
}
```

### 7.2 Partial Sequence Handling

Escape sequences can arrive in multiple read calls:

```
Read 1: ESC [
Read 2: 1 ; 2 A
```

The parser must handle partial sequences:

1. Accumulate bytes in the buffer.
2. Try to parse a complete sequence.
3. If incomplete, wait for more bytes.
4. If invalid, discard the buffer and report an error.

### 7.3 Timeout

If a partial sequence doesn't complete within a timeout (e.g., 50ms), it's treated as a standalone ESC key press. This prevents the application from hanging on ambiguous input.

## 8. Error Handling

### 8.1 Parse Errors

```rust
pub enum InputError {
    InvalidSequence(Vec<u8>),
    UnsupportedProtocol(String),
    IoError(io::Error),
}
```

### 8.2 Error Recovery

- **Invalid sequence:** Log the bytes and discard. Continue parsing.
- **Unsupported protocol:** Fall back to basic protocol.
- **I/O error:** Retry or abort (depending on severity).

## 9. Performance

### 9.1 Input Latency

Target: <1ms from keystroke to event delivery.

- Byte reading: ~0.1ms (crossterm).
- Parsing: ~0.01ms.
- Event dispatch: ~0.1ms.
- **Total: ~0.21ms.**

### 9.2 Throughput

Target: Handle 1000+ events per second.

- Keyboard: ~10 events/second (human typing speed).
- Mouse: ~100 events/second (fast movement).
- Paste: ~1 event with large payload.

The input system is not throughput-bound — it's latency-bound.

## 10. Future Considerations

### 10.1 Kitty Protocol Full Support

Full Kitty protocol support includes:
- Key repeat events.
- Multi-tap detection.
- Compose key sequences.
- Unicode key names.

### 10.2 Gamepad/Joystick Support

Terminal applications can receive gamepad input via:
- Kitty keyboard protocol extensions.
- Custom escape sequences.
- External input devices (via serial/USB).

### 10.3 Voice Input

Voice input could be integrated via:
- External speech-to-text service.
- Parsed as text input events.
- Mapped to keyboard shortcuts.

### 10.4 Gesture Recognition

Touchpad gestures (on supported terminals) could be recognized:
- Pinch → zoom.
- Swipe → scroll.
- Two-finger tap → right-click.
