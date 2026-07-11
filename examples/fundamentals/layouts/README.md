# Layouts

Demonstrates BetterTUI layout primitives: Flex, Box, Grid, Spacer, Separator.

## Features Demonstrated

- Flex column and row layouts
- Box with padding and borders
- Grid for multi-column layouts
- Spacer for flexible spacing
- Separator for visual dividers
- Nested layout composition
- Gap control between elements

## Widgets Used

- Provider, Box, Flex, Grid, Spacer, Separator, Text, Heading

## Keyboard Shortcuts

| Key | Action              |
|-----|---------------------|
| 1   | Toggle padding      |
| 2   | Toggle grid mode    |
| q   | Quit                |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] Column layout renders correctly
- [ ] Row layout renders correctly
- [ ] Box padding is visible
- [ ] Grid displays columns
- [ ] Spacer pushes elements apart
- [ ] Separator divides sections
- [ ] Pressing 1/2 toggles layout modes
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A showcase of layout primitives demonstrating column, row, and grid arrangements with proper spacing and borders.

## Known Limitations

- React component stubs; actual rendering via Rust engine
- Grid columns are fixed; responsive grid not yet implemented
