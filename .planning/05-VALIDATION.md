# Phase 5: Terminal Window - Validation

## Dimension 8: Nyquist Validation

**Requirements:** TERM-01, TERM-02, TERM-03, TERM-04, TERM-05, TERM-06, TERM-07, TERM-08

### Success Criteria (from ROADMAP.md)

1. Terminal renders in the center of the desktop.
2. Terminal can be dragged, maximized, minimized, and resized.
3. Terminal accepts typing input.
4. Commands like `help`, `about`, `skills`, `projects`, `contact`, `whoami`, `date`, `clear` output correctly.
5. Async API commands `waifu` and `joke` fetch and display output inline.
6. Pressing Arrow Up/Down cycles command history.
7. Double-clicking desktop icons auto-runs the corresponding command in the terminal.

### Testing Strategy

- Compile and run via `dx serve`.
- Upon reaching the desktop, verify the terminal is visible with the intro prompt.
- Drag the terminal header to move it.
- Use the bottom-right corner to resize.
- Click Maximize and Minimize buttons to verify behaviors.
- Type `help` and hit Enter, verify output list.
- Type `clear`, verify lines disappear.
- Type `waifu` and `joke`, verify fetching logic works without panicking.
- Double-click the "Projects" icon on the desktop and verify the terminal runs the `projects` command.
