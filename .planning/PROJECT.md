# Personal Portfolio (OS/Terminal Concept)

## What This Is

A personal developer portfolio built in Rust using Dioxus. It presents an interactive "operating system" experience in the browser, featuring a boot sequence, login screen, desktop environment, and a draggable, functional terminal window. Users explore the portfolio through terminal commands and desktop icons.

## Core Value

An immersive, memorable, and interactive OS/terminal simulation that showcases technical proficiency in Rust and Dioxus while providing standard portfolio information (projects, skills, about me).

## Requirements

### Validated

- ✓ Basic Dioxus 0.7 fullstack workspace setup (web, api, desktop, mobile crates)
- ✓ Standard project routing and server function capabilities

### Active

- [ ] Implement OS boot sequence animation
- [ ] Implement login screen with working clock
- [ ] Implement desktop environment with custom background image support
- [ ] Implement draggable, resizable window component for the terminal
- [ ] Implement functional terminal with theme support (PowerShell, CMD, Matrix, Ubuntu, Dracula)
- [ ] Implement terminal commands: help, about, skills, projects, contact, whoami, date, clear
- [ ] Implement external API integrations for terminal commands (waifu, joke)
- [ ] Implement desktop icons to trigger terminal commands
- [ ] Implement taskbar (Start menu, Social links, Theme switcher, Wallpaper switcher, Clock)
- [ ] Port styling and animations from the original React/Tailwind codebase to Dioxus

### Out of Scope

- [ ] Mobile/Desktop native app deployment for now — focus on web first.
- [ ] Real file system interaction in the terminal — it's a simulated portfolio experience, not a real shell.

## Context

- We are porting an existing React/Vite/Tailwind codebase (located in the `tfish` directory) to Rust using Dioxus 0.7.
- The original codebase relies heavily on Radix UI, Tailwind CSS, and `framer-motion`/CSS animations.
- The user is fine with using the placeholder content ("Ovi ren", "iamovi.github.io", etc.) from the `tfish` codebase for the initial port.
- We have an existing codebase map in `.planning/codebase` describing the Dioxus 0.7 architecture.

## Constraints

- **Tech Stack**: Must use Rust and Dioxus 0.7. Dioxus 0.7 introduces API changes (e.g., `Signal` instead of `use_state`, no `cx`, etc.) which must be adhered to.
- **Styling**: Needs to match the aesthetic of the original `tfish` project, potentially using Tailwind CSS or standard CSS.
- **Performance**: The OS animations and draggable windows need to be performant in WebAssembly.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| OS/Terminal concept | Distinctive way to showcase skills and stand out as a developer | — Pending |
| Use placeholder info for now | Focus on porting the functionality first, customize content later | — Pending |

---
*Last updated: 2026-05-20 after initialization*

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state
