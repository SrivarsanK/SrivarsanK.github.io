# Phase 6: Final Polish - Theme Integration & Auto-Scroll

## Domain Knowledge
During Phase 5, the core Terminal Window and Terminal Commands were implemented simultaneously. The final missing pieces for the portfolio OS are dynamic theming for the Terminal and auto-scrolling behavior when new commands are executed.

## Technical Approach (Dioxus 0.7)
1. **Dynamic Theming**:
   - The OS currently tracks the selected theme in `main.rs` (`current_theme` signal).
   - We need to pass `current_theme` to the `Terminal` component via props.
   - Define theme colors for the terminal (Background, Text, Prompt Color, etc.) matching the current selected theme ("powershell", "ubuntu", "matrix", etc.).
2. **Auto-Scroll**:
   - Add an `id="terminal-body"` to the scrolling container in `terminal.rs`.
   - Use a `use_effect` hook that watches the length of the `lines` signal.
   - Inside the hook, execute JS via `web_sys` (or `eval` if using Dioxus `use_eval`) to scroll the element to its `scrollHeight`.
3. **Focus Management**:
   - Ensure clicking the terminal body automatically focuses the hidden input field.
