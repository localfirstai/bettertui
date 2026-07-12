# Dashboard

Comprehensive system monitoring dashboard with live stats, activity feed, and progress visualization.

## Features Demonstrated

- Multi-panel Grid layout
- StatCard component composition
- Activity feed with severity badges
- Progress bar visualization
- StatusLine for live metadata
- Auto-refresh via keyboard
- Heading, Separator, Spacer, Stack composition

## Widgets Used

- Provider, Box, Flex, Grid, Text, Heading, Badge, Progress, Separator, Spacer, Stack, StatusLine

## Keyboard Shortcuts

| Key | Action          |
|-----|-----------------|
| r   | Refresh (tick++) |
| q   | Quit            |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] Dashboard title renders
- [ ] 6 stat cards display in 2x3 grid
- [ ] CPU value changes on refresh
- [ ] Activity feed shows 4 items with badges
- [ ] Progress bar is visible
- [ ] StatusLine shows version and tick
- [ ] Pressing r increments tick
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A professional dashboard layout with stat cards, activity feed, progress bar, and live status.

## Known Limitations

- No real system metrics (simulated data)
- No mouse interaction
- React components are thin wrappers (element descriptors); the live native render loop is not yet connected, so this example exercises the API surface and reconciler rather than painting pixels.
