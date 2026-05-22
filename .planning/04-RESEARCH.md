# Phase 4: Desktop Environment - Research

## Domain Knowledge

This phase ports `DesktopIcons.tsx` and sets up the interactive desktop backdrop in `Index.tsx`.

- **DesktopIcons**: Renders the icons for "About Me", "Projects", "Waifu", "Joke", and "Recycle Bin".
- **DesktopIcon**: A draggable icon. It can be dragged around the desktop container. A double-click on any icon executes its corresponding action (which sends a command to the terminal).
- **Draggability**: Dragging must be smooth. In React, this is achieved by registering `mousemove` and `mouseup` event listeners on the `window` when dragging starts.

## Technical Approach (Dioxus 0.7)

1. **Desktop Icons Representation**:
   - Create `packages/ui/src/desktop_icons.rs`.
   - Define a struct/enum representing each desktop icon (label, command/action, icon SVG/path, position).
2. **Dragging with web-sys Window Listeners**:
   - We will use local signal state in each `DesktopIcon` to track position `pos: (i32, i32)` and `is_dragging: bool`.
   - To make dragging smooth and prevent the cursor from losing the icon when moving fast, we will attach global window event listeners when `is_dragging` is true.
   - We can implement this in a `use_effect` that responds to `is_dragging`:
     - When `is_dragging` becomes true: register window `mousemove` and `mouseup` listeners.
     - When `is_dragging` becomes false or on clean-up: unregister them.
     - Keep track of the `drag_start` mouse offset `(i32, i32)`.
3. **Double Click / Open Action**:
   - Dioxus 0.7 supports `ondoubleclick` event handler.
   - When double-clicked, call an `on_icon_click` `EventHandler<String>` to notify the parent application of the command to execute (e.g. `about`, `projects`, etc.).
4. **Icons & Layout**:
   - Port the SVG icons or use standard Lucide SVG pathways directly inside the rust files for User, Folder, Ghost, Smile, and Trash.
   - Style them using modern, transparent glassy overlays (`backdrop-blur-sm`, `bg-blue-500/20`, etc.).

## Validation Architecture

- Verify icons render at their initial coordinates on the desktop.
- Verify clicking and dragging moves the icon across the screen.
- Verify releasing the mouse button stops dragging and leaves the icon at the new position.
- Verify double-clicking an icon triggers the double-click event (we can print to console or notify via state for now).
