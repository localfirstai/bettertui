# System Monitor

Production-quality real-time system monitor demonstrating BetterTUI's full capabilities.

## Features Demonstrated

- Real-time data updates (1-second interval)
- Multi-core CPU monitoring with progress bars
- Memory usage tracking (used/total/cached)
- Disk I/O monitoring
- Network traffic monitoring
- Process table with sorting and selection
- Keyboard-driven navigation
- Live status line
- Multi-section complex layout
- Badge status indicators
- Progress bar animations

## Widgets Used

- Provider, Box, Flex, Grid, Text, Heading, Badge, Progress, DataTable, Separator, StatusLine, Spacer

## Keyboard Shortcuts

| Key | Action                    |
|-----|---------------------------|
| j   | Move selection down       |
| k   | Move selection up         |
| 1   | Sort by PID               |
| 2   | Sort by Name              |
| 3   | Sort by CPU               |
| 4   | Sort by Memory            |
| Space | Toggle process status   |
| h   | Toggle help panel         |
| q   | Quit                      |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] CPU bars update in real-time
- [ ] Memory usage fluctuates
- [ ] Disk I/O rates change
- [ ] Network traffic updates
- [ ] Process table renders
- [ ] j/k navigation works
- [ ] Sorting by column works
- [ ] StatusLine shows live data
- [ ] Uptime counter increments
- [ ] Pressing q exits cleanly
- [ ] No React warnings
- [ ] No memory leaks over time

## Expected Behaviour

A professional system monitor with live-updating metrics, similar to htop/btop. All values update in real-time. Process list is navigable and sortable.

## Known Limitations

- Simulated data (no real system metrics)
- React components are thin wrappers (element descriptors); the live native render loop is not yet connected, so this example exercises the API surface and reconciler rather than painting pixels.
- Process killing is conceptual only
