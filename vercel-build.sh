#!/bin/bash
# vercel-build.sh
# Exit immediately if a command exits with a non-zero status
set -e

echo "=== Installing Rust WebAssembly target ==="
rustup target add wasm32-unknown-unknown

echo "=== Installing cargo-binstall ==="
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

echo "=== Installing Dioxus CLI ==="
# Force x86_64-unknown-linux-gnu target to use prebuilt binaries
# instead of building from source (which is slow and fails in musl env)
cargo binstall -y --target x86_64-unknown-linux-gnu dioxus-cli

echo "=== Building Dioxus Web App ==="
dx build --release --platform web -p web

echo "=== Build finished successfully! ==="
