# Phase 3: Taskbar - Research

## Domain Knowledge
In this phase, we implement the bottom taskbar of the OS environment. In `tfish`, the taskbar resides in a `<footer>` element and contains:
1. **Start Button & Start Menu**: Shows user details (Ovi ren), pinned links (GitHub, Email), a "Log out" button, and a "Reset" button (with confirmation dialog).
2. **System Tray**:
   - Wallpaper uploader (triggers local file upload).
   - Theme dropdown selector (PowerShell, CMD, Matrix, Ubuntu, Dracula).
3. **System Clock**: Ticking date and time display.

## Technical Approach (Dioxus 0.7)
1. **Layout & Primitives**:
   - Instead of shadcn/ui (which uses Radix UI and Tailwind), we will use custom vanilla CSS and Dioxus signals.
   - **Start Menu**: Toggled with `use_signal(|| false)`. Rendered as an absolute div anchored above the Start button.
   - **Dropdown Menu**: Toggled with `use_signal(|| false)`. Rendered as an absolute div anchored above the Palette icon.
   - **Alert Dialog (Reset)**: Toggled with `use_signal(|| false)`. Rendered as a fixed full-screen modal overlay.
2. **File Upload (Wallpaper)**:
   - Dioxus 0.7 provides a file selection API on events.
   - On change of `<input type="file" />`, we get `e.files()`.
   - We can read the first file asynchronously on the client using `web-sys` or Dioxus' built-in file reader if available. Alternatively, we can use `gloo-file` or `web-sys` directly to read the file as a DataURL so it can be set as CSS `background-image`.
   - Let's check how to do this in Rust. The easiest way on the web target is using `web_sys` and `FileReader` to read the file to a Data URL.
3. **Lucide Icons**:
   - Since we are not using a Rust Lucide library, we will implement the icons as reusable inline SVG components or directly render inline SVGs for maximum control and zero external dependency problems.
   - Icons needed: `LayoutGrid` (Start), `Github`, `Mail`, `LogOut`, `ImageIcon` (Wallpaper), `Palette` (Theme).

## Validation Architecture
- Verify clicking the Start button opens and closes the Start menu.
- Verify clicking Pinned links redirects properly.
- Verify clicking "Log out" returns the state to `Login`.
- Verify clicking "Reset" opens the dialog, and confirming reloads the app state.
- Verify selecting a theme changes the active theme state.
- Verify uploading an image changes the desktop background.
