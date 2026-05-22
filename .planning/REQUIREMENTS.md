# Requirements: Personal Portfolio (OS/Terminal Concept)

**Defined:** 2026-05-20
**Core Value:** An immersive, memorable, and interactive OS/terminal simulation that showcases technical proficiency in Rust and Dioxus while providing standard portfolio information (projects, skills, about me).

## v1 Requirements

### Architecture & Foundation

- [ ] **ARCH-01**: Setup Dioxus 0.7 fullstack with routing in workspace (web, api, desktop, mobile crates)
- [ ] **ARCH-02**: Implement basic server functions framework for backend processing (if needed for APIs like waifu/joke)
- [ ] **ARCH-03**: Support CSS inclusion per component using `document::Link` or global styling in `index.css`

### Boot Sequence

- [ ] **BOOT-01**: Display "BIOS" style boot text sequence on initial load
- [ ] **BOOT-02**: Show text incrementally (simulating boot time)
- [ ] **BOOT-03**: Auto-transition to login screen after boot finishes

### Login Screen

- [ ] **LOG-01**: Display login screen with profile picture/icon and name
- [ ] **LOG-02**: Show current date and time on login screen
- [ ] **LOG-03**: Provide "Login" button with loading spinner simulation
- [ ] **LOG-04**: Transition to desktop environment on successful login

### Desktop Environment

- [ ] **DESK-01**: Display desktop with customizable background wallpaper
- [ ] **DESK-02**: Support default wallpaper (`frieren.jpg` via asset macro)
- [ ] **DESK-03**: Render desktop icons for About, Projects, Waifu, Joke, Recycle Bin
- [ ] **DESK-04**: Allow icons to be double-clicked to trigger terminal commands
- [ ] **DESK-05**: Allow icons to be dragged around the desktop

### Terminal Component

- [ ] **TERM-01**: Implement windowed terminal interface
- [ ] **TERM-02**: Support draggable window
- [ ] **TERM-03**: Support resizable window (width/height)
- [ ] **TERM-04**: Support minimize/maximize window controls
- [ ] **TERM-05**: Retain command history (up/down arrows to navigate)
- [ ] **TERM-06**: Display current user prompt (e.g. `PS C:\Users\ren>`)
- [ ] **TERM-07**: Auto-scroll to bottom on new output
- [ ] **TERM-08**: Support visual themes (PowerShell, CMD, Matrix, Ubuntu, Dracula)

### Terminal Commands

- [ ] **CMD-01**: Implement `help` command
- [ ] **CMD-02**: Implement `about` command
- [ ] **CMD-03**: Implement `skills` command
- [ ] **CMD-04**: Implement `projects` command
- [ ] **CMD-05**: Implement `contact` command
- [ ] **CMD-06**: Implement `whoami` and `date` commands
- [ ] **CMD-07**: Implement `clear` command
- [ ] **CMD-08**: Implement `waifu` command (fetch from `https://api.waifu.pics/sfw/waifu`)
- [ ] **CMD-09**: Implement `joke` command (fetch from `https://v2.jokeapi.dev/joke/Any`)

### Taskbar

- [ ] **TASK-01**: Render bottom taskbar with Windows-style layout
- [ ] **TASK-02**: Implement Start Menu button with popup (showing user info and pinned links)
- [ ] **TASK-03**: Implement Logout and Reset functionality in Start Menu
- [ ] **TASK-04**: Implement Theme switcher dropdown in system tray
- [ ] **TASK-05**: Implement Wallpaper uploader in system tray
- [ ] **TASK-06**: Display real-time clock in system tray

## v2 Requirements

### Extended Features

- **EXT-01**: Real file system simulation (cd, ls, cat)
- **EXT-02**: Multiple overlapping draggable windows
- **EXT-03**: Native desktop/mobile app deployment via Dioxus

## Out of Scope

| Feature | Reason |
|---------|--------|
| Complex window management (z-index sorting, taskbar grouping) | Too complex for initial v1 release, single terminal window is sufficient |
| Mobile layout for terminal | OS simulation works best on desktop screens, mobile will be a simplified view |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| ARCH-01 | Phase 1 | Pending |
| ARCH-02 | Phase 1 | Pending |
| ARCH-03 | Phase 1 | Pending |
| BOOT-01 | Phase 2 | Pending |
| BOOT-02 | Phase 2 | Pending |
| BOOT-03 | Phase 2 | Pending |
| LOG-01 | Phase 2 | Pending |
| LOG-02 | Phase 2 | Pending |
| LOG-03 | Phase 2 | Pending |
| LOG-04 | Phase 2 | Pending |
| TASK-01 | Phase 3 | Pending |
| TASK-02 | Phase 3 | Pending |
| TASK-03 | Phase 3 | Pending |
| TASK-04 | Phase 3 | Pending |
| TASK-05 | Phase 3 | Pending |
| TASK-06 | Phase 3 | Pending |
| DESK-01 | Phase 4 | Pending |
| DESK-02 | Phase 4 | Pending |
| DESK-03 | Phase 4 | Pending |
| DESK-04 | Phase 4 | Pending |
| DESK-05 | Phase 4 | Pending |
| TERM-01 | Phase 5 | Pending |
| TERM-02 | Phase 5 | Pending |
| TERM-03 | Phase 5 | Pending |
| TERM-04 | Phase 5 | Pending |
| TERM-05 | Phase 5 | Pending |
| TERM-06 | Phase 5 | Pending |
| TERM-07 | Phase 5 | Pending |
| TERM-08 | Phase 5 | Pending |
| CMD-01 | Phase 6 | Pending |
| CMD-02 | Phase 6 | Pending |
| CMD-03 | Phase 6 | Pending |
| CMD-04 | Phase 6 | Pending |
| CMD-05 | Phase 6 | Pending |
| CMD-06 | Phase 6 | Pending |
| CMD-07 | Phase 6 | Pending |
| CMD-08 | Phase 6 | Pending |
| CMD-09 | Phase 6 | Pending |

**Coverage:**

- v1 requirements: 38 total
- Mapped to phases: 38
- Unmapped: 0 ✓

---
*Requirements defined: 2026-05-20*
*Last updated: 2026-05-20 after initial definition*
