# Capability Inspector

Displays all detected terminal capabilities and environment information.

## Features Demonstrated

- Terminal capability detection
- Environment variable reading
- Color depth detection
- Unicode/emoji support detection
- Input protocol detection
- Graphics protocol detection
- Clipboard support detection
- Terminal size detection
- Platform information

## Widgets Used

- Provider, Box, Flex, Text, Heading, Badge, Separator, StatusLine

## Keyboard Shortcuts

| Key | Action           |
|-----|------------------|
| Tab | Cycle category   |
| q   | Quit             |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] All 8 categories render
- [ ] Terminal info shows env vars
- [ ] Display capabilities listed
- [ ] Input capabilities listed
- [ ] Rendering capabilities listed
- [ ] Graphics capabilities listed
- [ ] Clipboard capabilities listed
- [ ] Size information shown
- [ ] Environment info shown
- [ ] Status badges show correct variants
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A comprehensive display of terminal capabilities organized by category. Each capability shows its detected status with a colored badge.

## Known Limitations

- Many capabilities require native detection (not available in pure JS)
- Falls back to env var reading and defaults
- React components are thin wrappers (element descriptors); the live native render loop is not yet connected, so this example exercises the API surface and reconciler rather than painting pixels.
