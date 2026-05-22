# Concerns

## Current State

This is a **Dioxus starter template** — nearly untouched scaffolding. The codebase is clean but has no real portfolio content.

## Issues

### No Error Handling

- `api::echo()` called with `.await.unwrap()` — will panic on server errors
- No error boundaries or user-facing error states

### No Real Content

- Hero component links to Dioxus docs, not actual portfolio content
- Blog is a placeholder demo with no content management
- Echo is a server function demo, not portfolio-relevant

### No Styling System

- Minimal CSS with no design system, no custom properties, no responsive design
- No typography, color palette, or spacing system defined

### No SEO/Meta

- No `<title>`, `<meta>` tags, or Open Graph metadata
- No structured data for portfolio/blog content

### Desktop/Mobile Not Wired

- Desktop and mobile packages are scaffolded but likely mirror web exactly
- No platform-specific adaptations

---
*Mapped: 2026-05-20*
