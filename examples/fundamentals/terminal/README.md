# Terminal

Terminal capability demonstration including Unicode, emoji, and styling.

## Features Demonstrated

- Unicode character display (box drawing, arrows, math, currency)
- Emoji rendering
- Text styling (bold, dim, color, underline, strikethrough)
- Terminal concept visualization
- Process lifecycle concepts

## Widgets Used

- Provider, Box, Flex, Text, Heading, Badge, Separator, StatusLine, Spacer

## Keyboard Shortcuts

| Key | Action                  |
|-----|-------------------------|
| u   | Cycle unicode set       |
| e   | Cycle emoji set         |
| s   | Cycle style set         |
| q   | Quit                    |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] Unicode characters render correctly
- [ ] Emoji display properly
- [ ] Text styles (bold, dim, etc.) work
- [ ] All sections render
- [ ] Keyboard cycling works
- [ ] StatusLine shows controls
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A display of terminal capabilities showing Unicode, emoji, and styled text. Each section can be cycled through different display sets.

## Known Limitations

- True PTY allocation not yet implemented (uses piped stdio)
- Terminal process spawning via Rust engine not yet wired to React layer
