# Forms

Interactive form widget demonstration covering all BetterTUI input components.

## Features Demonstrated

- Input (text entry)
- Textarea (multi-line entry)
- Checkbox (boolean toggle)
- Switch (boolean toggle)
- Slider (numeric range)
- Radio (single selection)
- Select (dropdown concept)
- State management for all widgets
- Keyboard navigation

## Widgets Used

- Provider, Box, Flex, Text, Heading, Badge, Input, Textarea, Checkbox, Switch, Slider, Radio, Separator, StatusLine, Spacer

## Keyboard Shortcuts

| Key | Action                    |
|-----|---------------------------|
| i   | Cycle input value         |
| t   | Cycle textarea content    |
| c   | Toggle checkbox           |
| s   | Toggle switch             |
| +/- | Adjust slider             |
| 1/2/3 | Select radio option     |
| q   | Quit                      |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] All form sections render
- [ ] Input displays placeholder and value
- [ ] Textarea shows multi-line content
- [ ] Checkbox toggles ON/OFF
- [ ] Switch toggles ON/OFF
- [ ] Slider value changes with +/-
- [ ] Radio selection updates
- [ ] StatusLine reflects current state
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

A form showcase with all widget types displayed and interactive via keyboard. State changes are immediately visible.

## Known Limitations

- React component stubs; actual widget rendering via Rust engine
- Select/Combobox are conceptual (dropdown not yet implemented in React layer)
