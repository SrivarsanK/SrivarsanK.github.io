#!/bin/bash
# vercel-build.sh
# Exit immediately if a command exits with a non-zero status
set -e

echo "=== Installing Rust WebAssembly target ==="
rustup target add wasm32-unknown-unknown

# Ensure bin directory exists and is in the PATH
mkdir -p bin
export PATH="$PWD/bin:$PATH"

echo "=== Checking if Dioxus CLI is available ==="
if command -v dx >/dev/null 2>&1 && dx --version >/dev/null 2>&1; then
    echo "=== Dioxus CLI is already installed and working: $(dx --version) ==="
else
    echo "=== Dioxus CLI not found or not working. Attempting to download prebuilt binary... ==="
    TARGET="x86_64-unknown-linux-gnu"
    URL="https://github.com/DioxusLabs/dioxus/releases/download/v0.7.9/dx-${TARGET}.zip"
    echo "Downloading Dioxus CLI from: $URL"
    
    if curl --fail --location --output dx.zip "$URL"; then
        echo "Extracting Dioxus CLI..."
        # Extract to bin folder. -o means overwrite files without prompting.
        unzip -o dx.zip -d bin/
        rm dx.zip
        chmod +x bin/dx
        
        # Verify if the downloaded binary runs (checks GLIBC compatibility)
        if dx --version >/dev/null 2>&1; then
            echo "=== Prebuilt Dioxus CLI is working: $(dx --version) ==="
        else
            echo "=== Prebuilt Dioxus CLI failed compatibility check (e.g. GLIBC mismatch). ==="
            echo "=== Falling back to compiling Dioxus CLI from source... ==="
            rm -f bin/dx
            # Install from source locally into ./bin so it gets cached by Vercel
            cargo install dioxus-cli --version 0.7.9 --locked --debug --root .
        fi
    else
        echo "=== Download failed. Falling back to compiling from source... ==="
        cargo install dioxus-cli --version 0.7.9 --locked --debug --root .
    fi
fi

echo "=== Building Dioxus Web App ==="
dx build --release --platform web -p web

echo "=== Build finished successfully! ==="
