# Phase 6: Final Polish - Validation

## Dimension 8: Nyquist Validation

**Requirements:** POL-01, POL-02

### Success Criteria
1. When selecting a new theme from the Taskbar, the Terminal background and text colors update dynamically.
2. When executing a command (e.g. `help`) that outputs many lines, the terminal automatically scrolls to the bottom so the prompt and input remain visible.
3. Clicking anywhere in the terminal body focuses the input field so the user can begin typing immediately.

### Testing Strategy
- Compile and run via `dx serve`.
- Upon reaching the desktop, change the theme via the Taskbar dropdown. Verify the terminal colors change.
- Execute `help` multiple times to overflow the container. Verify that upon each execution, the terminal body scrolls down to the latest input.
- Click the terminal background and verify typing immediately enters text into the prompt.
