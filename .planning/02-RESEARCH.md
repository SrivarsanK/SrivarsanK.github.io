# Phase 2: Boot & Login - Research

## Domain Knowledge
This phase ports `BootSequence.tsx` and `LoginScreen.tsx` from the React codebase to Dioxus 0.7.
- **BootSequence**: Displays BIOS-like startup text. It requires a timer to incrementally reveal text lines, simulating a boot process, and then calls a callback to transition state.
- **LoginScreen**: Shows the current time dynamically, a wallpaper, and a login button. Clicking login triggers a simulated loading state before transitioning.

## Technical Approach (Dioxus 0.7)
1. **State Management**:
   - `BootSequence`: Use `use_signal(|| 0)` to track the current line index.
   - `LoginScreen`: Use `use_signal` for the current time string and `is_logging_in` state.
2. **Timers / Async**:
   - In Dioxus, background tasks can be spawned using `spawn(async move { ... })`.
   - For web compatibility, we can use `gloo-timers::future::sleep` to await the delay in the spawned async block, which updates the signal.
   - E.g., for `BootSequence`, a loop that sleeps and increments the signal until all lines are shown.
3. **App Level Routing / State**:
   - The main `App` component will need to manage the current OS state: `Booting`, `Login`, `Desktop`. We can use an `enum OsState` and `use_signal(|| OsState::Booting)`.
   - We will render either `BootSequence`, `LoginScreen`, or the future `Desktop` component based on this signal.

## Validation Architecture
- Verify the Boot Sequence reveals lines and automatically transitions.
- Verify the Login Screen displays updating time.
- Verify clicking "Login" shows the loading spinner and transitions to Desktop.
