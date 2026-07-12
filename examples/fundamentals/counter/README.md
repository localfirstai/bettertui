# Counter

A simple counter demonstrating state management and keyboard input.

## Features Demonstrated

- Mutable state with re-rendering
- Keyboard input handling
- Badge component for status display
- Separator for visual division
- StatusLine for persistent status
- Nested Flex layouts

## Widgets Used

- Provider, Box, Flex, Text, Badge, Separator, StatusLine, Heading, Spacer

## Framework APIs

- CommandBuffer, createReconciler, reconciler.createInstance

## Keyboard Shortcuts

| Key | Action       |
|-----|--------------|
| +   | Increment    |
| -   | Decrement    |
| r   | Reset to 0   |
| q   | Quit         |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] Renders counter at 0
- [ ] Pressing + increments count
- [ ] Pressing - decrements count
- [ ] Pressing r resets to 0
- [ ] Min/max tracking works
- [ ] StatusLine updates correctly
- [ ] Pressing q exits cleanly
- [ ] No React warnings
- [ ] No Rust panics

## Expected Behaviour

A clean display showing the current count, action buttons, and a status line. Count updates immediately on keypress.

## Known Limitations

- React components are thin wrappers (element descriptors); the live native render loop is not yet connected, so this example exercises the API surface and reconciler rather than painting pixels.
