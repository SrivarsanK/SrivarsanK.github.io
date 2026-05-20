# Phase 3: Taskbar - Validation

## Dimension 8: Nyquist Validation

**Requirements:** TASK-01, TASK-02, TASK-03, TASK-04, TASK-05, TASK-06

### Success Criteria (from ROADMAP.md)
1. A sticky taskbar appears at the bottom.
2. Start menu opens with user info and pinned links.
3. Theme and wallpaper controls exist in the tray.

### Testing Strategy
- Compile and run via `dx serve`.
- Once logged in, check that the taskbar is stickily anchored to the bottom.
- Click the Start button. Verify the Start menu pops up.
- Click the Theme (palette) button. Verify the dropdown opens. Select a theme (e.g., Matrix) and verify that the app theme state changes (e.g., text color changes).
- Click the Wallpaper button. Select a local image and verify it becomes the desktop background.
- Click the Reset button in the Start menu. Verify the confirmation modal opens. Cancel it, then verify it closes. Click reset and confirm, verify settings are reset.
- Click the Log out button. Verify it logs out to the login screen.
- Verify the system clock in the bottom right corner shows the current time/date and updates.
