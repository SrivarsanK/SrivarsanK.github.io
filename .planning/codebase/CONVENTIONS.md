# Conventions

## Component Pattern

```rust
// 1. Imports
use dioxus::prelude::*;

// 2. Asset constants
const COMPONENT_CSS: Asset = asset!("/assets/styling/component.css");

// 3. Component function
#[component]
pub fn ComponentName(/* props */) -> Element {
    rsx! {
        // CSS link first
        document::Link { rel: "stylesheet", href: COMPONENT_CSS }
        // Component markup
        div { id: "component-name", /* ... */ }
    }
}
```

## CSS Organization

- Global CSS in platform-specific `assets/` directories
- Component CSS in `ui/assets/styling/` (shared) or platform `assets/` (platform-specific)
- Each component loads its own CSS — no global CSS bundle for components

## Module Pattern

- One component per file
- Re-export via `lib.rs` (for crates) or `mod.rs` (for directories)
- Public API surface explicitly defined

## Server Functions

- Defined in `api` crate, called from `ui` components
- Use `#[post("/api/endpoint")]` macro
- Return `Result<T, ServerFnError>`
- Called with `.await.unwrap()` (no error handling yet)

## Routing

- Central `Route` enum per platform
- Platform-specific layout wrapping shared `Navbar`
- Dynamic segments as enum variant fields

---
*Mapped: 2026-05-20*
