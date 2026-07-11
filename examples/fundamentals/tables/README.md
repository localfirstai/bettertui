# Tables

Table and DataTable demonstration with sorting and keyboard navigation.

## Features Demonstrated

- Table (basic 3-column)
- DataTable (full featured with selection)
- Column sorting (5 columns)
- Row navigation (j/k/arrows)
- Selection highlighting
- Aggregated department summary

## Widgets Used

- Provider, Box, Flex, Text, Heading, Badge, Table, DataTable, Separator, StatusLine, Spacer

## Keyboard Shortcuts

| Key | Action              |
|-----|---------------------|
| j/↓ | Move selection down |
| k/↑ | Move selection up   |
| 1   | Sort by Name        |
| 2   | Sort by Role        |
| 3   | Sort by Department  |
| 4   | Sort by Salary      |
| 5   | Sort by Status      |
| q   | Quit                |

## Manual Testing Checklist

- [ ] Starts successfully
- [ ] Basic table renders
- [ ] DataTable renders with all columns
- [ ] j/k navigation works
- [ ] Arrow key navigation works
- [ ] Sorting by column works (1-5)
- [ ] Selection highlight visible
- [ ] Department summary shows correct data
- [ ] StatusLine updates on selection change
- [ ] Pressing q exits cleanly
- [ ] No React warnings

## Expected Behaviour

Two tables displayed: a simple one and a full-featured DataTable. Rows can be navigated and columns sorted.

## Known Limitations

- React component stubs; table rendering via Rust engine
