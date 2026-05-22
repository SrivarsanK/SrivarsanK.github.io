#!/bin/bash
# vercel-build.sh
set -e

echo "=== Installing Rust WebAssembly target ==="
rustup target add wasm32-unknown-unknown

# --- Dioxus CLI: cache-first strategy ---
# .dx-cache/ lives inside project dir → Vercel persists it between deploys.
# First deploy compiles from source (~7m). Every deploy after: ~0s.
DX_CACHE_DIR="$PWD/.dx-cache"
DX_CACHE_BIN="$DX_CACHE_DIR/dx"
mkdir -p "$DX_CACHE_DIR"

echo "=== Setting up Dioxus CLI ==="
if [ -f "$DX_CACHE_BIN" ] && "$DX_CACHE_BIN" --version >/dev/null 2>&1; then
    echo "=== Using cached Dioxus CLI: $($DX_CACHE_BIN --version) ==="
else
    echo "=== No cached dx found. Compiling from source (first deploy only)... ==="
    cargo install dioxus-cli --version 0.7.9 --locked --debug --root "$DX_CACHE_DIR"
    # cargo install puts binary in $root/bin/dx, move it up
    mv "$DX_CACHE_DIR/bin/dx" "$DX_CACHE_BIN"
    rmdir "$DX_CACHE_DIR/bin" 2>/dev/null || true
    rm -f "$DX_CACHE_DIR/.crates.toml" "$DX_CACHE_DIR/.crates2.json"
    echo "=== Compiled and cached: $($DX_CACHE_BIN --version) ==="
fi

# Put cached dx on PATH
export PATH="$DX_CACHE_DIR:$PATH"

echo "=== Building Dioxus Web App ==="
dx build --release --platform web -p web

echo "=== Build finished successfully! ==="
