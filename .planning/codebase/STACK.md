# Technology Stack

## Core Framework

- **Dioxus 0.7.1** — Rust-based cross-platform UI framework
  - Features: `router`, `fullstack`
  - Supports web, desktop, and mobile from a single codebase
  - Uses RSX macro for declarative UI (similar to JSX)
  - Server functions via `#[post]` macro for fullstack RPC

## Language

- **Rust** — Edition 2021
  - Workspace resolver v2
  - Cargo workspace with 5 member crates

## Build Tooling

- **dx serve** — Dioxus CLI for dev server, hot reload, bundling
- **Cargo** — Rust package manager and build system
- **clippy.toml** — Custom lint configuration

## Styling

- **Vanilla CSS** — Per-component CSS files loaded via `document::Link`
  - `main.css` — Global styles (web)
  - `hero.css`, `navbar.css`, `echo.css` — Component-scoped styles (ui)
  - `blog.css` — View-specific styles (web)

## Assets

- **Static assets** via Dioxus `asset!()` macro
  - SVG header image
  - Favicon (ICO)
  - CSS files

## Platform Targets

| Platform | Package | Status |
|----------|---------|--------|
| Web | `packages/web` | Active (fullstack with SSR) |
| Desktop | `packages/desktop` | Scaffolded |
| Mobile | `packages/mobile` | Scaffolded |

---
*Mapped: 2026-05-20*
