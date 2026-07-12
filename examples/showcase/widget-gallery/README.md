# Widget Gallery

Visual reference displaying every BetterTUI widget with all variants and states.

## Features Demonstrated

- All 40 BetterTUI React components
- Layout widgets (Box, Flex, Grid, Stack, Spacer, Separator)
- Typography widgets (Text, Heading, Label, Code, Blockquote)
- Interactive widgets (Button, Input, Textarea, Checkbox, Switch, Slider, Radio, Select, Combobox, Tabs)
- Navigation widgets (Tabs, Accordion)
- Feedback widgets (Badge, Progress, Spinner)
- Data display widgets (List, Tree, Table, DataTable)
- Overlay widgets (Tooltip, Modal, Popover, Dropdown, ContextMenu)
- Status widgets (Toast, StatusLine)
- Container widgets (Pane, Viewport, Calendar, Chart)

## Widgets Used

All 40 components from @bettertui/react

## Keyboard Shortcuts

| Key | Action                |
|-----|-----------------------|
| Tab | Next category         |
| Shift+Tab | Previous category |
| q   | Quit                  |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] All 9 categories render
- [ ] Tab switching works
- [ ] Layout widgets display
- [ ] Typography widgets show styles
- [ ] Interactive widgets render
- [ ] Badge variants display
- [ ] Progress bar renders
- [ ] Table/DataTable render
- [ ] StatusLine shows controls
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A categorized gallery of all widgets. Tab through categories to see each group.

## Known Limitations

- React components are thin wrappers (element descriptors); the live native render loop is not yet connected, so this example exercises the API surface and reconciler rather than painting pixels.
- Overlay widgets (Tooltip, Modal, etc.) are conceptual
