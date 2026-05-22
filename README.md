<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" alt="Rust Logo" width="100" height="100" style="margin-right: 20px;">
  <img src="https://avatars.githubusercontent.com/u/79236386?s=200&v=4" alt="Dioxus Logo" width="100" height="100">

  <h1>🚀 Web OS Portfolio</h1>
  <p>A stunning, high-performance web-based desktop environment portfolio built entirely with <b>Rust</b> and <b>Dioxus 0.7</b>.</p>

  <p>
    <a href="#features">Features</a> •
    <a href="#tech-stack">Tech Stack</a> •
    <a href="#project-structure">Project Structure</a> •
    <a href="#getting-started">Getting Started</a>
  </p>
</div>

---

## ✨ Features

- **💻 Immersive OS Experience:** A complete, beautifully designed desktop environment directly in the browser.
- **🎨 Custom Animations:** GPU-accelerated, 60fps CSS animations for silky-smooth interactions (window scaling, icon dragging, menu transitions).
- **🔒 Interactive Boot & Login:** Features a fully animated boot sequence and a lock screen to mimic real OS behavior.
- **🖥️ Functional Terminal:** Built-in simulated terminal with draggable windows, instant resizing, maximize/minimize states, and command history.
- **📂 Draggable Icons:** Interactive desktop icons with perfectly synced drag-and-drop physics and highlight states.
- **🌗 Theme Ready:** Architected with dynamic color tokens mimicking macOS aesthetics (dark/light mode capable).
- **🚀 WebAssembly Powered:** Runs entirely client-side using WASM for near-native performance.

---

## 🛠️ Tech Stack

- **[Rust](https://www.rust-lang.org/)**: Core logic and application state.
- **[Dioxus (v0.7)](https://dioxuslabs.com/)**: Reactive UI framework used for building the component tree and managing state.
- **Tailwind-inspired CSS**: Custom CSS (`main.css`) utilizing utility classes and advanced CSS keyframe animations.
- **WebAssembly (WASM)**: Compilation target for executing Rust code at native speeds in the browser.

---

## 📁 Project Structure

This project is structured as a Cargo workspace to keep concerns separated:

```text
portfolio/
├─ packages/
│  ├─ web/         # 🌐 Web application entry point (WASM build)
│  │  ├─ assets/   # CSS, Fonts, Images
│  │  └─ src/      # main.rs (Bootstrapper)
│  ├─ ui/          # 🧩 Shared UI components (The OS interface)
│  │  └─ src/
│  │     ├─ boot_sequence.rs
│  │     ├─ desktop_icons.rs
│  │     ├─ login_screen.rs
│  │     ├─ taskbar.rs
│  │     └─ terminal.rs
│  └─ api/         # ⚙️ Shared backend logic (if applicable)
└─ Cargo.toml      # Workspace configuration
```

---

## 🚀 Getting Started

### Prerequisites

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install the `dx` CLI tool for Dioxus:
   ```bash
   cargo binstall dioxus-cli
   ```
   *(Or refer to the [Dioxus documentation](https://dioxuslabs.com/learn/0.7/getting_started) for alternative installation methods).*

### Running Locally

1. Clone the repository and navigate into it.
2. Change into the `web` package directory:
   ```bash
   cd packages/web
   ```
3. Start the development server with hot-reloading:
   ```bash
   dx serve
   ```
4. Open your browser to `http://localhost:8080`.

---

## 🌐 Deployment

This project is configured to build and deploy to **Vercel** with support for both the WebAssembly static frontend and Rust serverless functions:

- **Build Command**: `bash vercel-build.sh`
- **Output Directory**: `target/dx/web/release/web/public`
- **Required Environment Variables**:
  - `RESEND_API_KEY`: Your Resend API key (needed by the contact form API).

---

## 💡 Why Rust for a Web UI?

Using Rust and Dioxus provides incredible memory safety, strong typing, and minimal overhead. Unlike heavy JavaScript frameworks, this web OS compiles to a lean WASM binary, allowing us to manage complex state (like overlapping windows, dragging coordinates, and terminal buffers) with zero fear of runtime type errors.

---

<div align="center">
  <i>Built with ❤️ using Rust & Dioxus</i>
</div>
