# Roadmap: Personal Portfolio (OS/Terminal Concept)

## Overview

We are building a highly interactive, simulated OS and terminal experience as a personal portfolio. The user journey goes from a simulated boot sequence into a login screen, which unlocks a functional desktop environment. On the desktop, a draggable, resizable terminal window allows users to execute commands to learn about the developer. The project will be implemented in Rust using Dioxus 0.7, porting an existing React codebase.

## Phases

- [ ] **Phase 1: Foundation** - Setup Dioxus 0.7 workspace and global CSS structures
- [ ] **Phase 2: Boot & Login** - Implement BIOS boot animation and time-based login screen
- [ ] **Phase 3: Taskbar** - Implement the Start menu, system tray, and state integrations
- [ ] **Phase 4: Desktop Environment** - Implement background rendering and draggable icons
- [ ] **Phase 5: Terminal Component** - Implement the core draggable window, text rendering, and auto-scroll
- [ ] **Phase 6: Terminal Commands** - Implement command execution logic and API integrations

## Phase Details

### Phase 1: Foundation
**Goal**: Establish the basic Dioxus 0.7 fullstack structure, styling baseline, and server functions logic if needed.
**Depends on**: Nothing
**Requirements**: ARCH-01, ARCH-02, ARCH-03
**Success Criteria**:
  1. The Dioxus app runs without errors via `dx serve`.
  2. Global CSS and component-level styles apply correctly.
**Plans**: TBD

Plans:
- [ ] 01-01: TBD

### Phase 2: Boot & Login
**Goal**: Create the initial user onboarding flow with a boot animation and lock screen.
**Depends on**: Phase 1
**Requirements**: BOOT-01, BOOT-02, BOOT-03, LOG-01, LOG-02, LOG-03, LOG-04
**Success Criteria**:
  1. Users see a simulated text-based BIOS boot sequence.
  2. Users are presented with a login screen showing the current time.
  3. Clicking "Login" shows a spinner and transitions to the desktop state.
**Plans**: TBD

Plans:
- [ ] 02-01: TBD

### Phase 3: Taskbar
**Goal**: Build the bottom OS taskbar including Start menu, links, and system tray (clock, theme, wallpaper).
**Depends on**: Phase 2
**Requirements**: TASK-01, TASK-02, TASK-03, TASK-04, TASK-05, TASK-06
**Success Criteria**:
  1. A sticky taskbar appears at the bottom.
  2. Start menu opens with user info and pinned links.
  3. Theme and wallpaper controls exist in the tray.
**Plans**: TBD

Plans:
- [ ] 03-01: TBD

### Phase 4: Desktop Environment
**Goal**: Render the main desktop backdrop and interactive icons.
**Depends on**: Phase 3
**Requirements**: DESK-01, DESK-02, DESK-03, DESK-04, DESK-05
**Success Criteria**:
  1. Desktop displays the default `frieren.jpg` background.
  2. Icons (About, Projects, Waifu, Joke, Trash) are rendered on the desktop.
  3. Icons can be dragged around via mouse.
**Plans**: TBD

Plans:
- [ ] 04-01: TBD

### Phase 5: Terminal Component
**Goal**: Build the core window manager logic and terminal text rendering system.
**Depends on**: Phase 4
**Requirements**: TERM-01, TERM-02, TERM-03, TERM-04, TERM-05, TERM-06, TERM-07, TERM-08
**Success Criteria**:
  1. A window can be dragged by its header and resized.
  2. Terminal displays output history and accepts text input.
  3. Terminal auto-scrolls when new text appears.
  4. Themes correctly change terminal colors.
**Plans**: TBD

Plans:
- [ ] 05-01: TBD

### Phase 6: Terminal Commands
**Goal**: Wire up the interactive commands that provide the actual portfolio content.
**Depends on**: Phase 5
**Requirements**: CMD-01, CMD-02, CMD-03, CMD-04, CMD-05, CMD-06, CMD-07, CMD-08, CMD-09
**Success Criteria**:
  1. `help`, `about`, `skills`, `projects`, and `contact` return correct static text.
  2. `clear` wipes the terminal history.
  3. `waifu` and `joke` fetch data from external APIs and render correctly.
**Plans**: TBD

Plans:
- [ ] 06-01: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Foundation | 0/TBD | Not started | - |
| 2. Boot & Login | 0/TBD | Not started | - |
| 3. Taskbar | 0/TBD | Not started | - |
| 4. Desktop Environment | 0/TBD | Not started | - |
| 5. Terminal Component | 0/TBD | Not started | - |
| 6. Terminal Commands | 0/TBD | Not started | - |
