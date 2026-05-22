# Phase 5: Terminal Window - Research

## Domain Knowledge

This phase implements the core application logic of the portfolio OS, which is the Terminal Window interface. The original React version `tfish/src/components/Terminal.tsx` implements:

- A command parser executing: `help`, `about`, `skills`, `projects`, `contact`, `whoami`, `date`, `clear`, `waifu` (API), and `joke` (API).
- A draggable window header to move the terminal across the screen.
- Maximize/Minimize functionality.
- Resizing functionality at the bottom right corner.
- Command history navigation using Arrow Up / Arrow Down.
- Auto-complete via Tab key.
- Custom rendering for outputs (including React components for Waifu and Joke which display fetched data and error handling).

## Technical Approach (Dioxus 0.7)

1. **Terminal Component (`terminal.rs`)**:
   - `struct TerminalLine` representing lines of input and output (enum for `Input`, `Output`, `Error`, etc.).
   - Track `lines` using `use_signal(|| Vec::<TerminalLine>::new())`.
   - Track `history` and `history_index` via signals.
   - Use `onkeydown` on the `<input>` element to handle Enter, ArrowUp, ArrowDown, Tab.
2. **Windowing / Dragging / Resizing**:
   - Track `is_minimized`, `is_maximized`, `position (x,y)`, and `size (width, height)`.
   - Just like Phase 4 desktop dragging, we can use `onpointerdown`/`onpointermove`/`onpointerup` interceptors on the main layout or window for dragging and resizing.
3. **Async Commands (waifu, joke)**:
   - For web compatibility, we can fetch data using `reqwest` (requires `wasm` feature enabled) or `gloo-net` for HTTP requests.
   - Dioxus async handles this elegantly: `spawn(async move { ... fetch ... })` and updating the output signal inside the future.
4. **Integration**:
   - The terminal needs to be rendered on the Desktop and take external commands (e.g. from Desktop icons double clicks). We can expose a `Resource` or `Signal` at the app level `external_command`, which the Terminal listens to via `use_effect`, pushing the command string to its execution pipeline when triggered.

## API Dependencies

- `https://api.waifu.pics/sfw/waifu` (Returns JSON with `{ "url": "..." }`)
- `https://v2.jokeapi.dev/joke/Any` (Returns JSON with `{ "type": "single" | "twopart", "joke": "...", "setup": "...", "delivery": "..." }`)
