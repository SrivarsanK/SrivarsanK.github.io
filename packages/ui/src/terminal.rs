use dioxus::prelude::*;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use std::time::Duration;
use gloo_timers::future::sleep;

#[derive(Clone)]
enum LineType {
    Input,
    Output,
    Error,
    Info,
    Success,
    Component(Element),
}

#[derive(Clone)]
struct TerminalLine {
    id: usize,
    line_type: LineType,
    content: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct TerminalProps {
    pub external_command: Signal<Option<String>>,
    pub current_theme: Signal<String>,
    pub is_minimized: Signal<bool>,
}

#[derive(Deserialize)]
struct WaifuResponse {
    url: String,
}

#[derive(Deserialize)]
struct JokeResponse {
    #[serde(rename = "type")]
    joke_type: String,
    joke: Option<String>,
    setup: Option<String>,
    delivery: Option<String>,
}

const WELCOME_MESSAGE: &str = r#"Hello, World! I'm Ovi ren
I'm a writer, i write scripts.

Type 'help' to see available commands."#;

fn focus_input() {
    if let Some(win) = web_sys::window() {
        if let Some(doc) = win.document() {
            if let Some(el) = doc.get_element_by_id("terminal-input") {
                if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                    let _ = input.focus();
                }
            }
        }
    }
}

fn scroll_to_bottom() {
    if let Some(win) = web_sys::window() {
        if let Some(doc) = win.document() {
            if let Some(el) = doc.get_element_by_id("terminal-body") {
                el.set_scroll_top(el.scroll_height());
            }
        }
    }
}

