#!/bin/bash
# vercel-build.sh
# Exit immediately if a command exits with a non-zero status
set -e

echo "=== Installing Rust WebAssembly target ==="
rustup target add wasm32-unknown-unknown

echo "=== Checking if Dioxus CLI is already installed ==="
if command -v dx >/dev/null 2>&1; then
    echo "=== Dioxus CLI is already installed: $(dx --version) ==="
else
    echo "=== Dioxus CLI not found. Installing from source... ==="
    # Compile from source in debug mode to avoid GLIBC version mismatches
    # while dramatically speeding up compile times on Vercel (disables optimizations)
    cargo install dioxus-cli --version 0.7.9 --locked --debug
fi

echo "=== Building Dioxus Web App ==="
dx build --release --platform web -p web

echo "=== Build finished successfully! ==="
