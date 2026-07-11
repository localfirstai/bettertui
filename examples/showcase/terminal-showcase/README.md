# Terminal Showcase

Advanced terminal features: escape sequences, cursor control, and raw input handling.

## Features Demonstrated

- Raw mode stdin handling
- Arrow key detection
- Special key combinations (Ctrl+C, Tab)
- Escape sequence parsing
- Terminal state display
- Key history tracking
- StatusLine with terminal info

## Widgets Used

- Provider, Box, Flex, Text, Heading, Badge, Separator, StatusLine

## Keyboard Shortcuts

| Key | Action                    |
|-----|---------------------------|
| ↑↓←→ | Arrow keys              |
| Tab | Tab key                   |
| Any | Display key info          |
| q   | Quit                      |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] Arrow keys are detected
- [ ] Tab key is detected
- [ ] Key history displays
- [ ] Special keys show escape codes
- [ ] StatusLine shows terminal info
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A terminal showcase that demonstrates raw input handling and escape sequence parsing.

## Known Limitations

- No mouse event capture
- No terminal resize handling
- Limited to keyboard input
