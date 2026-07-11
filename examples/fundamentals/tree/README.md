# Tree

Tree view with expand/collapse and keyboard navigation.

## Features Demonstrated

- Tree component with nested nodes
- Expand/collapse functionality
- Keyboard navigation
- Selection tracking
- Node counting (files vs folders)
- Multi-level nesting

## Widgets Used

- Provider, Box, Flex, Text, Heading, Badge, Tree, Separator, StatusLine, Spacer

## Keyboard Shortcuts

| Key | Action                    |
|-----|---------------------------|
| j   | Move selection down       |
| k   | Move selection up         |
| Enter/Space | Toggle expand/collapse |
| q   | Quit                      |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] Tree renders with nested structure
- [ ] Initial expanded nodes show children
- [ ] j/k navigation works
- [ ] Enter/Space toggles expand/collapse
- [ ] File count updates correctly
- [ ] Selected node info shows correctly
- [ ] StatusLine shows controls
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A file explorer tree with expandable directories. Navigation with j/k, expand/collapse with Enter/Space.

## Known Limitations

- React component stubs; tree rendering via Rust engine
