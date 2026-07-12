# Performance Lab

Stress testing and benchmarking tool for BetterTUI rendering performance.

## Features Demonstrated

- Large dataset rendering (tables, trees)
- Continuous rendering throughput
- FPS measurement
- Frame timing
- Memory usage tracking
- Multiple test scenarios
- Test history and results
- Real-time metrics display

## Widgets Used

- Provider, Box, Flex, Text, Heading, Badge, Progress, Table, Tree, Separator, StatusLine

## Keyboard Shortcuts

| Key | Action                  |
|-----|-------------------------|
| 1-5 | Select test scenario    |
| Space | Start/stop test       |
| c   | Clear history           |
| q   | Quit                    |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] Test selection works (1-5)
- [ ] Space starts/stops test
- [ ] FPS counter updates
- [ ] Frame count increments
- [ ] Render time tracked
- [ ] Large table test renders
- [ ] Large tree test renders
- [ ] Rapid updates test runs
- [ ] Mixed workload test runs
- [ ] Test history records results
- [ ] c clears history
- [ ] StatusLine shows metrics
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A performance testing interface with selectable stress tests. Running a test shows real-time FPS, frame count, and render time metrics.

## Known Limitations

- React components are thin wrappers (element descriptors); the live native render loop is not yet connected, so this example exercises the API surface and reconciler rather than painting pixels.
- Memory usage is simulated
- FPS measured at application level (not true vsync)