#[component]
pub fn Terminal(props: TerminalProps) -> Element {
    let mut external_cmd = props.external_command;
    let current_theme = props.current_theme;
    let mut is_minimized = props.is_minimized;

    let mut lines = use_signal(|| {
        vec![TerminalLine {
            id: 0,
            line_type: LineType::Output,
            content: WELCOME_MESSAGE.to_string(),
        }]
    });
    let mut next_id = use_signal(|| 1usize);

    let mut current_input = use_signal(|| String::new());
    let mut command_history = use_signal(|| Vec::<String>::new());
    let mut history_index = use_signal(|| -1i32);

    let mut is_maximized = use_signal(|| false);

    // Animation state: "open", "minimize", "restore", "idle"
    let mut anim_state = use_signal(|| "open".to_string());

    let mut pos = use_signal(|| (100.0_f64, 60.0_f64));
    let mut size = use_signal(|| (760.0_f64, 460.0_f64));

    // Center terminal on first mount
    use_effect(move || {
        if let Some(win) = web_sys::window() {
            let w = win
                .inner_width()
                .unwrap_or(wasm_bindgen::JsValue::from(1280))
                .as_f64()
                .unwrap_or(1280.0);
            let h = win
                .inner_height()
                .unwrap_or(wasm_bindgen::JsValue::from(720))
                .as_f64()
                .unwrap_or(720.0);
            let term_w = 760.0_f64.min(w - 120.0);
            let term_h = 460.0_f64.min(h - 100.0);
            let x = ((w - term_w) / 2.0).max(20.0);
            let y = ((h - 40.0 - term_h) / 2.0).max(20.0);
            pos.set((x, y));
            size.set((term_w, term_h));
        }
    });

    let mut dragging_header = use_signal(|| None::<(f64, f64)>);
    let mut resizing = use_signal(|| None::<(f64, f64, f64, f64)>);

    // Execute command closure
    let mut execute_command = move |cmd_str: String| {
        let trimmed = cmd_str.trim().to_string();
        let trimmed_lower = trimmed.to_lowercase();

        // Echo the input line
        let id = *next_id.read();
        *next_id.write() += 1;
        lines.write().push(TerminalLine {
            id,
            line_type: LineType::Input,
            content: cmd_str.clone(),
        });

        if trimmed.is_empty() {
            scroll_to_bottom();
            return;
        }

        command_history.write().push(trimmed.clone());
        history_index.set(-1);

        if trimmed_lower == "cls" || trimmed_lower == "clear" {
            lines.write().clear();
            scroll_to_bottom();
            return;
        }

        let out_id = *next_id.read();
        *next_id.write() += 1;

        match trimmed_lower.as_str() {
            "help" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "Available commands:\n  help      - Show this help message\n  about     - Learn about me\n  skills    - View my technical skills\n  projects  - Browse my projects\n  contact   - Get my contact information\n  whoami    - Display current user\n  date      - Show current date and time\n  clear/cls - Clear the terminal\n  waifu     - Show a random waifu image\n  joke      - Tell a random joke".to_string(),
                });
            }
            "about" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Component(rsx! {
                        div {
                            class: "my-2 space-y-3",
                            div {
                                class: "border border-border rounded-lg overflow-hidden w-56 bg-secondary/50 shadow-lg",
                                img {
                                    src: "https://iamovi.github.io/assets/sword.jpg",
                                    alt: "Profile",
                                    class: "w-full h-auto object-cover"
                                }
                            }
                            div {
                                class: "font-mono leading-relaxed bg-secondary/30 p-4 border border-border rounded-lg max-w-lg",
                                div { class: "text-terminal-cyan font-bold mb-2", "┌ ABOUT ME ┐" }
                                p { class: "mb-2 text-foreground/90", "Hi! I'm a hobby programmer. I love building things and exploring new technologies in my free time." }
                                p { class: "text-foreground/90", "When I was in high school I started programming as my hobby, that's how it all started." }
                            }
                        }
                    }),
                    content: "".to_string(),
                });
            }
            "skills" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "┌─────────────────────────────────────────┐\n│  TECHNICAL SKILLS                       │\n├─────────────────────────────────────────┤\n│  • Rust, TypeScript, Python             │\n│  • Dioxus, React, Next.js               │\n│  • TailwindCSS, Git, Docker             │\n└─────────────────────────────────────────┘".to_string(),
                });
            }
            "projects" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "┌─────────────────────────────────────────┐\n│  FEATURED PROJECTS                      │\n├─────────────────────────────────────────┤\n│  01. syswaifu                           │\n│  02. button-will-react                  │\n│  Visit https://iamovi.github.io         │\n└─────────────────────────────────────────┘".to_string(),
                });
            }
            "contact" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "┌─────────────────────────────────────────┐\n│  CONTACT                                │\n├─────────────────────────────────────────┤\n│  📧 Email      fornet.ovi@email.com     │\n│  🐙 GitHub     github.com/iamovi        │\n│  🌐 Website    iamovi.github.io         │\n└─────────────────────────────────────────┘".to_string(),
                });
            }
            "whoami" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "ren@portfolio".to_string(),
                });
            }
            "date" => {
                let now = chrono::Local::now();
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: now.format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
            "waifu" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Info,
                    content: "Fetching waifu...".to_string(),
                });

                spawn(async move {
                    if let Ok(resp) = reqwest::get("https://api.waifu.pics/sfw/waifu").await {
                        if let Ok(data) = resp.json::<WaifuResponse>().await {
                            let mut l = lines.write();
                            if let Some(pos) = l.iter().position(|x| x.id == out_id) {
                                l[pos] = TerminalLine {
                                    id: out_id,
                                    line_type: LineType::Component(rsx! {
                                        img {
                                            src: "{data.url}",
                                            class: "my-2 rounded-lg shadow-lg border border-border max-w-xs"
                                        }
                                    }),
                                    content: "".to_string(),
                                };
                            }
                            drop(l);
                            scroll_to_bottom();
                            return;
                        }
                    }
                    let mut l = lines.write();
                    if let Some(pos) = l.iter().position(|x| x.id == out_id) {
                        l[pos] = TerminalLine {
                            id: out_id,
                            line_type: LineType::Error,
                            content: "Failed to fetch waifu.".to_string(),
                        };
                    }
                    drop(l);
                    scroll_to_bottom();
                });
            }
            "joke" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Info,
                    content: "Fetching a joke...".to_string(),
                });

                spawn(async move {
                    if let Ok(resp) = reqwest::get("https://v2.jokeapi.dev/joke/Any").await {
                        if let Ok(data) = resp.json::<JokeResponse>().await {
                            let mut l = lines.write();
                            if let Some(pos) = l.iter().position(|x| x.id == out_id) {
                                let content_elem = if data.joke_type == "single" {
                                    rsx! { p { class: "text-foreground/90", "{data.joke.clone().unwrap_or_default()}" } }
                                } else {
                                    rsx! {
                                        div { class: "space-y-2",
                                            p { class: "text-foreground/90 italic", "\"{data.setup.clone().unwrap_or_default()}\"" }
                                            p { class: "text-terminal-cyan font-bold", "{data.delivery.clone().unwrap_or_default()}" }
                                        }
                                    }
                                };
                                l[pos] = TerminalLine {
                                    id: out_id,
                                    line_type: LineType::Component(rsx! {
                                        div { class: "my-2 p-3 border border-border rounded-lg bg-secondary/30 max-w-lg font-mono leading-relaxed",
                                            div { class: "text-terminal-yellow font-bold mb-2", "┌ JOKE ┐" }
                                            {content_elem}
                                        }
                                    }),
                                    content: "".to_string(),
                                };
                            }
                            drop(l);
                            scroll_to_bottom();
                            return;
                        }
                    }
                    let mut l = lines.write();
                    if let Some(pos) = l.iter().position(|x| x.id == out_id) {
                        l[pos] = TerminalLine {
                            id: out_id,
                            line_type: LineType::Error,
                            content: "Failed to fetch joke.".to_string(),
                        };
                    }
                    drop(l);
                    scroll_to_bottom();
                });
            }
            _ => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Error,
                    content: format!("'{}': command not found. Type 'help' for available commands.", trimmed),
                });
            }
        }

        scroll_to_bottom();
    };

    // Watch external_command signal
    use_effect(move || {
        let cmd = external_cmd.read().clone();
        if let Some(cmd_str) = cmd {
            if !cmd_str.is_empty() {
                execute_command(cmd_str);
                external_cmd.set(None);
                if is_minimized() {
                    is_minimized.set(false);
                }
            }
        }
    });

    // Auto-scroll when lines change
    use_effect(move || {
        let _ = lines.read().len();
        scroll_to_bottom();
    });

    // Theme matching
    let theme_val = current_theme.read().clone();
    let (bg_color, text_color, prompt_color, prompt_text, border_color, header_bg) =
        match theme_val.as_str() {
            "powershell" => (
                "#012456", "#ffffff", "#ffffff",
                "PS C:\\Users\\ren> ", "#1a3a6e", "#0a1e4a",
            ),
            "ubuntu" => (
                "#300a24", "#ffffff", "#ffffff",
                "ren@ubuntu:~$ ", "#5c1345", "#470e35",
            ),
            "matrix" => (
                "#000500", "#00FF41", "#00FF41",
                "neo@matrix:~$ ", "#003300", "#001a00",
            ),
            "cmd" => (
                "#000000", "#ffffff", "#ffffff",
                "C:\\Users\\ren> ", "#333333", "#111111",
            ),
            "dracula" => (
                "#282a36", "#f8f8f2", "#bd93f9",
                "λ ", "#44475a", "#1e2029",
            ),
            _ => (
                "#012456", "#ffffff", "#ffffff",
                "PS C:\\Users\\ren> ", "#1a3a6e", "#0a1e4a",
            ),
        };

    // All known commands for Tab autocomplete
    const ALL_COMMANDS: &[&str] = &[
        "help", "about", "skills", "projects", "contact",
        "whoami", "date", "waifu", "joke", "clear", "cls",
    ];

    let handle_key_down = move |e: Event<KeyboardData>| {
        match e.key() {
            Key::Enter => {
                let input = current_input.read().clone();
                current_input.set(String::new());
                execute_command(input);
                // Re-focus input after command
                focus_input();
            }
            Key::Tab => {
                e.prevent_default();
                let partial = current_input.read().to_lowercase();
                if !partial.is_empty() {
                    let matches: Vec<&str> = ALL_COMMANDS
                        .iter()
                        .filter(|cmd| cmd.starts_with(partial.as_str()))
                        .copied()
                        .collect();
                    if matches.len() == 1 {
                        current_input.set(matches[0].to_string());
                    } else if matches.len() > 1 {
                        let hint = matches.join("  ");
                        let id = *next_id.read();
                        *next_id.write() += 1;
                        lines.write().push(TerminalLine {
                            id,
                            line_type: LineType::Info,
                            content: hint,
                        });
                        scroll_to_bottom();
                    }
                }
            }
            Key::ArrowUp => {
                let h = command_history.read();
                if !h.is_empty() {
                    let mut idx = history_index();
                    if idx < (h.len() as i32) - 1 {
                        idx += 1;
                    }
                    history_index.set(idx);
                    let cmd_idx = h.len() - 1 - (idx as usize);
                    current_input.set(h[cmd_idx].clone());
                }
            }
            Key::ArrowDown => {
                let h = command_history.read();
                let mut idx = history_index();
                if idx > 0 {
                    idx -= 1;
                    history_index.set(idx);
                    let cmd_idx = h.len() - 1 - (idx as usize);
                    current_input.set(h[cmd_idx].clone());
                } else {
                    history_index.set(-1);
                    current_input.set(String::new());
                }
            }
            _ => {}
        }
    };

    let handle_pointer_move = move |e: Event<PointerData>| {
        if let Some(offset) = dragging_header() {
            pos.set((
                e.client_coordinates().x - offset.0,
                e.client_coordinates().y - offset.1,
            ));
        } else if let Some(start) = resizing() {
            let width = f64::max(400.0, start.2 + (e.client_coordinates().x - start.0));
            let height = f64::max(280.0, start.3 + (e.client_coordinates().y - start.1));
            size.set((width, height));
        }
    };

    let handle_pointer_up = move |_| {
        dragging_header.set(None);
        resizing.set(None);
    };

    // Minimized pill at bottom
    if is_minimized() {
        return rsx! {
            div {
                class: "pill-enter",
                style: "position: fixed; bottom: 40px; left: 50%; transform: translateX(-50%); background-color: {header_bg}; border: 1px solid {border_color}; border-radius: 0.5rem 0.5rem 0 0; padding: 0.35rem 1.25rem; cursor: pointer; box-shadow: 0 -4px 20px rgba(0,0,0,0.4); display: flex; align-items: center; gap: 0.5rem; z-index: 60; pointer-events: auto;",
                onclick: move |_| {
                    anim_state.set("restore".to_string());
                    is_minimized.set(false);
                },
                div {
                    class: "taskbar-indicator-active",
                    style: "width: 8px; height: 8px; border-radius: 9999px; background-color: {prompt_color};"
                }
                span {
                    style: "font-size: 0.75rem; font-family: monospace; color: {text_color};",
                    "Terminal — Guest@Portfolio"
                }
            }
        };
    }

    let (_transform, left_style, top_style) = if is_maximized() {
        ("none".to_string(), "0".to_string(), "0".to_string())
    } else {
        (
            "none".to_string(),
            format!("{}px", pos().0),
            format!("{}px", pos().1),
        )
    };
    let width_style = if is_maximized() {
        "100%".to_string()
    } else {
        format!("{}px", size().0)
    };
    let height_style = if is_maximized() {
        "100%".to_string()
    } else {
        format!("{}px", size().1)
    };

    let user_select_style = if dragging_header().is_some() || resizing().is_some() {
        "user-select: none; -webkit-user-select: none;"
    } else {
        ""
    };

    // Determine animation class based on state
    let anim_class = match anim_state().as_str() {
        "open" => "terminal-open",
        "restore" => "terminal-restore",
        "minimize" => "terminal-minimize",
        _ => "",
    };

    // Maximize transition: smooth border-radius + size
    let border_radius = if is_maximized() { "0" } else { "0.5rem" };

    let transition_style = if dragging_header().is_some() || resizing().is_some() {
        "transition: none;"
    } else {
        "transition: width 300ms cubic-bezier(0.4, 0, 0.2, 1), height 300ms cubic-bezier(0.4, 0, 0.2, 1), top 300ms cubic-bezier(0.4, 0, 0.2, 1), left 300ms cubic-bezier(0.4, 0, 0.2, 1), border-radius 300ms ease;"
    };

    rsx! {
        div {
            class: "{anim_class}",
            style: "
                position: absolute;
                top: {top_style};
                left: {left_style};
                width: {width_style};
                height: {height_style};
                z-index: 40;
                border: 1px solid {border_color};
                border-radius: {border_radius};
                overflow: hidden;
                box-shadow: 0 25px 60px rgba(0,0,0,0.7), 0 0 0 1px {border_color};
                display: flex;
                flex-direction: column;
                pointer-events: auto;
                {transition_style}
                {user_select_style}
            ",
            onanimationend: move |_| {
                anim_state.set("idle".to_string());
            },
            onpointermove: handle_pointer_move,
            onpointerup: handle_pointer_up,
            onpointerleave: handle_pointer_up,

            // ─── Title Bar ────────────────────────────────────────────────
            div {
                style: "
                    background-color: {header_bg};
                    border-bottom: 1px solid {border_color};
                    padding: 0 0.75rem;
                    height: 32px;
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    cursor: move;
                    user-select: none;
                    flex-shrink: 0;
                ",
                onpointerdown: move |e| {
                    if !is_maximized() {
                        dragging_header.set(Some((
                            e.client_coordinates().x - pos().0,
                            e.client_coordinates().y - pos().1,
                        )));
                    }
                },

                // Traffic lights
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    onclick: move |e| e.stop_propagation(),

                    // Red = close (acts as minimize here like macOS)
                    div {
                        class: "traffic-light",
                        style: "width: 12px; height: 12px; border-radius: 50%; background: #ff5f57; cursor: pointer; flex-shrink: 0;",
                        title: "Minimize",
                        onclick: move |e| {
                            e.stop_propagation();
                            anim_state.set("minimize".to_string());
                            let mut is_min_clone = is_minimized.clone();
                            spawn(async move {
                                sleep(Duration::from_millis(300)).await;
                                is_min_clone.set(true);
                            });
                        }
                    }
                    // Yellow = minimize (acts as restore/maximize)
                    div {
                        class: "traffic-light",
                        style: "width: 12px; height: 12px; border-radius: 50%; background: #febc2e; cursor: pointer; flex-shrink: 0;",
                        title: "Maximize / Restore",
                        onclick: move |e| {
                            e.stop_propagation();
                            is_maximized.set(!is_maximized());
                        }
                    }
                    // Green = fullscreen
                    div {
                        class: "traffic-light",
                        style: "width: 12px; height: 12px; border-radius: 50%; background: #28c840; cursor: pointer; flex-shrink: 0;",
                        title: "Full Screen",
                        onclick: move |e| {
                            e.stop_propagation();
                            is_maximized.set(true);
                        }
                    }
                }

                // Centered title
                span {
                    style: "font-size: 0.7rem; font-family: monospace; color: {text_color}; opacity: 0.75; position: absolute; left: 50%; transform: translateX(-50%);",
                    "Terminal — Guest@Portfolio:~"
                }

                // Right spacer to balance traffic lights
                div { style: "width: 54px;" }
            }

            // ─── Scrollable Output Body ────────────────────────────────────
            div {
                id: "terminal-body",
                class: "terminal-theme-transition",
                style: "
                    background-color: {bg_color};
                    color: {text_color};
                    flex: 1;
                    overflow-y: auto;
                    padding: 0.75rem 1rem 0.5rem 1rem;
                    font-family: 'JetBrains Mono', 'Fira Code', monospace;
                    font-size: 0.8125rem;
                    line-height: 1.6;
                    min-height: 0;
                    scrollbar-width: thin;
                    scrollbar-color: {border_color} transparent;
                ",
                onclick: move |_| focus_input(),

                for line in lines() {
                    div {
                        key: "{line.id}",
                        style: match &line.line_type {
                            LineType::Error   => "color: #f87171; white-space: pre-wrap; margin-bottom: 2px;",
                            LineType::Success  => "color: #4ade80; white-space: pre-wrap; margin-bottom: 2px;",
                            LineType::Info     => "color: #67e8f9; white-space: pre-wrap; margin-bottom: 2px;",
                            _                  => "white-space: pre-wrap; margin-bottom: 2px;",
                        },
                        match &line.line_type {
                            LineType::Input => {
                                rsx! {
                                    div {
                                        style: "display: flex; flex-wrap: wrap;",
                                        span {
                                            style: "color: {prompt_color}; margin-right: 0.4rem; flex-shrink: 0; white-space: nowrap;",
                                            "{prompt_text}"
                                        }
                                        span { "{line.content}" }
                                    }
                                }
                            }
                            LineType::Component(elem) => {
                                rsx! { div { {elem.clone()} } }
                            }
                            _ => {
                                rsx! { div { "{line.content}" } }
                            }
                        }
                    }
                }
            }

            // ─── Input Row (OUTSIDE scrollable area) ───────────────────────
            div {
                style: "
                    background-color: {bg_color};
                    border-top: 1px solid {border_color}33;
                    padding: 0.4rem 1rem;
                    display: flex;
                    align-items: center;
                    flex-shrink: 0;
                ",
                onclick: move |_| focus_input(),

                span {
                    style: "color: {prompt_color}; margin-right: 0.4rem; flex-shrink: 0; white-space: nowrap; font-family: monospace; font-size: 0.8125rem;",
                    "{prompt_text}"
                }
                input {
                    id: "terminal-input",
                    r#type: "text",
                    value: "{current_input()}",
                    oninput: move |e| current_input.set(e.value()),
                    onkeydown: handle_key_down,
                    style: "
                        flex: 1;
                        min-width: 0;
                        background: transparent;
                        outline: none;
                        border: none;
                        color: {text_color};
                        font-family: 'JetBrains Mono', 'Fira Code', monospace;
                        font-size: 0.8125rem;
                        caret-color: {prompt_color};
                        padding: 0;
                    ",
                    autofocus: true,
                    spellcheck: false,
                    autocomplete: "off",
                    autocorrect: "off",
                    autocapitalize: "off",
                }
            }

            // ─── Status Footer ─────────────────────────────────────────────
            div {
                style: "
                    background-color: {header_bg};
                    border-top: 1px solid {border_color};
                    padding: 0.2rem 0.75rem;
                    font-size: 0.65rem;
                    font-family: monospace;
                    color: {text_color};
                    opacity: 0.75;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    flex-shrink: 0;
                    user-select: none;
                ",

                // Left hint
                span { "Type 'help' for commands" }

                // Right hints + resize handle
                div {
                    style: "display: flex; align-items: center; gap: 0.75rem; position: relative;",
                    span { "↑↓ History" }
                    span { "•" }
                    span { "Tab Autocomplete" }

                    // Resize handle (bottom-right corner)
                    if !is_maximized() {
                        div {
                            style: "
                                cursor: nwse-resize;
                                position: absolute;
                                right: -0.5rem;
                                bottom: -0.2rem;
                                padding: 0.2rem;
                                z-index: 50;
                            ",
                            onpointerdown: move |e| {
                                e.stop_propagation();
                                resizing.set(Some((
                                    e.client_coordinates().x,
                                    e.client_coordinates().y,
                                    size().0,
                                    size().1,
                                )));
                            },
                            div {
                                style: "width: 10px; height: 10px; border-right: 2px solid {text_color}; border-bottom: 2px solid {text_color}; opacity: 0.5;"
                            }
                        }
                    }
                }
            }
        }
    }
}
