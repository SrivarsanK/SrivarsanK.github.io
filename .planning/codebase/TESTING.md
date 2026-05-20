# Testing

## Current State

**No tests exist.** The codebase has zero test files or test modules.

## Test Infrastructure

- No test framework configured beyond default `cargo test`
- No integration tests
- No component tests
- No snapshot tests

## Recommendations

- Add `#[cfg(test)]` modules in library crates (`ui`, `api`)
- Consider Dioxus component testing utilities
- Server function tests in `api` crate

---
*Mapped: 2026-05-20*
