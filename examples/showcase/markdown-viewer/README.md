# Markdown Viewer

Terminal-based markdown rendering with syntax highlighting and navigation.

## Features Demonstrated

- Markdown content rendering
- Heading hierarchy display
- Code block visualization
- List rendering
- Badge for document stats
- Keyboard navigation between sections
- StatusLine with document info

## Widgets Used

- Provider, Box, Flex, Text, Heading, Badge, Code, Separator, StatusLine

## Keyboard Shortcuts

| Key | Action              |
|-----|---------------------|
| j   | Next section        |
| k   | Previous section    |
| q   | Quit                |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] Markdown content renders
- [ ] Headings display at correct levels
- [ ] Code blocks are highlighted
- [ ] Lists render properly
- [ ] j/k navigates sections
- [ ] Badge shows section count
- [ ] StatusLine shows current section
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A markdown viewer that renders formatted content in the terminal with section navigation.

## Known Limitations

- No inline images
- No link following
- Limited markdown subset supported
