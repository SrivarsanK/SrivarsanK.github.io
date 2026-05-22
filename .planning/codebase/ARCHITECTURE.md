# Architecture

## Workspace Structure

```text
portfolio/
├── Cargo.toml              # Workspace root
├── packages/
│   ├── web/                # Web platform (fullstack: SSR + client)
│   │   ├── src/main.rs     # Entry point, routes, App component
│   │   ├── src/views/      # Web-specific page components
│   │   └── assets/         # Web-specific CSS, favicon
│   ├── desktop/            # Desktop platform (scaffolded)
│   ├── mobile/             # Mobile platform (scaffolded)
│   ├── ui/                 # Shared UI components library
│   │   ├── src/lib.rs      # Component exports
│   │   ├── src/hero.rs     # Hero section component
│   │   ├── src/navbar.rs   # Navigation bar component
│   │   ├── src/echo.rs     # Server function demo component
│   │   └── assets/         # Shared SVG, component CSS
│   └── api/                # Shared server functions
│       └── src/lib.rs      # Echo server function
```

## Dependency Graph

```text
web ──→ ui ──→ api
         │      │
         │      └── dioxus (fullstack)
         └── dioxus
```

- `web` depends on `ui` (shared components) and `dioxus` (router, fullstack)
- `ui` depends on `api` (server function calls) and `dioxus`
- `api` depends on `dioxus` (fullstack server functions)

## Routing

- `Route` enum in `web/src/main.rs` with `#[derive(Routable)]`
- Two routes: `/` (Home) and `/blog/:id` (Blog with dynamic segment)
- Layout via `#[layout(WebNavbar)]` wrapping `Outlet<Route>`

## Data Flow

1. Client renders RSX components
2. Components call server functions from `api` crate (e.g., `api::echo()`)
3. Server functions execute on server, return results via Dioxus fullstack RPC
4. Component state updated via signals (`use_signal`)

## Component Pattern

- Components are `#[component]` annotated functions returning `Element`
- Each component loads its own CSS via `document::Link`
- Props via function arguments (e.g., `children: Element`, `id: i32`)

---
Mapped: 2026-05-20
