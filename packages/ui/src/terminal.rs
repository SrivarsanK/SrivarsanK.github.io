use dioxus::prelude::*;
use serde::Deserialize;
use wasm_bindgen::JsCast;

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

#[component]
pub fn Terminal(props: TerminalProps) -> Element {
    let mut external_cmd = props.external_command;
    let current_theme = props.current_theme;
    let mut lines = use_signal(|| vec![
        TerminalLine {
            id: 0,
            line_type: LineType::Output,
            content: WELCOME_MESSAGE.to_string(),
        }
    ]);
    let mut next_id = use_signal(|| 1);
    
    let mut current_input = use_signal(|| String::new());
    let mut command_history = use_signal(|| Vec::<String>::new());
    let mut history_index = use_signal(|| -1i32);
    
    let mut is_minimized = use_signal(|| false);
    let mut is_maximized = use_signal(|| false);
    
    let mut pos = use_signal(|| (100.0_f64, 20.0_f64));
    let mut size = use_signal(|| (760.0_f64, 460.0_f64));

    // Center terminal on first mount based on actual window size
    use_effect(move || {
        if let Some(win) = web_sys::window() {
            let w = win.inner_width().unwrap_or(wasm_bindgen::JsValue::from(1280))
                .as_f64().unwrap_or(1280.0);
            let h = win.inner_height().unwrap_or(wasm_bindgen::JsValue::from(720))
                .as_f64().unwrap_or(720.0);
            let term_w = 760.0_f64.min(w - 120.0);
            let term_h = 460.0_f64.min(h - 100.0);
            let x = ((w - term_w) / 2.0).max(100.0);
            let y = ((h - 40.0 - term_h) / 2.0).max(20.0);
            pos.set((x, y));
            size.set((term_w, term_h));
        }
    });
    
    let mut dragging_header = use_signal(|| None::<(f64, f64)>);
    let mut resizing = use_signal(|| None::<(f64, f64, f64, f64)>); // start_x, start_y, start_w, start_h

    // Execute command function
    let mut execute_command = move |cmd_str: String| {
        let trimmed = cmd_str.trim().to_lowercase();
        
        let id = next_id();
        next_id += 1;
        
        lines.write().push(TerminalLine {
            id,
            line_type: LineType::Input,
            content: cmd_str.clone(),
        });

        if trimmed.is_empty() {
            return;
        }

        command_history.write().push(cmd_str.clone());
        history_index.set(-1);

        if trimmed == "cls" || trimmed == "clear" {
            lines.write().clear();
            return;
        }

        let out_id = next_id();
        next_id += 1;

        match trimmed.as_str() {
            "help" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "Available commands:\n  help      - Show this help message\n  about     - Learn about me\n  skills    - View my technical skills\n  projects  - Browse my projects\n  contact   - Get my contact information\n  whoami    - Display current user\n  date      - Show current date and time\n  clear     - Clear the terminal\n  waifu     - Show a random waifu image\n  joke      - Tell a random joke".to_string(),
                });
            }
            "about" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Component(rsx! {
                        div {
                            class: "my-4 space-y-4",
                            div {
                                class: "border border-border rounded-lg overflow-hidden w-64 bg-secondary/50 shadow-lg",
                                img {
                                    src: "https://iamovi.github.io/assets/sword.jpg",
                                    alt: "Profile",
                                    class: "w-full h-auto object-cover"
                                }
                            }
                            div {
                                class: "font-mono leading-relaxed bg-secondary/30 p-4 border border-border rounded-lg",
                                div { class: "text-terminal-cyan font-bold mb-2", "┌ ABOUT ME ┐" }
                                p { class: "mb-4 text-foreground/90", "Hi! I'm a hobby programmer. I love building things and exploring new technologies in my free time." }
                                p { class: "text-foreground/90", "When i was in high school i started programming as my hobby, that's how it all started." }
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
                // Initial loading message
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
                                            class: "my-4 rounded-lg shadow-lg border border-border max-w-sm"
                                        }
                                    }),
                                    content: "".to_string(),
                                };
                            }
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
                });
            }
            "joke" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Info,
                    content: "Fetching a joke for you...".to_string(),
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
                                        div { class: "space-y-4",
                                            p { class: "text-foreground/90 italic", "\"{data.setup.clone().unwrap_or_default()}\"" }
                                            p { class: "text-terminal-cyan font-bold", "{data.delivery.clone().unwrap_or_default()}" }
                                        }
                                    }
                                };
                                l[pos] = TerminalLine {
                                    id: out_id,
                                    line_type: LineType::Component(rsx! {
                                        div { class: "my-4 p-4 border border-border rounded-lg bg-secondary/30 max-w-lg font-mono leading-relaxed",
                                            div { class: "text-terminal-yellow font-bold mb-2", "┌ JOKE ┐" }
                                            {content_elem}
                                        }
                                    }),
                                    content: "".to_string(),
                                };
                            }
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
                });
            }
            _ => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Error,
                    content: format!("Command not found: {}. Type 'help' for available commands.", trimmed),
                });
            }
        }
    };

    // Watch external_command
    use_effect(move || {
        let cmd = external_cmd.read().clone();
        if let Some(cmd_str) = cmd {
            if !cmd_str.is_empty() {
                execute_command(cmd_str);
                // Clear command once executed
                if !is_minimized() {
                    external_cmd.set(None);
                } else {
                    is_minimized.set(false);
                    external_cmd.set(None);
                }
            }
        }
    });

    // Auto-scroll logic
    use_effect(move || {
        let _ = lines.read().len();
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(el) = doc.get_element_by_id("terminal-body") {
                    el.set_scroll_top(el.scroll_height());
                }
            }
        }
    });

    // Theme matching logic
    let theme_val = current_theme.read().clone();
    let (bg_color, text_color, prompt_color, prompt_text, border_color) = match theme_val.as_str() {
        "powershell" => ("#012456", "#ffffff", "#00ff00", "PS Guest@Portfolio> ", "#1a3a6e"),
        "ubuntu"     => ("#300a24", "#ffffff", "#8ae234", "guest@portfolio:~$ ", "#6a1a4a"),
        "matrix"     => ("#000000", "#00ff00", "#00ff00", "guest@matrix:~$ ",    "#003300"),
        "retro"      => ("#2d2d2d", "#ffb000", "#ffb000", "C:\\> ",               "#555500"),
        "dracula"    => ("#282a36", "#f8f8f2", "#50fa7b", "guest@dracula:~$ ",   "#44475a"),
        _            => ("#012456", "#ffffff", "#00ff00", "guest@portfolio:~$ ",  "#1a3a6e"),
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
                execute_command(input);
                current_input.set(String::new());
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
                        // Show matches as a hint line
                        let hint = matches.join("  ");
                        let id = next_id();
                        next_id += 1;
                        lines.write().push(TerminalLine {
                            id,
                            line_type: LineType::Info,
                            content: hint,
                        });
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
            let height = f64::max(300.0, start.3 + (e.client_coordinates().y - start.1));
            size.set((width, height));
        }
    };

    let handle_pointer_up = move |_| {
        dragging_header.set(None);
        resizing.set(None);
    };

    if is_minimized() {
        return rsx! {
            div {
                style: "position: fixed; bottom: 48px; right: 1rem; background-color: {bg_color}; border: 1px solid {border_color}; border-radius: 0.5rem 0.5rem 0 0; padding: 0.4rem 1rem; cursor: pointer; box-shadow: 0 -4px 20px rgba(0,0,0,0.4); display: flex; align-items: center; gap: 0.5rem; z-index: 60;",
                onclick: move |_| is_minimized.set(false),
                div { style: "width: 0.5rem; height: 0.5rem; border-radius: 9999px; background-color: {prompt_color}; animation: pulse 2s cubic-bezier(0.4,0,0.6,1) infinite;" }
                span { style: "font-size: 0.75rem; font-family: monospace; color: {text_color};", "Terminal ─ Guest@Portfolio" }
            }
        };
    }

    let transform = if is_maximized() { "none".to_string() } else { format!("translate3d({}px, {}px, 0)", pos().0, pos().1) };
    let width_style = if is_maximized() { "100%".to_string() } else { format!("{}px", size().0) };
    let height_style = if is_maximized() { "100%".to_string() } else { format!("{}px", size().1) };
    let pos_style = if is_maximized() { "fixed" } else { "absolute" };
    let z_index = if is_maximized() { 50 } else { 40 };
    let drag_class = if dragging_header().is_some() || resizing().is_some() { "select-none" } else { "transition-all duration-200" };
    let user_select_style = if dragging_header().is_some() || resizing().is_some() { "user-select: none;" } else { "" };

    rsx! {
        div {
            style: "transform: {transform}; width: {width_style}; height: {height_style}; position: {pos_style}; top: 0; left: 0; z-index: {z_index}; border: 1px solid {border_color}; border-radius: 0.5rem; overflow: hidden; box-shadow: 0 25px 60px rgba(0,0,0,0.6), 0 0 0 1px {border_color}; display: flex; flex-direction: column; pointer-events: auto; {user_select_style} {drag_class}",
            onpointermove: handle_pointer_move,
            onpointerup: handle_pointer_up,
            onpointerleave: handle_pointer_up,
            
            // Header — themed titlebar
            div {
                style: "background-color: {border_color}; border-bottom: 1px solid {border_color}; padding: 0.4rem 0.75rem; display: flex; align-items: center; justify-content: space-between; cursor: move; user-select: none; flex-shrink: 0;",
                onpointerdown: move |e| {
                    if !is_maximized() {
                        dragging_header.set(Some((
                            e.client_coordinates().x - pos().0,
                            e.client_coordinates().y - pos().1
                        )));
                    }
                },
                div {
                    style: "display: flex; align-items: center; gap: 0.5rem;",
                    // Traffic lights
                    div { style: "width: 12px; height: 12px; border-radius: 50%; background: #ff5f57; cursor: pointer;",
                        onclick: move |e| { e.stop_propagation(); is_minimized.set(true); }
                    }
                    div { style: "width: 12px; height: 12px; border-radius: 50%; background: #febc2e; cursor: pointer;",
                        onclick: move |e| { e.stop_propagation(); is_maximized.set(!is_maximized()); }
                    }
                    div { style: "width: 12px; height: 12px; border-radius: 50%; background: #28c840;"
                    }
                }
                span { style: "font-size: 0.7rem; font-family: monospace; color: {text_color}; opacity: 0.8; flex: 1; text-align: center;", "Terminal — Guest@Portfolio:~" }
                div { style: "width: 3rem;" } // spacer to center title
            }

            // Body
            div {
                id: "terminal-body",
                style: "background-color: {bg_color}; color: {text_color};",
                class: "flex-1 overflow-y-auto p-4 font-mono text-sm leading-relaxed terminal-scrollbar min-h-0",
                onclick: move |_| {
                    if let Some(win) = web_sys::window() {
                        if let Some(doc) = win.document() {
                            if let Some(input) = doc.get_element_by_id("terminal-input") {
                                if let Ok(html_input) = input.dyn_into::<web_sys::HtmlInputElement>() {
                                    let _ = html_input.focus();
                                }
                            }
                        }
                    }
                },
                for line in lines() {
                    div { key: "{line.id}",
                        class: match &line.line_type {
                            LineType::Error => "text-terminal-red whitespace-pre-wrap",
                            LineType::Success => "text-terminal-green whitespace-pre-wrap",
                            LineType::Info => "text-terminal-cyan whitespace-pre-wrap",
                            _ => "text-foreground whitespace-pre-wrap"
                        },
                        match &line.line_type {
                            LineType::Input => {
                                rsx! { div { class: "flex", span { style: "color: {prompt_color};", class: "mr-2", "{prompt_text}" } span { "{line.content}" } } }
                            }
                            LineType::Component(elem) => {
                                rsx! { div { class: "animate-fade-in", {elem.clone()} } }
                            }
                            _ => {
                                rsx! { div { class: "animate-fade-in", "{line.content}" } }
                            }
                        }
                    }
                }

                // Input Line
                div {
                    style: "display: flex; align-items: center; margin-top: 0.5rem; flex-shrink: 0;",
                    span { style: "color: {prompt_color}; margin-right: 0.5rem; flex-shrink: 0; white-space: nowrap;", "{prompt_text}" }
                    input {
                        id: "terminal-input",
                        r#type: "text",
                        value: "{current_input()}",
                        oninput: move |e| current_input.set(e.value()),
                        onkeydown: handle_key_down,
                        style: "flex: 1; min-width: 0; background: transparent; outline: none; border: none; color: {text_color}; font-family: 'JetBrains Mono', monospace; font-size: 0.875rem; caret-color: {prompt_color};",
                        autofocus: true,
                        spellcheck: false,
                        autocomplete: "off",
                        autocorrect: "off",
                        autocapitalize: "off",
                    }
                }
            }

            // Footer
            div {
                style: "background-color: {border_color}; border-top: 1px solid {border_color}; padding: 0.25rem 1rem; font-size: 0.65rem; font-family: monospace; color: {text_color}; opacity: 0.7; display: flex; justify-content: space-between; align-items: center; flex-shrink: 0; position: relative;",
                span { "Type 'help' for commands" }
                div { style: "display: flex; align-items: center; gap: 1rem;",
                    span { "↑↓ History" }
                    if !is_maximized() {
                        div {
                            style: "cursor: nwse-resize; position: absolute; right: 0; bottom: 0; padding: 0.25rem; z-index: 50;",
                            onpointerdown: move |e| {
                                resizing.set(Some((e.client_coordinates().x, e.client_coordinates().y, size().0, size().1)));
                            },
                            div { style: "width: 0.75rem; height: 0.75rem; border-right: 2px solid {text_color}; border-bottom: 2px solid {text_color}; opacity: 0.5; margin: 0.25rem;" }
                        }
                    }
                }
            }
        }
    }
}
