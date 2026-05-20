# File Structure

## File Count

| Package | Rust Files | CSS Files | Assets | Total |
|---------|-----------|-----------|--------|-------|
| web | 4 | 2 | 1 | 7 |
| ui | 4 | 2 | 1 | 7 |
| api | 1 | 0 | 0 | 1 |
| desktop | ~4 | ~2 | ~1 | ~7 |
| mobile | ~4 | ~2 | ~1 | ~7 |
| **Total** | **~17** | **~8** | **~4** | **~29** |

## Key Files

| File | Purpose | Lines |
|------|---------|-------|
| `packages/web/src/main.rs` | App entry, routing, layout | 59 |
| `packages/web/src/views/home.rs` | Home page view | 11 |
| `packages/web/src/views/blog.rs` | Blog page with dynamic routing | 31 |
| `packages/ui/src/lib.rs` | Shared component exports | 11 |
| `packages/ui/src/hero.rs` | Hero section with links | 25 |
| `packages/ui/src/navbar.rs` | Navigation bar wrapper | 16 |
| `packages/ui/src/echo.rs` | Server function demo | 31 |
| `packages/api/src/lib.rs` | Echo server function | 9 |

## Naming Conventions

- Packages: lowercase single words (`web`, `ui`, `api`)
- Components: PascalCase (`Hero`, `Navbar`, `Echo`)
- Views: PascalCase matching route names (`Home`, `Blog`)
- CSS files: lowercase matching component names (`hero.css`, `navbar.css`)
- Asset constants: SCREAMING_SNAKE_CASE (`HERO_CSS`, `HEADER_SVG`)

---
*Mapped: 2026-05-20*
