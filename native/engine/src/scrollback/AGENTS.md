# AGENTS.md

## Buffer API

- Internally stores lines newest-to-oldest (index 0 = most recent).
- `line(index)` — index 0 is the most recent line, ascending index goes further back in time.
- `line_absolute(index)` — index 0 is the absolute oldest line, ascending goes forward in time.
- `visible_lines(offset, count)` always returns newest-first order regardless of offset direction.
- Default max_lines is 10,000. `set_max_lines(0)` disables the cap (unbounded).
